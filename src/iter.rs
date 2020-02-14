//

use crate::Pluralize;

use std::rc::Rc;

use core::mem::transmute;
use core::cell::Cell;
use core::convert::TryInto;
use core::marker::PhantomData;
use core::slice::IterMut;

/// The structure allowing us to communicate Additions to a Pluralized type through the Adder iterator.
pub struct AddController< T > {
    cell: Cell< Option< T > >
}

impl< T > AddController< T >
{
    /// Signal that the ```Adder``` should ```push( )``` the given value into the underlying collection.
    pub fn add( &self, d: T ) {
        self.cell.set( Some( d ) );
    }

    /// Returns the controller to its default value. If the controller is at its default value at the
    /// end of an iteration it will end the loop.
    ///
    /// Calling this without first calling ```.add( )``` is superfluous.
    pub fn clear( &self ) {
        self.cell.set( None );
    }

    fn _replace( &self, d: Option< T > ) -> Option< T > {
        self.cell.replace( d )
    }

    fn new( ) -> Rc< Self > {
        Rc::new(
            AddController {
                cell: Cell::new( None ),
            }
        )
    }

}

// TODO: An Option< T > implementation for Pluralize and the Adder. A Some() variant has Adder return
// None while a None variant allows one to add a value. A good way to have primitives act near exactly
// the same way as a Vec.
/// An Iterator which progressively adds to a collection behind a Pluralize trait. Vectors are added to
/// in stack order using the ```push()``` method while Primitives simply return None.
///
/// To add to a vector simply call ```.add( )``` on the returned controller, the supplied value will be
/// pushed to the vector at the end of the current iteration.
///
/// To end iteration either take no action or ```.clear( )``` the controller if a value was supplied
/// within the last block of iteration.
pub struct Adder< 'i, T, P: Pluralize< T > > {
    collection: &'i mut P,
    controller: Rc< AddController< T > >,
    first_run: bool,
}

impl< 'b, T: Pluralize< T > > Iterator for Adder< 'b, T, Vec< T > >
{
    type Item = Rc< AddController< T > >;

    fn next( &mut self ) -> Option< Self::Item > {
        if !self.first_run {
            let directive = self.controller._replace( None );
            if directive.is_none( ) {
                return None;
            } else {
                self.collection.push( directive.unwrap( ) );
            }

        } else {
            self.first_run = false;
        }
        Some( Rc::clone( &self.controller ) )
    }
}

impl < 'b, T, P: Pluralize< T > > Adder< 'b, T, P >
where T: Pluralize< T >
{

    /// Prefer the ```.adder( )``` method provided by the Pluralize trait
    pub fn new( collection: &'b mut P ) -> Self {
        Adder{
            collection: collection,
            controller: AddController::new( ),
            first_run: true,
        }
    }
}

impl< 'b, T: Pluralize< T > > Iterator for Adder< 'b, T, T > {
    type Item = Rc< AddController< T > >;

    fn next( &mut self ) -> Option< Self::Item > {
        None
    }
}

#[derive( PartialEq )]
enum RemoveCmd {
    Remove,
    Pass,
}

/// The structure allowing us to communicate removals from a Pluralized type through the Remover iterator
pub struct RemoveController {
    cell: Cell< RemoveCmd >,
}

impl RemoveController {

    /// Flag the element returned with this controller for removal. Removals occur at the end of the
    /// current iteration, before the next element is returned.
    pub fn remove( &self ) {
        self.cell.set( RemoveCmd::Remove );
    }

    /// Clear the removal flag for the element returned with this controller. This is the initial value
    /// of the ```RemoveController```; if you took no action to mark an element for removal this call
    /// is superfluous.
    pub fn pass( &self ) {
        self.cell.set( RemoveCmd::Pass );
    }

    fn _replace( &self, cmd: RemoveCmd ) -> RemoveCmd {
        self.cell.replace( cmd )
    }

