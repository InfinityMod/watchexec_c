use std::ffi::{
	CStr
};

use libc::{
    c_char
};

use watchexec::pathop::PathOp;

use watchexec::run::{
    run
};
use watchexec::cli::{
    process_args,
    init_app
};

use process::{
    Callback,
    DynArray,
    create_callback,
};

#[no_mangle]
pub extern "C" fn run_watcher(c_args: *const c_char, cb_c: extern fn(DynArray)){
    let cb: Callback = create_callback(cb_c);
    let args = init_app().get_matches_from(unsafe {CStr::from_ptr(c_args).to_str().unwrap().split(" ")});

    fn run_cb(mut cb: Callback, paths: Vec<PathOp>){
        cb.exec_callback(paths);
    }

    let _result = run(process_args(args), Some(move |paths| run_cb(cb.clone(), paths)));
}
