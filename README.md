Brainfuck Interpreter
=====================

A simple [brainfuck](https://en.wikipedia.org/wiki/Brainfuck) interpreter written in [Rust](https://www.rust-lang.org/).

### Implementation Notes

* Characters that don't correspond to instructions are treated as comments
* Square brackets that don't match are treated as a syntax error

### Implementation Constraints

The following outlines the implementation-specific behaviour that occurs when relevant conditions are 
reached. See [The Unofficial Constraints on Portable Brainfuck Implementations](https://www.muppetlabs.com/~breadbox/bf/standards.html) 
for more information.

This documents the current state of the interpreter and will be subject to change. The constraints given 
here are quite strict and only allow non-wraparound programs to be run.

* Size of cell array: Array size is currently 30, 000
* Moving pointer beyond the cell array results in a panic
* The range of values a single cell may contain is that of an unsigned 8-bit integer
* Attempting to increment or decrement the value of a cell beyond constraints results in a panic
* Data is input using stdin. Attempting to input a value without data in the input stream results in 
the value stored being zero
* Running a program that contains unbalanced brackets results in a panic

### Roadmap:

1. Working interpreter
    - [x] Instructions modify program state correctly
    - [x] Sequences/while loops implemented correctly
    - [] Cell array is of variable size
    - [] Cell array wraps around
    - [] Custom error types for underflow/overflow
2. Working compiler