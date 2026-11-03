//! Audited Shell/GDI boundary. Only a file resource; no LoadLibrary, OpenProcess or hooks.
use base64::Engine;
use std::{ffi::c_void, os::windows::ffi::OsStrExt, path::Path, ptr::null_mut};
use windows_sys::Win32::{
    Graphics::Gdi::*,
    UI::{Shell::ExtractIconExW, WindowsAndMessaging::*},
};
const SIZE: i32 = 32;
struct Resources {
    icon: HICON,
    screen: HDC,
    dc: HDC,
    bitmap: HBITMAP,
    previous: HGDIOBJ,
}
impl Drop for Resources {
    fn drop(&mut self) {
        // SAFETY: every handle is null or owned by this call. Restore the selected
        // object before deletion; release each resource exactly once on all exits.
        unsafe {
            if !self.previous.is_null() && !self.dc.is_null() {
                SelectObject(self.dc, self.previous);
            }
            if !self.bitmap.is_null() {
                DeleteObject(self.bitmap);
            }
            if !self.dc.is_null() {
                DeleteDC(self.dc);
            }
            if !self.screen.is_null() {
                ReleaseDC(null_mut(), self.screen);
            }
            if !self.icon.is_null() {
                DestroyIcon(self.icon);
            }
        }
    }
}
pub fn extract(path: &Path) -> Option<String> {
    let wide: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
    let mut owned = Resources {
        icon: null_mut(),
        screen: null_mut(),
        dc: null_mut(),
        bitmap: null_mut(),
        previous: null_mut(),
    };
    let mut pixels: *mut c_void = null_mut();
    // SAFETY: NUL-terminated local file name; one output HICON; fixed-size,
    // initialized BITMAPINFO and exactly SIZE*SIZE*4 owned DIB bytes. GDI writes
    // only while its bitmap/DC live. All resources use the RAII guard above.
    let rgba = unsafe {
        if ExtractIconExW(wide.as_ptr(), 0, &mut owned.icon, null_mut(), 1) != 1
            || owned.icon.is_null()
        {
            return None;
        }
        owned.screen = GetDC(null_mut());
        if owned.screen.is_null() {
            return None;
        }
        owned.dc = CreateCompatibleDC(owned.screen);
        if owned.dc.is_null() {
            return None;
        }
        let info = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: SIZE,
                biHeight: -SIZE,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB,
                ..Default::default()
            },
            ..Default::default()
        };
        owned.bitmap =
            CreateDIBSection(owned.dc, &info, DIB_RGB_COLORS, &mut pixels, null_mut(), 0);
        if owned.bitmap.is_null() || pixels.is_null() {
            return None;
        }
        owned.previous = SelectObject(owned.dc, owned.bitmap);
        if owned.previous.is_null() || owned.previous as isize == -1 {
            owned.previous = null_mut();
            return None;
        }
        std::ptr::write_bytes(pixels.cast::<u8>(), 0, (SIZE * SIZE * 4) as usize);
        if DrawIconEx(
            owned.dc,
            0,
            0,
            owned.icon,
            SIZE,
            SIZE,
            0,
            null_mut(),
            DI_NORMAL,
        ) == 0
        {
            return None;
        }
        if GdiFlush() == 0 {
            return None;
        }
        std::slice::from_raw_parts(pixels.cast::<u8>(), (SIZE * SIZE * 4) as usize).to_vec()
    };
    encode(rgba)
}
fn encode(mut rgba: Vec<u8>) -> Option<String> {
    let alpha = rgba.as_chunks::<4>().0.iter().any(|p| p[3] != 0);
    for pixel in rgba.as_chunks_mut::<4>().0 {
        pixel.swap(0, 2);
        if !alpha {
            pixel[3] = if pixel[..3].iter().any(|v| *v != 0) {
                255
            } else {
                0
            };
        } else if pixel[3] > 0 && pixel[3] < 255 {
            let a = u16::from(pixel[3]);
            for channel in &mut pixel[..3] {
                *channel = (u16::from(*channel) * 255 / a).min(255) as u8;
            }
        }
    }
    let mut bytes = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut bytes, SIZE as u32, SIZE as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.write_header().ok()?.write_image_data(&rgba).ok()?;
    }
    Some(format!(
        "data:image/png;base64,{}",
        base64::engine::general_purpose::STANDARD.encode(bytes)
    ))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fixed_pixels_are_encoded_as_bounded_png() {
        let uri = encode(vec![255; 32 * 32 * 4]);
        assert!(uri.as_ref().is_some_and(
            |value| value.starts_with("data:image/png;base64,") && value.len() < 10000
        ));
    }
}
