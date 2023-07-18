use std::ffi::CString;

pub fn fixed_c_str_to_string(data: &[i8]) -> String {
    let u8slice = unsafe { &*(data as *const _ as *const [u8]) };

    let mut bytes = u8slice.to_vec();

    let pos = bytes.iter().position(|c| *c == 0).unwrap_or(bytes.len());

    bytes.truncate(pos);
    let result = unsafe { CString::from_vec_unchecked(bytes) };

    result.to_str().unwrap_or_default().to_string()
}

pub type BCD16 = u16;
pub fn u16_to_bcd16(value: u16) -> u16 {
    let mut result: BCD16 = 0;
    let mut working_value = value;

    for index in 0..4 {
        let digit = working_value % 10;
        result = result | (digit << (index * 4));

        if working_value >= digit {
            working_value = (working_value - digit) / 10;
        } else {
            break;
        }
    }

    return result;
}

pub fn bcd16_to_u16(value: BCD16) -> u16 {
    let mut result: u16 = 0;
    let mut working_value = value;

    for index in 0..4 {
        let digit = working_value & 15;
        result += digit * (u16::pow(10, index));

        working_value = working_value >> 4;
    }

    return result;
}