# Rust Playground: data structure, algorithms, threads, etc.

This repository shows some examples of rust code for common coding
problems. As rust is different from other programming languages,
this is useful to have some patterns and code examples to translate
what we know in this new paradigm.

# How to use it?
```
git clone https://github.com/juli1/rust-playground.git
cd rust-playground
cargo run --bin animals
cargo run --bin main
cargo run --bin threads-unsafe
cargo run --bin threads-channel
cargo run --bin threads-rwlock
cargo test
```

# Rust concepts
This code illustrates the following rust concepts:
 * [Box](https://doc.rust-lang.org/std/boxed/struct.Box.html): in [src/trie.rs](src/trie.rs), [src/listnode.rs](src/listnode.rs), [src/tree.rs](src/tree.rs)
 * Threads: in [src/threads-unsafe.rs](src/threads-unsafe.rs) and
	[src/threads-rwlock.rs](src/threads-rwlock.rs)
 * [Rwlock](https://doc.rust-lang.org/std/sync/struct.RwLock.html): in
	[src/threads-rwlock.rs](src/threads-rwlock.rs)
 * [Iterator](https://rustbyexample.com/trait/iter.html): in
	[src/listnode.rs](src/listnode.rs)
 * Traits: in [src/animals.rs](src/animals.rs)


#  Code samples

### Traits
The file [src/animals.rs](src/animals.rs)
shows a very basic example of how to define traits
when defining a new structure Animals.


### Data Structure

#### LinkedList

The file [src/listnode.rs](src/listnode.rs) 
is a quick implementation of a linkedlist with rust.
The ListNode struct is used in [src/main.rs](src/main.rs) 

Note that this implementation also implements:
 * An iterator so that you can play like ```for v in node.iter() { println!("{}", n```
 * The Display trait to display a listnode and other next nodes so you can just
	do ```let n = ListNode::new(0); println!("treenode: {}", n);```

#### Binary Tree
The file [src/tree.rs](src/tree.rs) 
is a quick implementation of a binary tree with rust.
The TreeNode struct is used in [src/main.rs](src/main.rs) 

#### Trie
The file [src/trie.rs](src/trie.rs) is an implementation
of a trie in Rust. The TrieNode struct is used
in [src/main.rs](src/main.rs) to insert or search for words.

### Threads

#### Unsafe thread, communication through global variables
The file [src/threads-unsafe.rs](src/threads-unsafe.rs)
is a use of threads with unsafe operations (use of a global 
variable being modified).


#### Thread communication through channels
The file [src/threads-channel.rs](src/threads-channel.rs)
is a use of threads with channels.


#### Thread communication through using RwLock
The file [src/threads-rwlock.rs](src/threads-rwlock.rs)
is an example of use of rwlock between multiple readers
and one writer.


#### Thread with conditional variable
The file [src/threads-condvar.rs](src/threads-condvar.rs) shows
how to use a conditional variable with Rust. It spins 6 threads
and synchronize their start using a conditional variable.

# Crates being used/tested

## Mime guess
Try to guess the [MIME type](https://en.wikipedia.org/wiki/MIME) of a file.
You can see the file there: [src/mime-guess.rs](src/mime-guess.rs).

# Useful resources
 * [Little tour of multiple iterators](https://blog.guillaume-gomez.fr/articles/2017-03-09+Little+tour+of+multiple+iterators+implementation+in+Rust)
 * [Basic Trie Implementation](https://users.rust-lang.org/t/implementing-a-very-basic-trie/10788/2)
