//! **This is a modified (and incompatible) fork of**
//! **[enum-repr](https://crates.io/crates/enum-repr).**
//!
//! A lot of the changes have sacrificed robustness for compilation speed,
//! but if you run into trouble, you can always pass `fast = false`.
//!
//! Generate enum repr conversions compatible with type aliases.
//!
//! Generate with `#[EnumRepr(type = "TYPE")]`. The enum *must* implement
//! `Copy` and `Clone`, and it will derive `Copy`, `Clone`, `PartialEq`,
//! `Eq`, and `Debug` unless `derive = false`.
//!
//! Functions generated are
//!
//! > `fn repr(self) -> EnumReprType`
//! >
//! > `fn from_repr(x: EnumReprType) -> Option<Self>`
//!
//! The real enum discriminant is usually forced to be `#[repr(isize)]`.
//! If `u*` or `i*` types are used for the discriminant, the actual enum
//! representation is made to be `#[repr(that_type_specified)]`.
//! The list of types recognized as `u*` and `i*` currently is as follows:
//! `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`.
//! If the type is specified through a type alias, `#[repr(isize)]` is used.
//! Inability to specify type aliases as enum representations is this crate's
//! reason to exist.
//!
//! The code generated does not require std.
//!
//! # Examples
//!
//! Discriminants can be implicit if `implicit = true` (default):
//! ```
//! # extern crate vdex;
//! # extern crate enum_repr;
//! #
//! # use vdex::Enum;
//! # use enum_repr::EnumRepr;
//! #
//! #[EnumRepr(type = "i32")]
//! pub enum Test {
//!     A,
//!     B,
//!     C = 5,
//!     D,
//! }
//! 
//! fn main() {
//!     assert_eq!(Test::B.repr(), 1);
//!     assert_eq!(Test::from_repr(6), Some(Test::D));
//!     assert!(Test::from_repr(2).is_none());
//! }
//! ```
//!
//! Using implicit discriminants with the flag false is an error:
//! ```compile_fail
//! # extern crate vdex;
//! # extern crate enum_repr;
//! #
//! # use vdex::Enum;
//! # use enum_repr::EnumRepr;
//! #
//! #[EnumRepr(type = "i32", implicit = false)]
//! pub enum Test {
//!     A,
//!     B = 3
//! }
//! #
//! # fn main() {}
//! ```
//!
//! Take extra care to avoid collisions when using implicit discriminants:
//! ```compile_fail
//! # #![deny(overflowing_literals)]
//! # extern crate vdex;
//! # extern crate enum_repr;
//! #
//! # use vdex::Enum;
//! # use enum_repr::EnumRepr;
//! #
//! #[EnumRepr(type = "u8")]
//! enum Test {
//!     A = 1,
//!     B,
//!     C,
//!     D = 3,
//! }
//! #
//! # fn main() {}
//! ```
//!
//! Out of bound discriminants fail to compile:
//! ```compile_fail
//! # #![deny(overflowing_literals)]
//! # extern crate vdex;
//! # extern crate enum_repr;
//! #
//! # use vdex::Enum;
//! # use enum_repr::EnumRepr;
//! #
//! #[EnumRepr(type = "u8")]
//! enum Test {
//!     A = 256
//! }
//! #
//! # fn main() {}
//! ```
//!
//! Even if they are implicit:
//! ```compile_fail
//! # #![deny(overflowing_literals)]
//! # extern crate vdex;
//! # extern crate enum_repr;
//! #
//! # use vdex::Enum;
//! # use enum_repr::EnumRepr;
//! #
//! #[EnumRepr(type = "u8")]
//! enum Test {
//!     A = 255,
//!     B
//! }
//! #
//! # fn main() {}
//! ```
//!
//! Discriminants of a wrong type fail to compile as well:
//! ```compile_fail
//! # #![deny(overflowing_literals)]
//! # extern crate vdex;
//! # extern crate enum_repr;
//! #
//! # use vdex::Enum;
//! # use enum_repr::EnumRepr;
//! #
//! const C: u16 = 256;
//!
//! #[EnumRepr(type = "u8")]
//! enum Test {
//!     A = C
//! }
//! #
//! # fn main() {}
//! ```
//!
//! Using the actual enum discriminant representation:
//! ```
//! # extern crate vdex;
//! # extern crate enum_repr;
//! #
//! # use std::mem::size_of;
//! #
//! # use vdex::Enum;
//! # use enum_repr::EnumRepr;
//! #
//! #[EnumRepr(type = "u8")]
//! enum Test {
//!     A = 1
//! }
//!
//! fn main() {
//!     assert_eq!(size_of::<u8>(), size_of::<Test>());
//! }
//! ```
//!
//! Prevent automatic derive with `derive = false`:
//! ```
//! # extern crate vdex;
//! # extern crate enum_repr;
//! #
//! # use std::fmt;
//! # use vdex::Enum;
//! # use enum_repr::EnumRepr;
//! #
//! #[EnumRepr(type = "u8", derive = false)]
//! #[derive(Copy, Clone)]
//! enum Test {
//!     A,
//!     B,
//! }
//!
//! impl fmt::Debug for Test {
//!     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//!         write!(f, "Test({:?})", self.repr())
//!     }
//! }
//!
//! fn main() {
//!     assert_eq!(format!("{:?}", Test::B), "Test(1)");
//! }
//! ```

pub use enum_repr::EnumRepr;

/// All of the vdex C-style enums implement this trait, which allows for easy
/// conversion between the underlying integer representation and the enum type.
pub trait Enum<T: Copy> where Self: 'static + Sized + Copy {
    /// The total number of enum values.
    const COUNT: usize;

    /// All the enum values.
    const VALUES: &'static [Self];

    /// Returns the underlying representation of the enum value.
    fn repr(self) -> T;

    /// Returns the enum value corresponding to the passed representation, or
    /// `None` if no such enum value exists.
    fn from_repr(x: T) -> Option<Self>;
}
