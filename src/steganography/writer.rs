use std::{io::Write, path::Path};

use flate2::write::ZlibEncoder;

use crate::errors::{ErrorKind, Result};
use crate::prelude::*;

pub trait Writer {
    fn from_path<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
        Self: Sized;
    fn write(self, data: &[u8]) -> Result<Self>
    where
        Self: Sized;
    fn to_path<P>(self, path: P) -> Result<()>
    where
        P: AsRef<Path>;
}

#[derive(Debug, Clone)]
pub struct DefaultWriter {
    compression_enabled: bool,
    image: PngImage,
}

impl Writer for DefaultWriter {
    fn from_path<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
        Self: Sized,
    {
        DefaultWriter::from_image(PngImage::load_static_png(path)?)
    }

    fn write(mut self, data: &[u8]) -> Result<Self> {
        let mut data = if self.compression_enabled {
            Self::compress(data)?
        } else {
            data.to_vec()
        };

        // Insert metadata
        data.insert(0, self.get_flag());

        self.image.sten(&data)?;
        Ok(self)
    }

    fn to_path<P>(self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        self.image.save_static_png(path)
    }
}

impl DefaultWriter {
    pub fn from_image(image: PngImage) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            image,
            compression_enabled: false,
        })
    }

    pub fn set_compression(self, enabled: bool) -> Self {
        Self {
            compression_enabled: enabled,
            ..self
        }
    }

    pub fn to_container(self) -> PngImage {
        self.image
    }

    fn compress(data: &[u8]) -> Result<Vec<u8>> {
        let mut encoder = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        encoder
            .write_all(data)
            .map_err(|e| ErrorKind::TransformationError(Box::new(e)))?;
        Ok(encoder
            .finish()
            .map_err(|e| ErrorKind::TransformationError(Box::new(e)))?)
    }

    fn get_flag(&self) -> u8 {
        let mut flag = 0u8;
        flag |= (self.compression_enabled as u8) << 0;
        flag
    }
}
