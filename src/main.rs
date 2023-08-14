mod error;
mod gif;
mod progman;
mod utils;
mod worker_w;

use std::ffi::c_void;
use std::time::Duration;

use windows::Win32::Graphics::Gdi::{
    BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC, ReleaseDC,
    SelectObject, SetDIBits, BITMAPINFO, DIB_RGB_COLORS, SRCCOPY,
};

use windows::Win32::Graphics::Gdi::*;

use clap::Parser;
use progman::send_message;
use worker_w::get_worker_w;

#[derive(Debug, Parser)]
struct Args {
    file_path: String,
}

#[tokio::main]
async fn main() {
    let Args { file_path } = Args::parse();
    let frames = match gif::read(file_path) {
        Ok(frames) => frames,
        Err(_) => {
            println!("GIFの読み取りに失敗しました");

            return;
        }
    };

    match send_message() {
        Ok(_) => {}
        Err(_) => {
            println!("失敗しました");

            return;
        }
    }

    let worker_w = match get_worker_w() {
        Ok(hwnd) => hwnd,
        Err(_) => {
            println!("ウィンドウのハンドルが見つかりません");

            return;
        }
    };
    println!("spawn");
    let handle = tokio::spawn(async move {
        loop {
            for frame in &frames {
                let dc = unsafe { GetDC(worker_w) };
                let cv = unsafe { CreateCompatibleDC(dc) };
                let bmp =
                    unsafe { CreateCompatibleBitmap(dc, frame.width as i32, frame.height as i32) };
                let mut bi = BITMAPINFO::default();

                bi.bmiHeader = frame.header;

                unsafe {
                    SetDIBits(
                        cv,
                        bmp,
                        0,
                        frame.height as u32,
                        frame.bgr.as_ptr() as *const c_void,
                        &bi,
                        DIB_RGB_COLORS,
                    );
                }

                let mut bf = BLENDFUNCTION::default();

                bf.BlendOp = AC_SRC_OVER as u8;
                bf.BlendFlags = 0;
                bf.SourceConstantAlpha = 255;
                bf.AlphaFormat = AC_SRC_ALPHA as u8;

                unsafe {
                    SelectObject(cv, bmp);
                    AlphaBlend(
                        dc,
                        frame.left as i32,
                        frame.top as i32,
                        frame.width as i32,
                        frame.height as i32,
                        cv,
                        0,
                        0,
                        frame.width as i32,
                        frame.height as i32,
                        bf,
                    );
                    /*BitBlt(
                        dc,
                        frame.left as i32,
                        frame.top as i32,
                        frame.width as i32,
                        frame.height as i32,
                        cv,
                        0,
                        0,
                        SRCCOPY,
                    );*/
                }

                unsafe {
                    DeleteObject(bmp);
                    DeleteDC(cv);
                    ReleaseDC(worker_w, dc);
                }

                tokio::time::sleep(Duration::from_millis((10 * frame.delay).into())).await;
            }
        }
    })
    .await;

    match handle {
        Ok(_) => {}
        Err(_) => {}
    }
}
