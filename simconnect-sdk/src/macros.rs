macro_rules! success {
    ($hr:expr) => {{
        let hr = $hr;
        if hr != 0 {
            return Err(SimConnectError::SimConnectError(hr));
        }
    }};
}
pub(crate) use success;

macro_rules! ok_if_fail {
    ($hr:expr, $ret:expr) => {{
        let hr = $hr;
        let ret = $ret;
        if hr != 0 {
            return Ok(ret);
        }
    }};
}
pub(crate) use ok_if_fail;

macro_rules! as_c_string {
    ($target:expr) => {
        std::ffi::CString::new($target)
            .map_err(|_| SimConnectError::UnexpectedError("failed to create CString".to_string()))?
            .as_ptr()
    };
}
pub(crate) use as_c_string;
