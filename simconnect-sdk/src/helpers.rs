use std::ffi::CString;

pub fn fixed_c_str_to_string(data: &[i8]) -> String {
    let u8slice = unsafe { &*(data as *const _ as *const [u8]) };

    let mut bytes = u8slice.to_vec();

    let pos = bytes.iter().position(|c| *c == 0).unwrap_or(bytes.len());

    bytes.truncate(pos);
    let result = unsafe { CString::from_vec_unchecked(bytes) };

    result.to_str().unwrap_or_default().to_string()
}
