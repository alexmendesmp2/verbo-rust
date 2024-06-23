use std::ffi::{CString, CStr};
use std::net::TcpStream;
use std::io::{Read, Write};
use std::os::raw::{c_char, c_int};

extern crate oracle;
use oracle::{Connection};

#[no_mangle]
pub extern "C" fn call_hsm(
    module:         *const c_char,
    exponent:       *const c_char,
    public_key:     *mut c_char,
    private_key:    *mut c_char/*,
    return_code:    *mut c_int,
    return_reason:  *mut c_int*/
) -> *mut c_char {
    //let param_entrada: String = unsafe { CStr::from_ptr(param) }.to_string_lossy().into_owned();
    //println!("Call HSM! - {}", param_entrada);
    let module_in: String = unsafe { CStr::from_ptr(module) }.to_string_lossy().into_owned();
    let exponent_in: String = unsafe { CStr::from_ptr(exponent) }.to_string_lossy().into_owned();
    let public_key_in: String = unsafe { CStr::from_ptr(public_key) }.to_string_lossy().into_owned();
    let private_key_in: String = unsafe { CStr::from_ptr(private_key) }.to_string_lossy().into_owned();
    
    println!("module!           - {:?}", module_in);
    println!("exponent!         - {:?}", exponent_in);
    println!("public_key!       - {:?}", public_key_in);
    println!("private_key!      - {:?}", private_key_in);
    //println!("return_code!      - {:?}", return_code);
    //println!("return_reason!    - {:?}", return_reason);

    CString::new("A87B87DF7544CD").unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn save_to_oracle(label: *const c_char, key: *const c_char) {
    println!("Call save_to_oracle");
    println!("Label salva no Oracle - {}", unsafe { CStr::from_ptr(label) }.to_string_lossy().into_owned());
    println!("Chave salva no Oracle - {}", unsafe { CStr::from_ptr(key) }.to_string_lossy().into_owned());
}
