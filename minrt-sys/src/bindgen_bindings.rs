/* automatically generated by rust-bindgen 0.66.1 */

pub type wchar_t = ::core::ffi::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::core::ffi::c_longlong,
    pub __clang_max_align_nonce2: f64,
}
#[test]
fn bindgen_test_layout_max_align_t() {
    const UNINIT: ::core::mem::MaybeUninit<max_align_t> = ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<max_align_t>(),
        16usize,
        concat!("Size of: ", stringify!(max_align_t))
    );
    assert_eq!(
        ::core::mem::align_of::<max_align_t>(),
        8usize,
        concat!("Alignment of ", stringify!(max_align_t))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).__clang_max_align_nonce1) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce1)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).__clang_max_align_nonce2) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce2)
        )
    );
}
extern "C" {
    pub fn minrt_used_static_iwram() -> usize;
}
extern "C" {
    pub fn minrt_used_static_ewram() -> usize;
}
extern "C" {
    pub fn minrt_used_stack() -> usize;
}
extern "C" {
    pub fn minrt_stack_left() -> isize;
}
