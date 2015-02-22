extern crate alloc;
use std::raw;
use std::mem;
use std::slice;

use std::ffi::CString;

#[allow(unused)]
unsafe fn raw_byte_repr<'a, T>(ptr: &'a T) -> &'a [u8] {
    mem::transmute(raw::Slice{
        data: ptr as *const _ as *const u8,
        len: mem::size_of::<T>(),
    })
}

//~ pub fn str_to_ref(mystr:&str) -> *const i8 {
    //~ let s = CString::new(mystr).unwrap();
    //~ s.as_ptr() // s is still alive here }
    
    //~ let cstr = CString::new(mystr.as_bytes()).unwrap();
    //~ cstr.as_bytes().as_ptr() as *const i8
//~ }

//~ pub fn str_to_ref(mystr: &str) -> *const i8 {
    //~ let l = mystr.as_bytes();
    //~ unsafe {
        //~ let b = alloc::heap::allocate(mystr.len()+1, 8);
        //~ let s = slice::from_raw_parts_mut(b, mystr.len()+1);
        //~ slice::bytes::copy_memory(s, l);
        //~ s[mystr.len()] = 0;
        //~ return b as *const i8;
    //~ }
//~ }

