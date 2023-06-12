use png;
use std::io::BufWriter;
use std::ops::Index;
use std::path::Path;
use std::{fs::File, ops::IndexMut};

use crate::errors::{ErrorKind, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PngImage {
    color_type: png::ColorType,
    bit_depth: png::BitDepth,
    source_gamma: Option<png::ScaledFloat>,
    source_chromaticities: Option<png::SourceChromaticities>,
    width: u32,
    height: u32,
    pub data: Vec<u8>,
}

impl PngImage {
    pub fn load_static_png<P>(path: P) -> Result<PngImage>
    where
        P: AsRef<Path>,
    {
        // Open the file at the given path.
        let file = File::open(path).map_err(|e| ErrorKind::SysIOError(e))?;

        // Decode the PNG image.
        let decoder = png::Decoder::new(file);
        let mut reader = decoder
            .read_info()
            .map_err(|e| ErrorKind::PngDecodingError(e))?;

        // Create an empty buffer and read the first (static) frame.
        let mut buf = vec![0; reader.output_buffer_size()];
        reader
            .next_frame(&mut buf)
            .map_err(|e| ErrorKind::PngDecodingError(e))?;

        let info = reader.info();
        Ok(PngImage {
            color_type: info.color_type,
            bit_depth: info.bit_depth,
            source_gamma: info.source_gamma,
            source_chromaticities: info.source_chromaticities,
            width: info.width,
            height: info.height,
            data: buf,
        })
    }

    pub fn save_static_png<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        // Open the file at the given path.
        let file = File::create(path).map_err(|e| ErrorKind::SysIOError(e))?;

        // Create PNG encoder.
        let w = &mut BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.width, self.height);

        // Set PNG encoder parameters.
        encoder.set_color(self.color_type);
        encoder.set_depth(self.bit_depth);
        if let Some(sg) = self.source_gamma {
            encoder.set_source_gamma(sg);
        }
        if let Some(sc) = self.source_chromaticities {
            encoder.set_source_chromaticities(sc);
        }

        // Create PNG writer
        let mut writer = encoder
            .write_header()
            .map_err(|e| ErrorKind::PngEncodingError(e))?;

        // Write the image data.
        writer
            .write_image_data(&self.data)
            .map_err(|e| ErrorKind::PngEncodingError(e))?;

        Ok(())
    }
}

impl Index<usize> for PngImage {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for PngImage {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_png_io() {
        let png = PngImage::load_static_png("png_samples/high_res_nature.png").unwrap();
        png.save_static_png("test2.png").unwrap();
        std::fs::remove_file("test2.png").unwrap();
    }
}
