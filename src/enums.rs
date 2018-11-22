//! **This is a modified (and incompatible) fork of**
//! **[enum-repr](https://crates.io/crates/enum-repr).**
//!
//! Generate enum repr conversions compatible with type aliases.
//!
//! Generate with `#[EnumRepr(type = "TYPE")]`.
//!
//! Functions generated are
//! ```ignore
//! fn repr(&self) -> EnumReprType
//! fn from_repr(x: EnumReprType) -> Option<Self>
//! ```
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
//! ```
//! extern crate pbirch;
//! extern crate libc;
//!
//! use libc::*;
//!
//! use pbirch::enums::{Enum, EnumRepr};
//!
//! #[EnumRepr(type = "c_int")]
//! #[derive(Debug, PartialEq)]
//! pub enum IpProto {
//!     IP = IPPROTO_IP,
//!     IPv6 = IPPROTO_IPV6,
//!     // …
//! }
//!
//! fn main() {
//!     assert_eq!(IpProto::IP.repr(), IPPROTO_IP);
//!     assert_eq!(IpProto::from_repr(IPPROTO_IPV6), Some(IpProto::IPv6));
//!     assert!(IpProto::from_repr(12345).is_none());
//! }
//! ```
//!
//! ```
//! # extern crate pbirch;
//! # extern crate libc;
//! #
//! # use libc::*;
//! #
//! # use pbirch::enums::{Enum, EnumRepr};
//! #
//! #[EnumRepr(type = "c_int")]
//! # #[derive(Debug, Eq, Hash, PartialEq)]
//! pub enum InetDomain {
//!     Inet = 2,
//!     // …
//! }
//!
//! #[EnumRepr(type = "c_int")]
//! # #[derive(Debug, Eq, Hash, PartialEq)]
//! pub enum SocketType {
//!     Stream = 1,
//!     // …
//! }
//!
//! // …
//!
//! # fn main() { unsafe {
//! assert!(
//!    socket(InetDomain::Inet.repr(), SocketType::Stream.repr(), 0) != -1
//! );
//! # }}
//! ```
//!
//! ```no_run
//! # extern crate pbirch;
//! # extern crate libc;
//! #
//! # use libc::*;
//! #
//! # use pbirch::enums::{Enum, EnumRepr};
//! #
//! // compatible with documentation and other attributes
//!
//! /// Represents a layer 3 network protocol.
//! #[EnumRepr(type = "c_int")]
//! #[derive(Debug, PartialEq)]
//! pub enum IpProto {
//!     IP = IPPROTO_IP,
//!     IPv6 = IPPROTO_IPV6,
//!     // …
//! }
//! #
//! # fn main() {}
//! ```
//!
//! Discriminants can be implicit if `implicit = true`:
//! ```
//! # extern crate pbirch;
//! # extern crate libc;
//! #
//! # use libc::*;
//! #
//! # use pbirch::enums::{Enum, EnumRepr};
//! #
//!
//! #[EnumRepr(type = "c_int", implicit = true)]
//! #[derive(Debug, PartialEq)]
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
//! Using implicit discriminants without setting the flag is an error:
//! ```compile_fail
//! # extern crate pbirch;
//! # extern crate libc;
//! #
//! # use libc::*;
//! #
//! # use pbirch::enums::{Enum, EnumRepr};
//! #
//!
//! #[EnumRepr(type = "c_int")]
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
//! # extern crate pbirch;
//! #
//! # use pbirch::enums::{Enum, EnumRepr};
//! #
//! #[EnumRepr(type = "u8", implicit = true)]
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
//! # extern crate pbirch;
//! #
//! # use pbirch::enums::{Enum, EnumRepr};
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
//! # extern crate pbirch;
//! #
//! # use pbirch::enums::{Enum, EnumRepr};
//! #
//! #[EnumRepr(type = "u8", implicit = true)]
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
//! # extern crate pbirch;
//! #
//! # use pbirch::enums::{Enum, EnumRepr};
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
//! # extern crate pbirch;
//! #
//! # use std::mem::size_of;
//! #
//! # use pbirch::enums::{Enum, EnumRepr};
//! #
//! #[EnumRepr(type = "u8")]
//! #[derive(Debug, PartialEq)]
//! enum Test {
//!     A = 1
//! }
//!
//! fn main() {
//!     assert_eq!(size_of::<u8>(), size_of::<Test>());
//! }
//! ```

pub use enum_repr::EnumRepr;

pub trait Enum<T: Copy>: Sized {
    fn repr(&self) -> T;

    fn from_repr(x: T) -> Option<Self>;
}
