use std::ffi::c_void;

use windows_sys::Win32::Foundation::CloseHandle;
use windows_sys::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

/// Usually result is correct, but some cases (e.g. "runas /trustlevel:0x20000 $command") will
/// still produce positive results.
pub(crate) fn is_elevated() -> bool {
    let mut elevated = 0_u32;

    unsafe {
        let mut handle = 0_isize;

        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut handle) != 0 {
            let len = std::mem::size_of::<TOKEN_ELEVATION>();
            let mut buf = vec![0_u8; len];
            let buf_ptr = buf.as_mut_ptr();
            let mut ret_len = 0;

            if GetTokenInformation(handle, TokenElevation, buf_ptr as *mut c_void, len as u32, &mut ret_len) != 0 {
                // let te = *std::mem::transmute::<*mut u8, *mut TOKEN_ELEVATION>(buf_ptr);
                let te = std::ptr::read(buf_ptr as *mut TOKEN_ELEVATION);
                elevated = te.TokenIsElevated;
            }
        }

        if CloseHandle(handle) == 0 {
            // TODO error handling
            panic!("an unexpected error has occurred")
        }
    }

    elevated > 0
}
