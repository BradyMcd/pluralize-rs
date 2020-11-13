/*!
The ```Pluralize``` trait exists to offer a single generic trait which can yield an iterator from any
reference. This allows generic code to be implemented where the plurality of the generic type is
flexible. This is accomplished by casting the reference of any single primitive into a single
element array of the same type and calling the appropriate ```.iter()``` function.

In simplest terms if you specify that a generic type has the bounds ```Pluralize< T >``` then that
type could be a plain old ```T``` or a ```Vec<T>```. In order to make use of this simply call the
```.puralize( )``` method and iterate in a for loop.

## Limitations

Currently implementation of Pluralize over ```Option<T>``` is locked behind a feature: Options.  
Currently implementation of the Remover Iterator is locked behind a feature: Remover.

Both of these features are locked because they represent very janky code which could be broken
by anything which effects memory layout.
 */

extern crate cfg_if;
use cfg_if::cfg_if;

extern crate alloc;
use core::ops::{ Deref, DerefMut };
use alloc::vec::Vec;
use core::slice::{Iter, IterMut};
use core::mem::transmute;

pub mod iter;
pub use iter::{ Adder, AddController };

cfg_if!{ if #[cfg( any( feature = "Remover", feature = "Options" ) )] {
pub mod jank;
}}
cfg_if!{ if #[cfg( feature = "Remover")] {
pub use iter::{ Remover, RemoveController };
}
else if #[cfg( feature = "Options" )] {
use jank::{ JankIter, JankIterMut };
use core::marker::PhantomData;
}}

/// A trait implemented across both collections and single primitives which exposes an iterator
pub trait Pluralize< T > {
    fn pluralize<'a>( &'a self ) -> Iter<'a, T>;
    fn pluralize_mut<'a>( &'a mut self ) -> IterMut<'a, T>;
}

/// A trait enabling further mutations to Pluralize<> objects through two Controller-Iterator objects
pub trait PluralizeControlIter<T, P: Pluralize< T >> {
    fn adder<'a>( &'a mut self ) -> Adder< 'a, T, P >;
    cfg_if!{ if #[cfg( feature = "Remover" )] {
    fn remover<'p, 'a:'p>( &'a mut self ) -> Remover< 'p, 'a, T, P >;
    }}
}

impl< T, P > PluralizeControlIter<T, P> for P
where T: Pluralize< T >,
      P: Pluralize< T >
{
    #[inline(always)]
    fn adder<'a>( &'a mut self ) -> Adder<'a, T, P> {
        Adder::new( self )
    }
    cfg_if!{ if #[cfg(feature = "Remover")] {
    fn remover<'p, 'a: 'p>( &'p mut self ) -> Remover<'p, 'a, T, P> {
        Remover::new( self )
    }
    }}
}

impl< T > Pluralize< T > for Vec<T>
where T: Pluralize< T > /*If T doesn't also Pluralize over T then we aren't using this as a generic,
    we're just making a complicated call to .iter()*/
{
    #[inline(always)]
    fn pluralize<'a>( &'a self ) -> Iter<'a, T> {
        self.iter()
    }

    #[inline(always)]
    fn pluralize_mut<'a>( &'a mut self ) -> IterMut<'a, T> {
        self.iter_mut()
    }
}

impl<T> Pluralize< T > for Box<T>
where T: Pluralize< T >
{
    fn pluralize<'a>( &'a self ) -> Iter<'a, T> {
        self.deref().pluralize()
    }

    fn pluralize_mut<'a>( &'a mut self ) -> IterMut<'a, T> {
        self.deref_mut().pluralize_mut()
    }
}

