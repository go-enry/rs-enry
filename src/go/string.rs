use std::ffi::{CStr, CString};
use std::io::Read;
use std::os::raw::c_char;

#[repr(C)]
pub struct GoString {
    p: *const c_char,
    n: i64,
}

impl GoString {
    pub fn to_string(&self) -> String {
        let mut buf = vec![0u8; self.n as usize];
        unsafe {
            CStr::from_ptr(self.p)
                .to_bytes()
                .read_exact(&mut buf)
                .expect("Can't read from go string");
        }
        String::from_utf8(buf).unwrap()
    }
}

pub trait ToGoString {
    fn as_go_string(&self) -> GoString;
}

impl ToGoString for CString {
    fn as_go_string(&self) -> GoString {
        GoString {
            p: self.as_ptr(),
            n: self.as_bytes().len() as i64,
        }
    }
}