    fn new( ) -> Rc< Self > {
        Rc::new(
            RemoveController {
                cell: Cell::new( RemoveCmd::Pass ),
            }
        )
    }
}

/// An iterator over a Pluralize<T> which can modify and remove items from an underlying vector or
/// modify an underlying primitive.
pub struct Remover< 'p: 'a, 'a, T, P: Pluralize< T > > {
    collection: &'p mut P,
    ptr: *mut T,
    end: *mut T,
    controller: Rc< RemoveController >,
    _marker: PhantomData< &'a T >,
}

// This particular jank brought to you by a need to unify Remover::new( ) into a single function for
// trait to work properly. NOTE: the jank might make it particularly hard to make this code portable
// and extend the implementation to cover the Option<> type. 
struct JankIterMut< 'a, T > {
    ptr: *mut T,
    end: *mut T,
    _marker: PhantomData< &'a mut T >,
}
impl< 'p: 'a, 'a, T: Pluralize< T >, P: Pluralize< T > > Remover< 'p, 'a, T, P > {
    pub fn new( plural: &'p mut P ) -> Self {
        let len;
        let ( ptr, end ): ( *mut T, *mut T );
        unsafe {
            let iter = plural.pluralize_mut( );
            let hack = transmute::< IterMut< '_, T >, JankIterMut< '_, T > >( iter );
            len = hack.end.offset_from( hack.ptr );
        }

        ptr = unsafe{ transmute::< &mut P, &mut T >( plural ) };
        end = unsafe{ ptr.offset( len ) };

        Remover {
            collection: plural,
            ptr: ptr,
            end: end,
            controller: RemoveController::new( ),
            _marker: PhantomData,
        }
    }
}

impl< 'p, 'a: 'p, T: Pluralize< T > > Iterator for Remover< 'p, 'a, T, T > {
    type Item = ( Rc< RemoveController >, &'a mut T );
    fn next( &mut self ) -> Option< Self::Item > {

        if self.controller._replace( RemoveCmd::Pass ) == RemoveCmd::Remove {
            unimplemented!( "Removals not supported over primitives" );
        }

        if self.ptr == self.end {
            return None;
        }

        let old = self.ptr;
        unsafe{ self.ptr = self.ptr.offset( 1 ); }
        Some( ( Rc::clone( &self.controller ),
                unsafe{ old.as_mut( ).unwrap( ) } ) )
    }

}

impl< 'p: 'a, 'a, T: Pluralize< T > > Iterator for Remover< 'p, 'a, T, Vec< T > > {
    type Item = ( Rc< RemoveController >, &'a mut T );
    fn next( &mut self ) -> Option< Self::Item > {
        match self.controller._replace( RemoveCmd::Pass ) {
            RemoveCmd::Pass => { }
            RemoveCmd::Remove => {
                let idx = unsafe{
                    self.ptr.offset_from( &self.collection[0] as *const T ) - 1
                };
                self.collection.remove( idx.try_into( ).unwrap( ) );
                unsafe {
                    self.end.sub( 1 );
                    self.ptr.sub( 1 );
                }
            }
        }

        if self.ptr == self.end {
            return None;
        }

        let old = self.ptr;
        unsafe { self.ptr = self.ptr.offset( 1 ); }
        Some( ( Rc::clone( &self.controller ),
                unsafe{ old.as_mut( ).unwrap( ) }
        ) )
    }
}

#[cfg(test)]
mod tests {
    use core::mem::transmute;

    #[test]
    /// Key assumptions made in the design of the iterators.
    ///
    /// Nothing too groundbreaking, but good to keep in mind if you're reading the code.
    fn assumption( ) {
        let collection = vec!( 1,2,3,4,5 );
        let ptr: *const usize = &collection[0];
        // The location of the Vec is the same as it's 0th element
        assert_eq!( ptr, unsafe{
            transmute::< &Vec<usize>, *const usize >( &collection )
        } );
    }
}
