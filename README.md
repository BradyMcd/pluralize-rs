
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

Pluralize currently also is very limited in how you can interact with the type behind the Pluralize
binding. If your concrete type is a ```Vec<T>``` for example you can't push values to it. That means
once you've fed in a vector it's pretty much set in stone as far as it's length is concerned, you still
have ```.pluralize_mut( )``` available to manipulate specific values, but you can't remove them.  
Realistically this is a TODO for me, it's just not clear to me what pushing or popping a primitive value
means or how the code should react to that.
