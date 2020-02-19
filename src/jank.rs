
use core::marker::PhantomData;

#[derive(PartialEq, Debug)]
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

#[cfg(test)]
mod tests {

    use crate::jank::{JankIter, JankIterMut};

    use core::mem::{transmute, size_of};
    use core::marker::PhantomData;

    #[test]
    fn test_jank( ) {
        assert_eq!( size_of::<JankIter<u8>>(), size_of::<core::slice::Iter<u8>>());
        assert_eq!( size_of::<JankIterMut<u8>>(), size_of::<core::slice::IterMut<u8>>());

        let buffer: Vec< usize > = vec!( 1,2,3,4 );
        let standard_iter = buffer.iter( );

        let len = buffer.len( );
        let ptr: *const usize = &buffer[0];
        let end = unsafe{ ptr.add( len ) };
        let mut jank_iter = JankIter {
            ptr: ptr,
            end: end,
            _marker: PhantomData,
        };

        for (jank, std) in unsafe{ transmute::<&mut JankIter<usize>,
                                               &mut core::slice::Iter<usize>>
                                   ( &mut jank_iter ) }.zip( standard_iter ) {
            assert_eq!( jank, std );
        }
    }
}
