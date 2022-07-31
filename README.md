## typelist
Type-level sortable singly linked list

### Motivation
The main purpose is to represent composite units in [`typeunits`](https://github.com/Logarithmus/typeunits)

Because Rust lacks variadic generics, the list is implemented as a recursively nested tuple.

This is similar to `typenum::TArr`, but `typelist` produces much shorter types in compilation errors:
```rust
type List1 = (((() Const<1>), Const<2>), Const<3>)
type List2 = TArr<TArr<TArr<ATerm, Const<1>>, Const<2>>, Const<3>>;
```

### Features
- [x] merge sort
- [x] minimum
- [x] maximum
- [x] concatenation
- [x] push
- [x] pop
- [ ] `typenum_list![..]` macro for `typenum_alias::Const<N>` list construction
      (TODO: fix reversed order)
