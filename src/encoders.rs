use std::{
    fs,
    io::{self, BufWriter, Cursor},
    panic, path,
};

use log::{debug, info};
use mozjpeg::Compress;
use rgb::{ComponentBytes, RGBA8};

/// Encodes an image to file at path from a vector of RGBA8 pixels, width, height, output format and quality.
///
/// Result is
/// - Ok(()) if no errors
/// - Err if file cannot be created or changed
/// - Err if file format is not supported
///
/// # Panics
/// This function will panic if Error occurs in encode functions
///
/// TODO: Return error if inner functions returns error
pub fn encode_image(
    path: &path::PathBuf,
    pixels: &[RGBA8],
    output_format: &str,
    width: usize,
    height: usize,
    quality: f32,
) -> Result<(), io::Error> {
    info!("Started encoding...");
    let file_bytes = match output_format {
        "jpg" | "jpeg" => encode_jpeg(pixels, width, height, quality)?,
        "png" => encode_png(pixels, width, height)?,
        _ => return Err(io::Error::new(io::ErrorKind::Other, "File not supported")),
    };
    info!("Encoded {} bytes", file_bytes.len());

    let mut path = path.to_owned();
    path.set_extension(output_format);
    fs::write(path, file_bytes)
}

fn encode_jpeg(
    pixels: &[RGBA8],
    width: usize,
    height: usize,
    quality: f32,
) -> Result<Vec<u8>, io::Error> {
    panic::catch_unwind(|| -> Result<Vec<u8>, io::Error> {
        let mut comp = Compress::new(mozjpeg::ColorSpace::JCS_EXT_RGBA);

        comp.set_size(width, height);
        comp.set_quality(quality * 100.0);
        comp.set_color_space(mozjpeg::ColorSpace::JCS_YCbCr);
        comp.set_mem_dest();
        comp.start_compress();

        assert!(comp.write_scanlines(pixels[..].as_bytes()));

        comp.finish_compress();
        Ok(comp.data_to_vec().unwrap())
    })
    .unwrap_or(Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "Failed to read jpeg",
    )))
}

fn encode_png(pixels: &[RGBA8], width: usize, height: usize) -> Result<Vec<u8>, io::Error> {
    let mut buf = Cursor::new(Vec::with_capacity(width * height * 4));
    {
        debug!("Started encoding png...");
        let w = &mut BufWriter::new(&mut buf);
        let mut encoder = png::Encoder::new(w, width as u32, height as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut w = encoder.write_header()?;
        w.write_image_data(pixels.as_bytes())?;
        w.finish()?;
    }

    debug!("Encoded {} bytes", buf.get_ref().len());
    let opts = oxipng::Options::from_preset(2);

    let data =
        oxipng::optimize_from_memory(buf.get_ref(), &opts).unwrap_or(buf.get_ref().to_owned());
    Ok(data)
}
