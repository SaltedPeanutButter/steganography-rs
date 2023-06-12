use steganography_lib::prelude::*;

use rand::Rng;

#[test]
fn default_writer_and_reader_test() {
    // Generate 1kb worth of random data.
    let mut data = [0u8; 1024];
    rand::thread_rng().fill(&mut data);

    // Sten data.
    let modified_image = DefaultWriter::from_path("png_samples/high_res_nature.png")
        .unwrap()
        .write(&data)
        .unwrap()
        .to_container();

    // Desten data.
    let retrieved_data = DefaultReader::from_image(modified_image)
        .unwrap()
        .read()
        .unwrap();

    // Compare.
    assert_eq!(&retrieved_data[..], data);
}

#[test]
fn default_writer_and_reader_with_compression_test() {
    // Generate 1kb worth of random data.
    let mut data = [0u8; 1024];
    rand::thread_rng().fill(&mut data);

    // Sten data.
    let modified_image = DefaultWriter::from_path("png_samples/high_res_nature.png")
        .unwrap()
        .set_compression(true)
        .write(&data)
        .unwrap()
        .to_container();

    // Desten data.
    let retrieved_data = DefaultReader::from_image(modified_image)
        .unwrap()
        .read()
        .unwrap();

    // Compare.
    assert_eq!(&retrieved_data[..], data);
}