cfg_if!{ if #[cfg( feature = "Options")]{
impl< T > Pluralize< T > for Option< T >
where T: Pluralize< T > {
    #[inline(always)]
    fn pluralize<'a>( &'a self ) -> Iter<'a, T> {
        if self.is_none( ) {
            // Maybe we should switch this. I have a feeling we could easily seg fault by calling
            // as_slice
            let ptr = core::ptr::null( );
            let end = ptr;
            unsafe {
                transmute::< JankIter<'a, T>, Iter<'a, T> > (
                    JankIter {
                        ptr: ptr,
                        end: end,
                        _marker: PhantomData,
                    }
                )
            }
        } else {
            self.as_ref( )
                .unwrap( )
                .pluralize( )
        }
    }

    #[inline(always)]
    fn pluralize_mut<'a>( &'a mut self ) -> IterMut<'a, T> {
        if self.is_none( ) {
            let ptr = core::ptr::null_mut( );
            let end = ptr;
            unsafe{
                transmute::<JankIterMut<'a, T>, IterMut<'a, T>> (
                    JankIterMut {
                        ptr: ptr,
                        end: end,
                        _marker: PhantomData
                    }
                )
            }
        } else {
            self.as_mut( )
                .unwrap( )
                .pluralize_mut( )
        }
    }
}
}}

macro_rules! impl_tuple_pluralize {
    ($(
        $Tuple:ident {
            $($T:ident),+
        }
    )+) => {
        $(
            impl < $($T,)+ > Pluralize<($($T,)+)>
                for ($($T,)+)
            {
                #[inline(always)]
                fn pluralize<'a>( &'a self ) -> Iter<'a, ($($T,)+)> {
                    unsafe{transmute::<&'a($($T,)+), &'a [($($T,)+);1]>(self)}.iter( )
                }
                #[inline(always)]
                fn pluralize_mut<'a>( &'a mut self ) -> IterMut<'a, ($($T,)+)> {
                    unsafe{transmute::<&'a mut($($T,)+), &'a mut[($($T,)+);1]>(self)}
                    .iter_mut( )
                }
            }
        )+
    }
}

//Should make an equivelent proc_macro, #[derive(Pluralize)] would take care of import gore
#[macro_export]
macro_rules! impl_primitive_pluralize {
    ( $($t:ty), + ) => {
        $(
            impl Pluralize<$t> for $t {
                #[inline(always)]
                fn pluralize<'a>( &'a self ) -> Iter<'a, $t> {
                    unsafe{ core::mem::transmute::<&'a $t, &'a [$t;1]>(self)}.iter( )
                }

                #[inline(always)]
                fn pluralize_mut<'a>( &'a mut self ) -> IterMut<'a, $t> {
                    unsafe{ core::mem::transmute::<&'a mut $t, &'a mut[$t;1]>(self)}.iter_mut( )
                }
            }
        )+
    }
}

impl_primitive_pluralize!( i8, i16, i32, i64, i128, isize );
impl_primitive_pluralize!( u8, u16, u32, u64, u128, usize );
impl_primitive_pluralize!( bool, char, f32, f64 );

impl_tuple_pluralize!{
    Tuple1{
        A
    }
    Tuple2{
        A,
        B
    }
    Tuple3{
        A,
        B,
        C
    }
    Tuple4{
        A,
        B,
        C,
        D
    }
    Tuple5{
        A,
        B,
        C,
        D,
        E
    }
    Tuple6{
        A,
        B,
        C,
        D,
        E,
        F
    }
    Tuple7{
        A,
        B,
        C,
        D,
        E,
        F,
        G
    }
    Tuple8{
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H
    }
    Tuple9{
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I
    }
    Tuple10{
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J
    }
    Tuple11{
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
        K
    }
    Tuple12{
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
        K,
        L
    }
}

#[cfg(test)]
mod tests {
    use core::mem::transmute;

    #[test]
    /// Key assumptions made in the design of the Pluralize trait.
    fn assumption( ) {
        let primitive:usize = 5;
        let scarequotes_slice:&[usize;1] = unsafe{ transmute( &primitive ) };

        // A &usize looks the same as a &[usize;1] in terms of memory layout
        assert_eq!( primitive, scarequotes_slice[0] );
    }

}
