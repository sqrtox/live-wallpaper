use windows::s;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowA, SendMessageTimeoutA, SEND_MESSAGE_TIMEOUT_FLAGS,
};

use crate::error::{Error, Result};

pub fn send_message() -> Result<()> {
    let progman = match unsafe { FindWindowA(s!("Progman"), None) } {
        HWND(0) => return Err(Error::WindowNotFound),
        hwnd => hwnd,
    };

    unsafe {
        SendMessageTimeoutA(
            progman,
            0x52c,
            WPARAM(0),
            LPARAM(0),
            SEND_MESSAGE_TIMEOUT_FLAGS(0x0),
            1000,
            None,
        );
    }

    Ok(())
}
