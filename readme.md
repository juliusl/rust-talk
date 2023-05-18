# Talk about abstractions in Rust

The topic of this talk covers the re-imagining of a "Store" interface written in C# into Rust's abstraction semantics.

The code in this repo has examples in various states of "imagining".

**Examples**:

Examples are seperated by branches in various conditions,

- level_1        - Coupled specific implementation to abstraction
- level_2        - Seperated implementations but coupled abstractions
- level_3        - Decoupled abstractions
- level_3-derive - Decoupled abstractions and initial derive implementation

**Final**

Decoupled abstractions and deriving Store trait.