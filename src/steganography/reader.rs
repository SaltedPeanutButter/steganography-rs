use std::vec;
use std::{io::Read, path::Path};

use flate2::read::ZlibDecoder;

use crate::errors::{ErrorKind, Result};
use crate::prelude::*;

pub trait Reader {
    fn from_path<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
        Self: Sized;
    fn read(self) -> Result<Vec<u8>>
    where
        Self: Sized;
}

#[derive(Debug, Clone)]
pub struct DefaultReader {
    image: PngImage,
}

impl Reader for DefaultReader {
    fn from_path<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
        Self: Sized,
    {
        Ok(Self {
            image: PngImage::load_static_png(path)?,
        })
    }

    fn read(self) -> Result<Vec<u8>>
    where
        Self: Sized,
    {
        let data = self.image.desten()?;
        let flag = data[0];

        if DefaultReader::is_compression_enabled(flag) {
            DefaultReader::decompress(&data[1..])
        } else {
            Ok(data[1..].to_vec())
        }
    }
}

impl DefaultReader {
    pub fn from_image(image: PngImage) -> Result<Self> {
        Ok(Self { image })
    }

    fn is_compression_enabled(flag: u8) -> bool {
        flag & (1 << 0) != 0
    }

    fn decompress(data: &[u8]) -> Result<Vec<u8>> {
        let mut decoder = ZlibDecoder::new(data);
        let mut buffer = vec![];
        decoder
            .read_to_end(&mut buffer)
            .map_err(|e| ErrorKind::TransformationError(Box::new(e)))?;
        Ok(buffer)
    }
}
