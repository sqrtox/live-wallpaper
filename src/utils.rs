use windows::Win32::Foundation::HWND;

pub fn is_valid_hwnd(hwnd: &HWND) -> bool {
    hwnd.0 != 0
}
