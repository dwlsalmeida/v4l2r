use std::io::{SeekFrom, Read};
use std::io::{Cursor, Seek};
use anyhow::Result;
use bytes::Buf;

use crate::decoder::format::StreamSplitter;

pub struct IvfReader<S> {
    stream: S,
}

impl<S: Read> IvfReader<S> {
    pub fn new(mut stream: S) -> Result<Self> {
        let mut hdr = vec![0u8; 32];
        stream.read_exact(& mut hdr)?;
        Ok(Self {
            stream,
        })
    }
}

impl<S: Read>  Iterator for IvfReader<S> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut len = [0u8; 4];
        self.stream.read_exact(&mut len[..]).ok()?;

        // Skip PTS.
        let mut pts = [0u8; 8];
        self.stream.read_exact(&mut pts[..]).ok()?;

        let mut buf = vec![0u8; u32::from_le_bytes(len) as usize];
        self.stream.read_exact(&mut buf).ok()?;

        Some(buf)
    }
}

impl<S: Read> StreamSplitter for IvfReader<S> {}