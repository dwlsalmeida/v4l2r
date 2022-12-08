use std::convert::TryFrom;
use std::os::unix::io::AsRawFd;
use std::mem::{self, size_of};
use thiserror::Error;

use crate::bindings;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum ExtControlKind {
    H264DecodeMode = bindings::V4L2_CID_STATELESS_H264_DECODE_MODE,
    H264StartCode = bindings::V4L2_CID_STATELESS_H264_START_CODE,
    H264Sps = bindings::V4L2_CID_STATELESS_H264_SPS,
    H264Pps = bindings::V4L2_CID_STATELESS_H264_PPS,
    H264ScalingMatrix = bindings::V4L2_CID_STATELESS_H264_SCALING_MATRIX,
    H264PredWeights = bindings::V4L2_CID_STATELESS_H264_PRED_WEIGHTS,
    H264SliceParams = bindings::V4L2_CID_STATELESS_H264_SLICE_PARAMS,
    H264DecodeParams = bindings::V4L2_CID_STATELESS_H264_DECODE_PARAMS,
    FwhtParams = bindings::V4L2_CID_STATELESS_FWHT_PARAMS,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum Colorspace {
    Default = bindings::v4l2_colorspace_V4L2_COLORSPACE_DEFAULT,
    Smpte170M = bindings::v4l2_colorspace_V4L2_COLORSPACE_SMPTE170M,
    Smpte240M = bindings::v4l2_colorspace_V4L2_COLORSPACE_SMPTE240M,
    Rec709 = bindings::v4l2_colorspace_V4L2_COLORSPACE_REC709,
    Bt878 = bindings::v4l2_colorspace_V4L2_COLORSPACE_BT878,
    SystemM470 = bindings::v4l2_colorspace_V4L2_COLORSPACE_470_SYSTEM_M,
    SystemBG470 = bindings::v4l2_colorspace_V4L2_COLORSPACE_470_SYSTEM_BG,
    Jpeg = bindings::v4l2_colorspace_V4L2_COLORSPACE_JPEG,
    Srgb = bindings::v4l2_colorspace_V4L2_COLORSPACE_SRGB,
    OpRgb = bindings::v4l2_colorspace_V4L2_COLORSPACE_OPRGB,
    Bt2020 = bindings::v4l2_colorspace_V4L2_COLORSPACE_BT2020,
    Raw = bindings::v4l2_colorspace_V4L2_COLORSPACE_RAW,
    DciP3 = bindings::v4l2_colorspace_V4L2_COLORSPACE_DCI_P3,
}

impl From<u32> for Colorspace {
    fn from(colorspace: u32) -> Self {
        match colorspace {
            bindings::v4l2_colorspace_V4L2_COLORSPACE_DEFAULT => Colorspace::Default,
            bindings::v4l2_colorspace_V4L2_COLORSPACE_SMPTE170M => Colorspace::Smpte170M,
            bindings::v4l2_colorspace_V4L2_COLORSPACE_SMPTE240M => Colorspace::Smpte240M,
            bindings::v4l2_colorspace_V4L2_COLORSPACE_REC709 => Colorspace::Rec709,
            bindings::v4l2_colorspace_V4L2_COLORSPACE_BT878 => Colorspace::Bt878,
            bindings::v4l2_colorspace_V4L2_COLORSPACE_470_SYSTEM_M => Colorspace::SystemM470,
            bindings::v4l2_colorspace_V4L2_COLORSPACE_470_SYSTEM_BG => Colorspace::SystemBG470,
            bindings::v4l2_colorspace_V4L2_COLORSPACE_JPEG => Colorspace::Jpeg,
            bindings::v4l2_colorspace_V4L2_COLORSPACE_SRGB => Colorspace::Srgb,
            bindings::v4l2_colorspace_V4L2_COLORSPACE_OPRGB => Colorspace::OpRgb,
            bindings::v4l2_colorspace_V4L2_COLORSPACE_BT2020 => Colorspace::Bt2020,
            bindings::v4l2_colorspace_V4L2_COLORSPACE_RAW => Colorspace::Raw,
            bindings::v4l2_colorspace_V4L2_COLORSPACE_DCI_P3 => Colorspace::DciP3,
            _ => Colorspace::Default,
        }
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum XferFunc {
    Default = bindings::v4l2_xfer_func_V4L2_XFER_FUNC_DEFAULT,
    F709 = bindings::v4l2_xfer_func_V4L2_XFER_FUNC_709,
    Srgb = bindings::v4l2_xfer_func_V4L2_XFER_FUNC_SRGB,
    OpRgb = bindings::v4l2_xfer_func_V4L2_XFER_FUNC_OPRGB,
    Smpte240M = bindings::v4l2_xfer_func_V4L2_XFER_FUNC_SMPTE240M,
    None = bindings::v4l2_xfer_func_V4L2_XFER_FUNC_NONE,
    DciP3 = bindings::v4l2_xfer_func_V4L2_XFER_FUNC_DCI_P3,
    Smpte2084 = bindings::v4l2_xfer_func_V4L2_XFER_FUNC_SMPTE2084,
}

impl From<u32> for XferFunc {
    fn from(xfer_func: u32) -> Self {
        match xfer_func {
            bindings::v4l2_xfer_func_V4L2_XFER_FUNC_DEFAULT => XferFunc::Default,
            bindings::v4l2_xfer_func_V4L2_XFER_FUNC_709 => XferFunc::F709,
            bindings::v4l2_xfer_func_V4L2_XFER_FUNC_SRGB => XferFunc::Srgb,
            bindings::v4l2_xfer_func_V4L2_XFER_FUNC_OPRGB => XferFunc::OpRgb,
            bindings::v4l2_xfer_func_V4L2_XFER_FUNC_SMPTE240M => XferFunc::Smpte240M,
            bindings::v4l2_xfer_func_V4L2_XFER_FUNC_NONE => XferFunc::None,
            bindings::v4l2_xfer_func_V4L2_XFER_FUNC_DCI_P3 => XferFunc::DciP3,
            bindings::v4l2_xfer_func_V4L2_XFER_FUNC_SMPTE2084 => XferFunc::Smpte2084,
            _ => XferFunc::Default,
        }
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum YCbCrEncoding {
    Default = bindings::v4l2_ycbcr_encoding_V4L2_YCBCR_ENC_DEFAULT,
    E601 = bindings::v4l2_ycbcr_encoding_V4L2_YCBCR_ENC_601,
    E709 = bindings::v4l2_ycbcr_encoding_V4L2_YCBCR_ENC_709,
    Xv601 = bindings::v4l2_ycbcr_encoding_V4L2_YCBCR_ENC_XV601,
    Xv709 = bindings::v4l2_ycbcr_encoding_V4L2_YCBCR_ENC_XV709,
    Sycc = bindings::v4l2_ycbcr_encoding_V4L2_YCBCR_ENC_SYCC,
    Bt2020 = bindings::v4l2_ycbcr_encoding_V4L2_YCBCR_ENC_BT2020,
    Bt2020ConstLum = bindings::v4l2_ycbcr_encoding_V4L2_YCBCR_ENC_BT2020_CONST_LUM,
    Smpte240M = bindings::v4l2_ycbcr_encoding_V4L2_YCBCR_ENC_SMPTE240M,
}

impl From<u32> for YCbCrEncoding {
    fn from(ycbcr_encoding: u32) -> Self {
        match ycbcr_encoding {
            bindings::v4l2_ycbcr_encoding_V4L2_YCBCR_ENC_DEFAULT => YCbCrEncoding::Default,
            bindings::v4l2_ycbcr_encoding_V4L2_YCBCR_ENC_601 => YCbCrEncoding::E601,
            bindings::v4l2_ycbcr_encoding_V4L2_YCBCR_ENC_709 => YCbCrEncoding::E709,
            bindings::v4l2_ycbcr_encoding_V4L2_YCBCR_ENC_XV601 => YCbCrEncoding::Xv601,
            bindings::v4l2_ycbcr_encoding_V4L2_YCBCR_ENC_XV709 => YCbCrEncoding::Xv709,
            bindings::v4l2_ycbcr_encoding_V4L2_YCBCR_ENC_SYCC => YCbCrEncoding::Sycc,
            bindings::v4l2_ycbcr_encoding_V4L2_YCBCR_ENC_BT2020 => YCbCrEncoding::Bt2020,
            bindings::v4l2_ycbcr_encoding_V4L2_YCBCR_ENC_BT2020_CONST_LUM => YCbCrEncoding::Bt2020ConstLum,
            bindings::v4l2_ycbcr_encoding_V4L2_YCBCR_ENC_SMPTE240M => YCbCrEncoding::Smpte240M,
            _ => YCbCrEncoding::Default,
        }
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum Quantization {
    Default = bindings::v4l2_quantization_V4L2_QUANTIZATION_DEFAULT,
    FullRange = bindings::v4l2_quantization_V4L2_QUANTIZATION_FULL_RANGE,
    LimRange = bindings::v4l2_quantization_V4L2_QUANTIZATION_LIM_RANGE,
}

impl From<u32> for Quantization {
    fn from(quantization: u32) -> Self {
        match quantization {
            bindings::v4l2_quantization_V4L2_QUANTIZATION_DEFAULT => Quantization::Default,
            bindings::v4l2_quantization_V4L2_QUANTIZATION_FULL_RANGE => Quantization::FullRange,
            bindings::v4l2_quantization_V4L2_QUANTIZATION_LIM_RANGE => Quantization::LimRange,
            _ => Quantization::Default,
        }
    }
}

#[derive(Debug)]
pub struct FwhtParams {
    pub backward_ref_ts: u64,
    pub version: u32,
    pub width: u32,
    pub height: u32,
    pub flags: u32, //TODO: separate into: a "bitflags" for actual flags and two numbers for #components and pixel encoding type
    pub colorspace: Colorspace,
    pub xfer_func: XferFunc,
    pub ycbcr_enc: YCbCrEncoding,
    pub quantization: Quantization,
}

// Notice that this is private
//
// IMHO better to have size differences than to allocate dynamic memory on
// *every* call. Otherwise the inner type can be boxed, but still it will *not*
// need into/from_raw(), in that case you can cast the box to a pointer.
#[allow(clippy::large_enum_variant)]
enum CompoundControl {
    H264Sps(bindings::v4l2_ctrl_h264_sps),
    H264Pps(bindings::v4l2_ctrl_h264_pps),
    H264ScalingMatrix(bindings::v4l2_ctrl_h264_scaling_matrix),
    H264PredWeights(bindings::v4l2_ctrl_h264_pred_weights),
    H264SliceParams(bindings::v4l2_ctrl_h264_slice_params),
    H264DecodeParams(bindings::v4l2_ctrl_h264_decode_params),
    FwhtParams(bindings::v4l2_ctrl_fwht_params),
}

impl CompoundControl {
    fn size(&self) -> usize {
        match self {
            CompoundControl::H264Sps(_) => todo!(),
            CompoundControl::H264Pps(_) => todo!(),
            CompoundControl::H264ScalingMatrix(_) => todo!(),
            CompoundControl::H264PredWeights(_) => todo!(),
            CompoundControl::H264SliceParams(_) => todo!(),
            CompoundControl::H264DecodeParams(_) => todo!(),
            CompoundControl::FwhtParams(inner) => std::mem::size_of_val(inner),
        }
    }

    fn as_mut_ptr(&mut self) -> *mut std::ffi::c_void {
        match self {
            CompoundControl::H264Sps(_) => todo!(),
            CompoundControl::H264Pps(_) => todo!(),
            CompoundControl::H264ScalingMatrix(_) => todo!(),
            CompoundControl::H264PredWeights(_) => todo!(),
            CompoundControl::H264SliceParams(_) => todo!(),
            CompoundControl::H264DecodeParams(_) => todo!(),
            CompoundControl::FwhtParams(inner) => inner as *mut _ as *mut std::ffi::c_void,
        }
    }
}

impl TryFrom<ExtControlKind> for CompoundControl {
    type Error = ExtControlError;

    fn try_from(value: ExtControlKind) -> Result<Self, Self::Error> {
        match value {
            ExtControlKind::H264DecodeMode | ExtControlKind::H264StartCode => Err(ExtControlError::NotACompoundControlError),
            ExtControlKind::H264Sps => todo!(),
            ExtControlKind::H264Pps => todo!(),
            ExtControlKind::H264ScalingMatrix => todo!(),
            ExtControlKind::H264PredWeights => todo!(),
            ExtControlKind::H264SliceParams => todo!(),
            ExtControlKind::H264DecodeParams => todo!(),
            ExtControlKind::FwhtParams => Ok(CompoundControl::FwhtParams(bindings::v4l2_ctrl_fwht_params { .. unsafe {mem::zeroed()}})),
        }
    }
}

pub enum ExtControl {
    H264DecodeMode,
    H264StartCode,
    H264Sps,
    H264Pps,
    H264ScalingMatrix,
    H264PredWeights,
    H264SliceParams,
    H264DecodeParams,
    FwhtParams(FwhtParams),
}

impl From<CompoundControl> for ExtControl {
    fn from(ctrl: CompoundControl) -> Self {
        match ctrl {
            CompoundControl::H264Sps(_) => todo!(),
            CompoundControl::H264Pps(_) => todo!(),
            CompoundControl::H264ScalingMatrix(_) => todo!(),
            CompoundControl::H264PredWeights(_) => todo!(),
            CompoundControl::H264SliceParams(_) => todo!(),
            CompoundControl::H264DecodeParams(_) => todo!(),
            CompoundControl::FwhtParams(inner) => {
                ExtControl::FwhtParams(
                FwhtParams {
                    backward_ref_ts: inner.backward_ref_ts,
                    version: inner.version,
                    width: inner.width,
                    height: inner.height,
                    flags: inner.height,
                    colorspace: Colorspace::from(inner.colorspace),
                    xfer_func: XferFunc::from(inner.xfer_func),
                    ycbcr_enc: YCbCrEncoding::from(inner.ycbcr_enc),
                    quantization: Quantization::from(inner.quantization),
                }
            )
            }
        }
    }
}

#[doc(hidden)]
mod ioctl {
    use crate::bindings::v4l2_ext_controls;
    nix::ioctl_readwrite!(vidioc_g_ext_ctrls, b'V', 71, v4l2_ext_controls);
    nix::ioctl_readwrite!(vidioc_s_ext_ctrls, b'V', 72, v4l2_ext_controls);
}

#[derive(Debug, Error)]
pub enum ExtControlError {
    #[error("Unexpected ioctl error: {0}")]
    IoctlError(nix::Error),
    #[error("This control is not a compound control")]
    NotACompoundControlError,
}

// Get a single extended control
//
// While vidioc_g_ext_ctrls() accepts an array of controls,
// the kernel internally seems not to commit all the values atomically,
// and proceeds only until the first failure.
//
// Given the above we provide an interface for querying a single control,
// as this greatly simplifies the code.
pub fn g_ext_ctrl<F: AsRawFd>(
    fd: &F,
    ctrl_kind: ExtControlKind
) -> Result<ExtControl, ExtControlError> {
    let mut control = bindings::v4l2_ext_control {
        .. unsafe { mem::zeroed() }
    };
    let mut controls = bindings::v4l2_ext_controls {
        .. unsafe { mem::zeroed() }
    };

    let mut compound_ctrl = CompoundControl::try_from(ctrl_kind)?;

    control.id = ctrl_kind as u32;
    control.size = compound_ctrl.size() as u32;
    control.__bindgen_anon_1.ptr = compound_ctrl.as_mut_ptr();

    controls.__bindgen_anon_1.which = bindings::V4L2_CTRL_WHICH_CUR_VAL;
    controls.count = 1;
    controls.controls = &mut control;

    match unsafe { ioctl::vidioc_g_ext_ctrls(fd.as_raw_fd(), &mut controls) } {
        Ok(_) => Ok(ExtControl::from(compound_ctrl)),
        Err(e) => Err(ExtControlError::IoctlError(e)),
    }
}