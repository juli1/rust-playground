# rust-playground

This repository shows some example of rust code for common coding
problems. As rust is different from other programming languages,
this is useful to have some patterns and code examples to translate
what we know in this new paradigm.


## Traits
The file [src/animals.rs](src/animals.rs)
shows a very basic example of how to define traits
when defining a new structure Animals.

## Tree
The file [src/tree.rs](src/tree.rs) 
is a quick implementation of a binary tree with rust.

## Threads

### Unsafe thread, communication through global variables
The file [src/threads-unsafe.rs](src/threads-unsafe.rs)
is a use of threads with unsafe operations (use of a global 
variable being modified).


### Unsafe thread, communication through channels
The file [src/threads-unsafe.rs](src/threads-channel.rs)
is a use of threads with channels.
