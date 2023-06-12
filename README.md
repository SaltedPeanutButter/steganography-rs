# `Steganography` for Rust

A `rust` crate for performing steganography on PNG images and more!

## Why this project?

I first heard of steganography through a video by Tom Scott a few years back and I thought it was
an amazing invention. I actually tried to implement it in Python back then, but it was a long time
ago and I forgot where I put the source. So here I am, redoing this project in Rust.

I hope to bring some new improvements though. In the past, I simply decoded the image and just encode
data in the last significant bit (LSB) of each byte. Now that I learn a few more stuff in my programming
journey, I hope to make this project even better.

## Why PNG?

Simple. PNG is undoubtedly the most popular and supported lossless image format.

## Features

Basic features: Encode/decode data into/from a PNG image

New features:
- [ ] Add AES-GCM encryption
- [x] Add compression
- [ ] ~~Use more LSBs~~ _(Will not be implemented due to security concerns)_
- [ ] Implement a useful CLI
- [ ] Implement a REPL
- [ ] Add support for APNG