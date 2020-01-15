
use pluralize::{ Pluralize };

// An incredibly simplistic example
#[derive(Debug)]
struct MaybeMany< T: Pluralize< usize > > {
    data: T,
}

impl<T> MaybeMany<T>
where T: Pluralize< usize >
{
    fn new( data: T ) -> Self {
        MaybeMany {
            data: data
        }
    }
}

impl<T> PartialEq for MaybeMany<T>
where T: Pluralize< usize >
{
    fn eq( &self, other: &Self ) -> bool {
        self.data.pluralize( ).eq( other.data.pluralize( ) )
    }
}

#[test]
fn test_eq( ) {
    let single_a = MaybeMany::new( 42 );
    let single_b = MaybeMany::new( 1 );
    let many_a = MaybeMany::new( vec![20, 22] );
    let many_b = MaybeMany::new( vec![1] );

    assert_ne!( single_a, single_b );
    assert_eq!( single_a, MaybeMany::new( 42 ) );
    assert_ne!( many_a, many_b );
    assert_eq!( many_a, MaybeMany::new( vec![20, 22] ) );
    // assert_eq!( many_b, single_b ); This doesn't work thanks to the typechecker

}
