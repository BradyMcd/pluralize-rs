/*!
 * The ```Pluralize``` trait exists to offer a single generic trait which can yield an iterator from any
   reference. This allows generic code to be implemented where the plurality of the generic type is
   flexible. This is accomplished by casting the reference of any single primitive into a single
   element array of the same type and calling the appropriate ```.iter()``` function.

 * In simplest terms if you specify that a generic type has the bounds ```Pluralize< T >``` then that
   type could be a plain old ```T``` or a ```Vec<T>```. In order to make use of this simply call the
   ```.puralize( )``` method and iterate in a for loop.

 * ## Features

 * This crate is fully compatible with ```#![no_std]``` projects, just include a
   ```default-features=false``` directive along with the dependency information in your ```Cargo.toml```

 * ## Limitations

 * This approach does have some limitations you should be aware of.   
   More complex collections which don't use the ```std::slice::``` family of iterators aren't supported.   
   Currently, after the creation of a vector behind a Pluralize binding there is no way to grow that
   vector, it can only be modified using ```.pluralize_mut( )``` which isn't capable of doing anything
   other than modifying preexisting elements.
 */

#![cfg_attr(not(feature="std"), no_std)]

#[cfg(not(feature="std"))]
extern crate alloc;
#[cfg(not(feature="std"))]
use alloc::vec::Vec;
#[cfg(not(feature="std"))]
use core::slice::{Iter, IterMut};
#[cfg(feature="std")]
use std::slice::{Iter, IterMut};


/* NOTE/HACK:
 * I'm on the fence about implementing a pluralize for things like functions and raw pointers

 * I originally tried to do this with specializations, I'm going to just consider that an open problem.
   As that RFC stabilizes especially regarding default types I'll take another crack.
 * The main issue was that default types are opaque to default functions
 */

/// A trait implemented across both collections and single primitives which exposes an iterator
pub trait Pluralize< T > {
    fn pluralize<'a>( &'a self ) -> Iter<'a, T>;
    fn pluralize_mut<'a>( &'a mut self ) -> IterMut<'a, T>;
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
                    unsafe{core::mem::transmute::<&'a($($T,)+), &'a [($($T,)+);1]>(self)}.iter( )
                }
                #[inline(always)]
                fn pluralize_mut<'a>( &'a mut self ) -> IterMut<'a, ($($T,)+)> {
                    unsafe{core::mem::transmute::<&'a mut($($T,)+), &'a mut[($($T,)+);1]>(self)}
                    .iter_mut( )
                }
            }
        )+
    }
}

//Should make an equivelent proc_macro, #[derive(Pluralize)] would take care of the import gore
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
