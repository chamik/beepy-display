extern crate framebuffer;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use framebuffer::{Framebuffer, FramebufferError};
use std::path::PathBuf;
use std::{fs::File, io::Write};
use thiserror::Error;

pub struct BeepyDisplay {
    pub device: PathBuf,
    fb: Framebuffer,
    frame: Vec<u8>,
}

#[derive(Error, Debug)]
pub enum BeepyDisplayError {
    #[error(transparent)]
    FramebufferError(#[from] FramebufferError),
}

impl BeepyDisplay {
    pub fn new(device: PathBuf) -> Result<Self, BeepyDisplayError> {
        let fb = Framebuffer::new(&device)?;
        let frame = vec![0u8; 400 * 4 * 240 as usize];
        //                                   ^ each pixel is 32 bit

        Ok(Self { device, fb, frame })
    }

    pub fn flush(&mut self) {
        self.fb.write_frame(&self.frame);
    }
}

impl DrawTarget for BeepyDisplay {
    type Color = BinaryColor;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels.into_iter() {
            if let Ok((x @ 0..=399, y @ 0..=239)) = point.try_into() {
                let index: u32 = x * 4 + y * 1600;
                let c = if color.is_on() { 255 } else { 0 };

                self.frame[(index + 0) as usize] = c; // blue
                self.frame[(index + 1) as usize] = c; // green
                self.frame[(index + 2) as usize] = c; // red
                                                      // alpha is left alone
            }
        }

        Ok(())
    }
}

impl OriginDimensions for BeepyDisplay {
    fn size(&self) -> Size {
        Size::new(400, 240)
    }
}

pub fn unbind_console() -> Result<(), std::io::Error> {
    let mut file = File::create("/sys/class/vtconsole/vtcon1/bind")?;
    file.write_all(b"0")
}

pub fn bind_console() -> Result<(), std::io::Error> {
    let mut file = File::create("/sys/class/vtconsole/vtcon1/bind")?;
    file.write_all(b"1")
}
