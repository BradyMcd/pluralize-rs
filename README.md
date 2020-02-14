
# Pluralize-rs
Pluralize-rs aims to make generic programming in Rust capable of expressing structures which have a 
generically quantitied element. A type who has the bounds ```Pluralize<T>``` could either be a 
single primitive ```T``` or a ```Vec<T>```. A single Pluralize-bound ```T``` can be iterated over
by calling the ```pluralize( )``` method as can a Pluralize-bound ```Vec<T>```.

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

Two Iterator types have been added as a means of manipulating the underlying collection. Adder and 
Remover are both iterators which pass out an ```Rc``` to a controller type. Adder can ```.push( )```
into a pluralized ```Vec<T>``` while Remover is a mutable iterator which can delete items in the
collection as it iterates over them. Be wary of Remover, it's a very hacky implementation and will
panic if asked to remove a value when the type underlying the ```Pluralize``` trait is a primitive.
This panic will eventually be avoidable at the cost of probably more jank being added implementing
the trait over the Option type.
