# Tree
A small CLI utility that prints a visual representation of a directory and all its subdirectories.

# Usage
`tree <directory>`

```
$ ./tree ./tree
./tree/
|-- tree
|-- Cargo.toml
|-- Cargo.lock
|-- README.md
|-- .gitignore
|-- tst
|	|-- test-symlink -> ./src/main.rs
|	|-- foo
|	|	|-- bar
|	|	|	|-- baz
|-- src
|	|-- main.rs

```