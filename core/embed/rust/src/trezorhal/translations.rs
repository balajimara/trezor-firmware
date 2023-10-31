use super::ffi;

#[repr(C)]
pub struct PointerData {
    pub ptr: *const u8,
    pub len: u32,
}

pub fn translations_get() -> &'static [u8] {
    let mut len: u32 = 0;
    let ptr = unsafe { ffi::translations_read(&mut len, 0) };
    if ptr.is_null() {
        fatal_error!("Translations read failed", "");
    }
    unsafe { core::slice::from_raw_parts(ptr, len as usize) }
}

pub fn get_font_pointer(area_offset: u16, len: u16) -> PointerData {
    let mut overall_len: u32 = 0;
    let ptr = unsafe { ffi::translations_read(&mut overall_len, area_offset.into()) };
    if ptr.is_null() {
        fatal_error!("Translations read failed", "");
    }
    PointerData {
        ptr,
        len: len.into(),
    }
}
