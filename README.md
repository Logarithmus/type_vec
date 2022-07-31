# typelist
Type-level singly linked list.

Because Rust lacks variadic generics, the list is implemented as a recursively nested tuple,
like this:
```rust
type List = ((((() A), B), C), D)
```

This is similar to `typenum::TArr`, but the representation is much shorter,
as it's just a single bracket instead of `TArr` for each element and also `()` instead of 

Available operations:
- [x] merge sort
- [x] minimum
- [x] maximum
- [x] 
- [x]
- [x]
