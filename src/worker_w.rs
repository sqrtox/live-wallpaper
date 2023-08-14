use windows::s;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::{EnumWindows, FindWindowExA};

use crate::error::{Error, Result};
use crate::utils::is_valid_hwnd;

extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let shell = unsafe { FindWindowExA(hwnd, None, s!("SHELLDLL_DefView"), None) };

    if is_valid_hwnd(&shell) {
        let worker_w = unsafe { FindWindowExA(None, hwnd, s!("WorkerW"), None) };

        if is_valid_hwnd(&worker_w) {
            unsafe {
                *(lparam.0 as *mut isize) = worker_w.0;
            }
        }
    }

    BOOL(1)
}

pub fn get_worker_w() -> Result<HWND> {
    let mut worker_w = 0 as isize;

    unsafe {
        EnumWindows(
            Some(enum_windows_proc),
            LPARAM(&mut worker_w as *mut isize as isize),
        );
    }

    match worker_w {
        0 => Err(Error::WindowNotFound),
        worker_w => Ok(HWND(worker_w)),
    }
}
