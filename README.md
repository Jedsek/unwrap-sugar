# unwrap-sugar
This just unwraps all expression in the macro simply.  
The expressions must be unwrapable.

# Usage

```rust
unwrap_sugar! {
    // Immutable
    a = Some('a');

    // Mutable
    mut b = Some('b');

    // Immutable with Type
    c:char = {
        Some('C').map(|x| x.to_ascii_lowercase())
    };
};

assert_eq!(a, 'a');
assert_eq!(b, 'b');
assert_eq!(c, 'c');
```


