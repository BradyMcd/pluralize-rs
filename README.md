
# Pluralize-rs
Pluralize-rs aims to make generic programming in Rust capable of expressing structures which have a 
generically quantitied element. A type who has the bounds ```Pluralize<T>``` could either be a 
single primitive ```T```, a```Option<T>```, or a ```Vec<T>```. A single Pluralize-bound ```T``` can be 
iterated over by calling the ```pluralize( )``` method as can a Pluralize-bound ```Vec<T>``` or 
```Option<T>```.

The example given in the tests directory is quite reductive, so what is this actually useful for? 
Imagine you have a perfectly good singly linked list and you want a linked list with multiple link 
layers. You could just program one, using the singly linked list as a basic template or you could 
replace your link type (let's call it L) with a generic ```T: Pluralize<L>``` and replace your 
interactions with the link layer with iterators. A little bit of workshopping (to tag the links mostly)
and you've repurposed the code for a linked list into essentially a graph (without cycles).  
This is the use that I'm programming for, but maybe that will get someone else with more imagination 
than me off and running with a project either more insane or more practical than that.

## Limitations
The technique used to pluralize single primitives is only able to yield ::slice family iterators. You
can't to my knowledge adapt this to a more complex structure or iteration scheme, so no ambiguous 
tree-walking-or-primitive models without substantial work.

Currently the only way to remove from a ```Pluralize``` ```Vec<T>``` or ```Option<T>``` is locked behind
the "Remover" feature since ```Remover```s rely on casting from a mirrored type with the same layout as
```slice::IterMut```. This is not portable behaviour, realistically anything which effects memory layout
could make this code misbehave. There is a test in jank.rs to protect you from this, if it fails
something is broken and you shouldn't try to use Removers.

Currently the only way to implement ```Pluralize``` over ```Option<T>```' is locked behind the "Options"
feature since pulling a ```slice::Iter``` out of an ```Option<T>``` type requires the same mirrored
implementation ```Remover```s do. Again, tests to protect you from misbehaviour do exist and you should
heed them if they fail.
