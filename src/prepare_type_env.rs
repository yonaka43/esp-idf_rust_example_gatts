#[derive(Clone, Copy)]
pub struct PrepareTypeEnv {
    pub prepare_buf: *mut u8,
    pub prepare_len: u16,
}

impl Default for PrepareTypeEnv {
    fn default() -> Self {
        PrepareTypeEnv {
            prepare_buf: std::ptr::null::<()>() as *mut u8,
            prepare_len: 0,
        }
    }
}

impl PrepareTypeEnv {
    pub fn as_mut_ptr(self) -> *mut Self {
        Box::into_raw(Box::new(self)) as *mut PrepareTypeEnv
    }
}
