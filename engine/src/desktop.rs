use windows_sys::Win32::Foundation::POINT;
use windows_sys::Win32::Graphics::Gdi::{MONITOR_DEFAULTTONEAREST, MonitorFromPoint};
use windows_sys::Win32::UI::HiDpi::{GetDpiForMonitor, MDT_RAW_DPI};
use windows_sys::Win32::UI::WindowsAndMessaging::GetCursorPos;

pub struct CursorSample {
    pub x: i32,
    pub y: i32,
    pub dpi_x: u32,
    pub dpi_y: u32,
}

pub fn cursor() -> Option<CursorSample> {
    let mut point = POINT { x: 0, y: 0 };
    if unsafe { GetCursorPos(&mut point) } == 0 {
        return None;
    }

    let monitor = unsafe { MonitorFromPoint(point, MONITOR_DEFAULTTONEAREST) };

    let mut dpi_x: u32 = 0;
    let mut dpi_y: u32 = 0;
    let result = unsafe { GetDpiForMonitor(monitor, MDT_RAW_DPI, &mut dpi_x, &mut dpi_y ) };
    if result != 0 || dpi_x == 0 || dpi_y == 0 {
        return None;
    }

    Some(CursorSample {
        x: point.x,
        y: point.y,
        dpi_x,
        dpi_y,
    })
}
