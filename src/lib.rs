use std::ffi::{CString, CStr};
use std::net::TcpStream;
use std::io::{Read, Write};
use std::os::raw::{c_char, c_int};

extern crate oracle;
use oracle::{Connection};

#[no_mangle]
pub extern "C" fn call_hsm(param: *const c_char) -> *mut c_char {
    let param_entrada: String = unsafe { CStr::from_ptr(param) }.to_string_lossy().into_owned();
    println!("Call HSM! - {}", param_entrada);
    CString::new("A87B87DF7544CD").unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn save_to_oracle(param: *const c_char) {
    println!("Call save_to_oracle");
    println!("Chave salva no Oracle - {}", unsafe { CStr::from_ptr(param) }.to_string_lossy().into_owned());
}
