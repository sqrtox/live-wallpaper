use std::fs::File;
use std::io::Read;
use std::mem;
use std::path::Path;
use std::ptr;

use gif::{ColorOutput, DecodeOptions, Decoder, DisposalMethod, Frame};
use windows::Win32::Graphics::Gdi::{BITMAPINFOHEADER, BI_BITFIELDS, BI_RGB};

use crate::error::Result;

fn create_decoder<R>(r: R) -> Result<Decoder<R>>
where
    R: Read,
{
    let mut decoder = DecodeOptions::new();

    decoder.set_color_output(ColorOutput::RGBA);

    let decoder = decoder.read_info(r)?;

    Ok(decoder)
}

#[derive(Debug, Clone)]
pub struct GifFrameBitmap {
    pub dispose: DisposalMethod,
    pub delay: u16,
    pub top: u16,
    pub left: u16,
    pub width: u16,
    pub height: u16,
    pub bgr: Vec<u8>,
    pub header: BITMAPINFOHEADER,
}

pub fn read<P>(path: P) -> Result<Vec<GifFrameBitmap>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let mut decoder = create_decoder(file)?;
    let mut result = vec![];

    while let Some(Frame {
        delay,
        width,
        dispose,
        height,
        top,
        left,
        buffer,
        ..
    }) = decoder.read_next_frame()?
    {
        let width = width.clone();
        let height = height.clone();
        let bgr = buffer
            .chunks(width as usize * 4)
            .rev()
            .map(|v| v.chunks(4).map(|v| [v[2], v[1], v[0], v[3]]).flatten())
            .flatten()
            .map(|v| v.clone())
            .collect::<Vec<_>>();
        let data_len = width as usize * height as usize * 4;
        // let remain = width as usize * 3 % 4;
        // let data_len = if remain > 0 {
        //     let chunk_size = width as usize * 3;
        //     let line_bytes_len = chunk_size + 4 - remain;
        //     let data_len = line_bytes_len * height as usize;

        //     bgr.reserve(data_len);

        //     let mut p = bgr.as_mut_ptr();

        //     for c in bgr.chunks(chunk_size) {
        //         unsafe {
        //             ptr::copy_nonoverlapping(c.as_ptr(), p, chunk_size);
        //             p = p.add(line_bytes_len);
        //         }
        //     }

        //     data_len
        // } else {
        //     width as usize * height as usize * 3
        // };

        let mut header = BITMAPINFOHEADER::default();

        header.biSize = mem::size_of::<BITMAPINFOHEADER>() as u32;
        header.biWidth = width as i32;
        header.biHeight = height as i32;
        header.biPlanes = 1;
        header.biBitCount = 32;
        header.biSizeImage = data_len as u32;
        header.biClrImportant = 0;
        header.biCompression = BI_RGB.0 as u32;

        result.push(GifFrameBitmap {
            width,
            bgr,
            dispose: dispose.clone(),
            top: top.clone(),
            left: left.clone(),
            height,
            delay: delay.clone(),
            header,
        });
    }

    Ok(result)
}
