use std::raw;
use std::mem;
extern crate alloc;

use std::ffi::CString;
use std::slice;
use std::ptr;

#[allow(unused)]
unsafe fn raw_byte_repr<'a, T>(ptr: &'a T) -> &'a [u8] {
    mem::transmute(raw::Slice{
        data: ptr as *const _ as *const u8,
        len: mem::size_of::<T>(),
    })
}

//~ pub fn str_to_ref(mystr:&str) -> *const i8 {unsafe{
    //~ let cstr = CString::from_slice(mystr.as_bytes());
    //~ ptr::read(&cstr.as_slice_with_nul().as_ptr())
//~ }}

pub fn str_to_ref(line: &str) -> *const i8 {
    let l = line.as_bytes();
    unsafe {
        //alignment, whats that?
        let b = alloc::heap::allocate(line.len()+1, 8);
        let s = slice::from_raw_parts_mut(b, line.len()+1);
        slice::bytes::copy_memory(s, l);
        s[line.len()] = 0;
        return b as *const i8;
    }
}

