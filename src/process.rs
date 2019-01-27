use watchexec::pathop::PathOp;
use std::ffi::CString;
use chrono::prelude::DateTime;
use chrono::{Utc};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[repr(C)]
pub struct PathOp_C {
    pub path: *const libc::c_char,
    pub op: *const libc::c_char,
    pub cookie: libc::uint32_t,
    pub time: libc::int64_t,
    pub time_micro: libc::uint32_t
}

#[repr(C)]
pub struct DynArray {
    length: libc::size_t,
    capacity: libc::size_t,
    array: *mut PathOp_C,
}

#[derive(Clone)]
pub struct Callback{
    cb: extern fn(DynArray)
}

impl Callback{
    pub fn exec_callback(&mut self, paths: Vec<PathOp>){
        let mut paths_c:  Vec<PathOp_C> = pathop_to_pathop_c(paths);
        let result = DynArray {
            array: paths_c.as_mut_ptr(),
            length: paths_c.len() as _,
            capacity: paths_c.capacity() as _,
        };

        std::mem::forget(paths_c);
        (self.cb)(result);
    }
}

fn string_from_rust(str: String) -> *const libc::c_char {
    let s = CString::new(str).unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    p
}

fn pathop_to_pathop_c(paths: Vec<PathOp>) -> Vec<PathOp_C>{
    let mut c_paths: Vec<PathOp_C> = vec![];
    for p in paths{
        let _path = p.path;
        let _op = p.op;
        let _cookie = p.cookie;
        let _time = p.time;
        let path = _path.to_path_buf().into_os_string().into_string();
        let key = if _op.is_some(){
            let op = _op.unwrap();
            match op {
                op if PathOp::is_create(op) => "Create",
                op if PathOp::is_remove(op) => "Remove",
                op if PathOp::is_rename(op) => "Rename",
                op if PathOp::is_write(op) => "Write",
                op if PathOp::is_meta(op) => "Meta",
                _ => "None"
            }
        }else{
            "None"
        };

        let datetime = DateTime::<Utc>::from(_time);
        let unix_datetime = datetime.timestamp();
        let unix_micro_datetime = datetime.timestamp_subsec_micros();
        // Formats the combined date and time with the specified format string.
        //let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();

        let cookie = if _cookie.is_some(){
            _cookie.unwrap()
        }else{
            u32::max_value()
        }; 

        std::mem::forget(&cookie);
        c_paths.push(PathOp_C {
            path: string_from_rust(path.unwrap()),
            op: string_from_rust(key.to_owned()),
            cookie: cookie,
            time: unix_datetime,
            time_micro: unix_micro_datetime
        });
    }
    c_paths
}

pub fn create_callback(cb_c: extern fn(DynArray)) -> Callback{
    Callback{cb: cb_c}
}