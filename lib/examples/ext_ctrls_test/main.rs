use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use std::fs::File;
use std::os::unix::io::FromRawFd;
use v4l2r::ioctl::*;

const DEVICE_PATH: &str = "/dev/video2";

fn main() {
    let fd = unsafe {
        File::from_raw_fd(
            open(DEVICE_PATH, OFlag::O_RDWR | OFlag::O_CLOEXEC, Mode::empty())
                .unwrap_or_else(|_| panic!("Cannot open {}", DEVICE_PATH)),
        )
    };

    match g_ext_ctrl(&fd, ExtControlKind::FwhtParams) {
        Ok(fwht_params) => {
            if let ExtControl::FwhtParams(fwht_params) = fwht_params {
                println!("{:?}", fwht_params);
            }
        }
        Err(e) => println!("{e}"),
    };

}