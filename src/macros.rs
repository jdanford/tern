#[macro_export]
macro_rules! ptr {
    ($e:expr) => (&$e[0] as *const _)
}

#[macro_export]
macro_rules! mut_ptr {
    ($e:expr) => (&mut $e[0] as *mut _)
}

#[macro_export]
macro_rules! tryte_offset {
    ($e:expr,$n:expr) => ($e.offset(TRYTE_ISIZE * $n))
}

#[macro_export]
macro_rules! tryte_ptr {
    ($e:expr,$n:expr) => (tryte_offset!(ptr!($e), $n))
}
