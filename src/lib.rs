//! # Usage  
//! This just unwraps all expression in the macro simply.  
//! The expressions must be unwrapable.  
//!
//! ```rust
//! unwrap_sugar! {
//!     // Immutable
//!     a = Some('a');
//!
//!     // Mutable
//!     mut b = Some('b');
//!
//!     // Immutable with Type
//!     c:char = {
//!         Some('C').map(|x| x.to_ascii_lowercase())
//!     };
//! };
//!
//! assert_eq!(a, 'a');
//! assert_eq!(b, 'b');
//! assert_eq!(c, 'c');
//! ```
//!
#[macro_export]
macro_rules! unwrap_sugar {
    ($id:ident$(:$t:ty)? = $e:expr; $($tail:tt)*) => {
        let $id$(:$t)? = $e.unwrap();
        $crate::unwrap_sugar!($($tail)*)
    };
    (mut $id:ident$(:$t:ty)? = $e:expr; $($tail:tt)*) => {
        let mut $id$(:$t)? = $e.unwrap();
        $crate::unwrap_sugar!($($tail)*)
    };
    () => {};
}

#[cfg(test)]
mod tests {
    #[test]
    #[allow(unused_mut)]
    fn it_works() {
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
    }
}
