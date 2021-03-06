extern crate open_asset_importer as assimp;

use assimp::Importer;
use assimp::LogStream;
use std::ffi::CStr;
use std::os::raw::c_char;

unsafe extern "C" fn log_callback(msg: *const c_char, _userdata: *mut c_char) {
    let msg = CStr::from_ptr(msg);
    println!("Got log message {}", msg.to_str().unwrap());
}

#[test]
fn test_custom_logging() {
    LogStream::set_verbose_logging(true);
    let mut log_stream = LogStream::callback(log_callback);
    log_stream.attach();
    let importer = Importer::new();
    let _scene = importer.read_file("examples/box.obj");
}
