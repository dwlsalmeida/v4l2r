use std::io::Cursor;
use std::io::Read;

use anyhow::anyhow;
use anyhow::Result;
use bitreader::BitReader;

use crate::decoder::format::StreamSplitter;
use crate::decoder::format::vp8::IvfReader;

pub const SUPERFRAME_MARKER: u32 = 0x06;
pub const MAX_FRAMES_IN_SUPERFRAME: usize = 8;

pub const FRAME_MARKER: u32 = 0x02;
pub const SYNC_CODE: u32 = 0x498342;

#[derive(Default)]
/// The VP9 superframe header as per Annex B, B.2.1, B.2.2
pub struct SuperframeHeader {
    /// Indicates the number of frames within this superframe. NOTE - It is
    /// legal for a superframe to contain just a single frame and have NumFrames
    /// equal to 1.
    frames_in_superframe: u32,
    /// Specifies the size in bytes of frame number i (zero indexed) within this
    /// superframe.
    frame_sizes: Vec<usize>,
}


pub struct Vp9SuperframeParser<S> {
    ivf: IvfReader<S>,
    superframe_idx: usize,
    superframe_hdr: SuperframeHeader,
    offset: usize,
    data: Vec<u8>,
}

impl<S: Read> Vp9SuperframeParser<S> {
    pub fn new(stream: S) -> Result<Self> {
        Ok(Self {
            ivf: IvfReader::new(stream)?,
            superframe_idx: 0,
            superframe_hdr: Default::default(),
            offset: 0,
            data: Default::default(),
        })
    }

    fn parse_superframe_hdr(&mut self, data: &Vec<u8>) -> Result<SuperframeHeader> {
        let bitstream = data;

        // Skip to the end of the chunk.
        let mut reader = BitReader::new(&bitstream[bitstream.len() - 1..]);

        // Try reading a superframe marker.
        let marker = reader.read_u32(3)?;

        if marker != SUPERFRAME_MARKER {
            // Not a superframe
            return Ok(SuperframeHeader {
                frames_in_superframe: 1,
                frame_sizes: vec![bitstream.len()],
            });
        }

        let bytes_per_framesize = reader.read_u32(2)? + 1;
        let frames_in_superframe = reader.read_u32(3)? + 1;

        if frames_in_superframe > MAX_FRAMES_IN_SUPERFRAME as u32 {
            return Err(anyhow!(
                "Broken stream: too many frames in superframe, expected a maximum of {:?}, found {:?}",
                MAX_FRAMES_IN_SUPERFRAME,
                frames_in_superframe
            ));
        }

        let sz_index = 2 + frames_in_superframe * bytes_per_framesize;

        let index_offset = data.len() - sz_index as usize;
        let first_byte = data[index_offset];
        let last_byte = *data.last().unwrap();

        if first_byte != last_byte {
            // Also not a superframe, we must pass both tests as per the specification.
            return Ok(SuperframeHeader {
                frames_in_superframe: 1,
                frame_sizes: vec![bitstream.len()],
            });
        }

        let mut frame_sizes = vec![];
        let mut reader = BitReader::new(&bitstream[index_offset..]);

        // Skip the superframe header.
        let _ = reader.read_u32(8)?;

        for _ in 0..frames_in_superframe {
            let mut frame_size = 0;

            for j in 0..bytes_per_framesize {
                frame_size |= reader.read_u32(8)? << (j * 8);
            }

            frame_sizes.push(frame_size as usize);
        }

        Ok(SuperframeHeader {
            frames_in_superframe,
            frame_sizes,
        })
    }
}

impl<S: Read> Iterator for Vp9SuperframeParser<S> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {

        if self.superframe_idx == 0 {
            let data = self.ivf.next()?;
            self.superframe_hdr = self.parse_superframe_hdr(&data).ok()?;
            self.superframe_idx = self.superframe_hdr.frames_in_superframe as usize;
            self.offset = 0;
            self.data = data;
        }

        let idx = self.superframe_hdr.frame_sizes.len() - self.superframe_idx;
        let frame_sz = self.superframe_hdr.frame_sizes[idx];
        let data = &self.data[self.offset..self.offset + frame_sz];
        self.offset += frame_sz;

        self.superframe_idx -= 1;

        Some(Vec::from(data))
    }
}

impl<S: Read> StreamSplitter for Vp9SuperframeParser<S> {}