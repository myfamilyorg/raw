#![no_std]

pub trait AsRaw<T: ?Sized> {
    fn as_ptr(&self) -> *const T;
}

pub trait AsRawMut<T: ?Sized> {
    fn as_mut_ptr(&mut self) -> *mut T;
}
