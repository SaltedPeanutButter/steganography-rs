use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::errors::{ErrorKind, Result};
use crate::prelude::*;

pub trait Stenable {
    fn sten(&mut self, data: &[u8]) -> Result<()>;
    fn desten(&self) -> Result<Vec<u8>>;
}

impl Stenable for Vec<u8> {
    fn sten(&mut self, data: &[u8]) -> Result<()> {
        // Write length.
        for i in 0..64 {
            let bit = (data.len() >> i) & 1;
            self[i] = (self[i] & 0xFE) | bit as u8;
        }

        // Compute and write hash.
        let hasher = &mut DefaultHasher::new();
        data.hash(hasher);
        let hash = hasher.finish();

        for i in 0..64 {
            let bit = (hash >> i) & 1;
            self[i + 64] = (self[i + 64] & 0xFE) | bit as u8;
        }

        // Check if the data fits into the PNG image.
        if data.len() * 8 > self.len() {
            return Err(ErrorKind::DataTooLarge(data.len() * 8, self.len()));
        }

        // Iterate over data bytes.
        for (i, byte) in data.iter().enumerate() {
            // Iterate over bits in the current byte.
            for j in 0..8 {
                // Get the current bit.
                let bit = (byte >> j) & 1;

                // Set the bit in the PNG image.
                self[128 + i * 8 + j] = (self[i * 8 + j] & 0xFE) | bit;
            }
        }
        Ok(())
    }

    fn desten(&self) -> Result<Vec<u8>> {
        // Read length.
        let mut length = 0usize;
        for i in 0..64 {
            length |= ((self[i] & 1) as usize) << i;
        }

        // Read hash.
        let mut hash = 0u64;
        for i in 0..64 {
            hash |= (((self[i + 64] & 1) as usize) << i) as u64;
        }

        // Iterate over data bytes.
        let mut data = vec![];
        for i in 0..length {
            // Read the current byte.
            let mut byte = 0;
            for j in 0..8 {
                byte |= (self[128 + i * 8 + j] & 1) << j;
            }
            data.push(byte);
        }

        // Verify hash.
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        if hasher.finish() != hash {
            return Err(ErrorKind::HashMismatch(hasher.finish(), hash));
        }
        Ok(data)
    }
}

impl Stenable for PngImage {
    fn sten(&mut self, data: &[u8]) -> Result<()> {
        // Write length.
        for i in 0..64 {
            let bit = (data.len() >> i) & 1;
            self[i] = (self[i] & 0xFE) | bit as u8;
        }

        // Compute and write hash.
        let hasher = &mut DefaultHasher::new();
        data.hash(hasher);
        let hash = hasher.finish();

        for i in 0..64 {
            let bit = (hash >> i) & 1;
            self[i + 64] = (self[i + 64] & 0xFE) | bit as u8;
        }

        // Check if the data fits into the PNG image.
        if data.len() * 8 > self.data.len() {
            return Err(ErrorKind::DataTooLarge(data.len() * 8, self.data.len()));
        }

        // Iterate over data bytes.
        for (i, byte) in data.iter().enumerate() {
            // Iterate over bits in the current byte.
            for j in 0..8 {
                // Get the current bit.
                let bit = (byte >> j) & 1;

                // Set the bit in the PNG image.
                self[128 + i * 8 + j] = (self[i * 8 + j] & 0xFE) | bit;
            }
        }
        Ok(())
    }

    fn desten(&self) -> Result<Vec<u8>> {
        // Read length.
        let mut length = 0usize;
        for i in 0..64 {
            length |= ((self[i] & 1) as usize) << i;
        }

        // Read hash.
        let mut hash = 0u64;
        for i in 0..64 {
            hash |= (((self[i + 64] & 1) as usize) << i) as u64;
        }

        // Iterate over data bytes.
        let mut data = vec![];
        for i in 0..length {
            // Read the current byte.
            let mut byte = 0;
            for j in 0..8 {
                byte |= (self[128 + i * 8 + j] & 1) << j;
            }
            data.push(byte);
        }

        // Verify hash.
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        if hasher.finish() != hash {
            return Err(ErrorKind::HashMismatch(hasher.finish(), hash));
        }
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn vec_u8_stenable_test() {
        // Generate 64 bytes worth of random data.
        let mut data = [0u8; 64];
        rand::thread_rng().fill(&mut data);

        // Generate 1024 bytes worth of data to act as container.
        let mut container = [0u8; 1024];
        rand::thread_rng().fill(&mut container);
        let mut container = container.to_vec();

        // Sten data.
        container.sten(&data).unwrap();

        // Desten data.
        let extracted_data = container.desten().unwrap();

        // Compare data.
        assert_eq!(extracted_data, data);
    }
}
