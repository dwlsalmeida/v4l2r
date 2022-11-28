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

impl From<(ExtControlKind, &bindings::v4l2_ext_control)> for ExtControl {
    fn from((ctrl_kind, ctrl): (ExtControlKind, &bindings::v4l2_ext_control)) -> Self {
        match ctrl_kind {
            ExtControlKind::H264DecodeMode => todo!(),
            ExtControlKind::H264StartCode => todo!(),
            ExtControlKind::H264Sps => todo!(),
            ExtControlKind::H264Pps => todo!(),
            ExtControlKind::H264ScalingMatrix => todo!(),
            ExtControlKind::H264PredWeights => todo!(),
            ExtControlKind::H264SliceParams => todo!(),
            ExtControlKind::H264DecodeParams => todo!(),
            ExtControlKind::FwhtParams =>
                ExtControl::FwhtParams(
                    unsafe {
                        let ptr = ctrl.__bindgen_anon_1.p_fwht_params;
                        FwhtParams {
                            backward_ref_ts: (*ptr).backward_ref_ts,
                            version: (*ptr).version,
                            width: (*ptr).width,
                            height: (*ptr).height,
                            flags: (*ptr).flags,
                            colorspace: Colorspace::from((*ptr).colorspace),
                            xfer_func: XferFunc::from((*ptr).xfer_func),
                            ycbcr_enc: YCbCrEncoding::from((*ptr).ycbcr_enc),
                            quantization: Quantization::from((*ptr).quantization),
                        }
                    }
                ),
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
}

fn free_ctrl_data(ctrl_kind: ExtControlKind, ctrl: &bindings::v4l2_ext_control) {
    match ctrl_kind {
        ExtControlKind::H264DecodeMode => todo!(),
        ExtControlKind::H264StartCode => todo!(),
        ExtControlKind::H264Sps => todo!(),
        ExtControlKind::H264Pps => todo!(),
        ExtControlKind::H264ScalingMatrix => todo!(),
        ExtControlKind::H264PredWeights => todo!(),
        ExtControlKind::H264SliceParams => todo!(),
        ExtControlKind::H264DecodeParams => todo!(),
        ExtControlKind::FwhtParams => unsafe { drop(Box::from_raw(ctrl.__bindgen_anon_1.p_fwht_params)); },
    }
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

    match ctrl_kind {
        ExtControlKind::H264DecodeMode => todo!(),
        ExtControlKind::H264StartCode => todo!(),
        ExtControlKind::H264Sps => todo!(),
        ExtControlKind::H264Pps => todo!(),
        ExtControlKind::H264ScalingMatrix => todo!(),
        ExtControlKind::H264PredWeights => todo!(),
        ExtControlKind::H264SliceParams => todo!(),
        ExtControlKind::H264DecodeParams => todo!(),
        ExtControlKind::FwhtParams =>  {
            let fwht_params = Box::new(
                bindings::v4l2_ctrl_fwht_params { .. unsafe { mem::zeroed() } }
            );
            control.__bindgen_anon_1.p_fwht_params = Box::into_raw(fwht_params);
            control.size = size_of::<bindings::v4l2_ctrl_fwht_params>() as u32;
        },
    };

    control.id = ctrl_kind as u32;

    controls.__bindgen_anon_1.which = bindings::V4L2_CTRL_WHICH_CUR_VAL;
    controls.count = 1;
    controls.controls = &mut control;

    match unsafe { ioctl::vidioc_g_ext_ctrls(fd.as_raw_fd(), &mut controls) } {
        Ok(_) => {
            let ret = Ok(ExtControl::from((ctrl_kind, &control)));
            free_ctrl_data(ctrl_kind, &control);
            ret
        },
        Err(e) => {
            free_ctrl_data(ctrl_kind, &control);
            Err(ExtControlError::IoctlError(e))
        },
    }
}