use std::ffi::{CStr, CString};
use std::ptr;
use std::os::raw::{c_char, c_int};

// Define the sp_port struct as an opaque type
#[repr(C)]
pub struct sp_port {
    _private: [u8; 0],
}


#[repr(C)]
#[derive(Debug, PartialEq)]
// https://sigrok.org/api/libserialport/unstable/a00017.html#a8fa8ba0dd105754372ca82a6f391091e
pub enum SpReturn {
    SP_OK = 0,
    SP_ERR_ARG = -1,
    SP_ERR_FAIL = -2,
    SP_ERR_SUPP = -3,
}


// FFI bindings to libserialport functions
extern "C" {
    // int sp_list_ports(struct sp_port ***port_list);
    fn sp_list_ports(port_list: *mut *mut *mut sp_port) -> SpReturn;

    // const char *sp_get_port_name(const struct sp_port *port);
    fn sp_get_port_name(port: *const sp_port) -> *const c_char;

    // void sp_free_port_list(struct sp_port **port_list);
    fn sp_free_port_list(port_list: *mut *mut sp_port);
}