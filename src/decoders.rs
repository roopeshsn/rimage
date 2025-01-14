use std::{error::Error, fs, io, panic, path};

use log::info;
use mozjpeg::Decompress;
use rgb::{FromSlice, RGBA8};

/// Decodes an image file to a vector of RGBA8 pixels, along with the image's width and height.
///
/// Result is
/// - Ok with tuple (pixels in RGBA8, width, height)
/// - Err if error occurs from decode functions
/// - Err if input_format not supported
///
/// # Panics
/// This function will panic if file has no extension
///
/// TODO: Return error if file has no extension
pub fn decode_image(path: &path::PathBuf) -> Result<(Vec<RGBA8>, usize, usize), Box<dyn Error>> {
    let input_format = path.extension().unwrap();
    info!("Started decoding...");
    let decoded = match input_format.to_str() {
        Some("jpg") | Some("jpeg") => decode_jpeg(path)?,
        Some("png") => decode_png(path)?,
        _ => {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "File not supported",
            )))
        }
    };
    info!("Decoded {} pixels", decoded.0.len());

    Ok(decoded)
}

fn decode_jpeg(path: &path::PathBuf) -> Result<(Vec<RGBA8>, usize, usize), Box<dyn Error>> {
    panic::catch_unwind(|| -> Result<(Vec<RGBA8>, usize, usize), Box<dyn Error>> {
        let file_content = fs::read(path)?;

        let d = Decompress::new_mem(&file_content)?;
        let mut image = d.rgba()?;

        let width = image.width();
        let height = image.height();

        let pixels: Vec<RGBA8> = image.read_scanlines().unwrap();

        assert!(image.finish_decompress());
        Ok((pixels, width, height))
    })
    .unwrap_or(Err(Box::new(io::Error::new(
        io::ErrorKind::InvalidData,
        "Failed to read jpeg",
    ))))
}

fn decode_png(path: &path::PathBuf) -> Result<(Vec<RGBA8>, usize, usize), io::Error> {
    let d = png::Decoder::new(fs::File::open(path)?);
    let mut reader = d.read_info()?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf)?;
    let bytes = &buf[..info.buffer_size()];
    let pixels: Vec<RGBA8> = match info.color_type {
        png::ColorType::Grayscale => bytes
            .as_gray()
            .iter()
            .map(|pixel| RGBA8::new(pixel.0, pixel.0, pixel.0, 255))
            .collect(),
        png::ColorType::Rgb => bytes
            .as_rgb()
            .iter()
            .map(|pixel| pixel.alpha(255))
            .collect(),
        png::ColorType::GrayscaleAlpha => bytes
            .as_gray_alpha()
            .iter()
            .map(|pixel| RGBA8::new(pixel.0, pixel.0, pixel.0, pixel.1))
            .collect(),
        png::ColorType::Rgba => bytes.as_rgba().to_owned(),
        png::ColorType::Indexed => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "File ColorScheme not supported",
            ))
        }
    };
    Ok((pixels, info.width as usize, info.height as usize))
}
