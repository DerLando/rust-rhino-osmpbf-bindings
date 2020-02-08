use osmpbfreader::{
    OsmPbfReader,
};
use std::ffi::CStr;
use libc::c_char;

#[repr(C)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

#[no_mangle]
pub extern fn get_test_point() -> Point {
    Point {
        x: 2.0,
        y: -6.346757
    }
}

#[no_mangle]
pub extern fn read_pbf(s: *const c_char) -> usize {
    let c_str = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };

    let r_str = c_str.to_str().unwrap();

    let file = std::fs::File::open(r_str);
    match file {
        Err(e) => panic!("{:?}", e),
        Ok(f) => {
            let mut pbf = OsmPbfReader::new(f);
            let mut nb = 0;
            for _obj in pbf.iter().map(Result::unwrap) {
                nb += 1;
            }
            nb
        }
    }
}
