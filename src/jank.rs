
use core::marker::PhantomData;

pub struct JankIter< 'a, T > {
    pub ptr: *const T,
    pub end: *const T,
    pub _marker: PhantomData< &'a T >,
}

pub struct JankIterMut< 'a, T > {
    pub ptr: *mut T,
    pub end: *mut T,
    pub _marker: PhantomData< &'a mut T >,
}

