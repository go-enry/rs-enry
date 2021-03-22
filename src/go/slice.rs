use core::convert::From;
use core::default::Default;
use core::ffi::c_void;
use core::ptr::null_mut;
use std::ffi::CString;
use std::os::raw::c_char;

#[repr(C)]
pub struct GoSlice {
    data: *mut c_void,
    len: i64,
    cap: i64,
}

impl Default for GoSlice {
    fn default() -> Self {
        Self {
            data: null_mut(),
            len: 0,
            cap: 0,
        }
    }
}

pub trait ToGoSlice {
    fn as_go_slice(&self) -> GoSlice;
}

impl ToGoSlice for CString {
    fn as_go_slice(&self) -> GoSlice {
        GoSlice {
            data: self.as_ptr() as *mut c_void,
            len: self.as_bytes().len() as i64,
            cap: self.as_bytes().len() as i64,
        }
    }
}

impl From<GoSlice> for Vec<String> {
    fn from(s: GoSlice) -> Self {
       unsafe {
           let mut result = Vec::with_capacity(s.cap as usize);
           let strs = s.data as *mut *mut c_char;

           for i in 0..s.len {
               result.push(
                   CString::from_raw(*strs.offset(i as isize))
                       .into_string()
                       .unwrap()
               )
           }

           result
       }
    }
}
