// Copyright 2019 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! Module containing all of macro definitions for `async-coap-uri`.

pub use super::{impl_uri_buf_traits, impl_uri_traits};
pub use super::{rel_ref, uri, uri_ref};
pub use super::{rel_ref_format, uri_format, uri_ref_format};

// Internal macros.
#[doc(hidden)]
pub use super::{_impl_uri_buf_traits_base, _impl_uri_traits, _impl_uri_traits_base};

#[doc(hidden)]
#[macro_export]
macro_rules! _uri_const {
    ( $S:expr, $C:ty ) => {{
        const __CONST_S: &'static str = $S;
        // We do this weird casting thing here to make sure that we
        // don't end up using unstable features, while still allowing
        // these macros to be used to assign constants.
        unsafe {
            union Slices<'a> {
                str: &'a str,
                uri: &'a $C,
            }
            Slices { str: __CONST_S }.uri
        }
    }};
}

/// Creates a `&'static UriRef` from a string literal.
///
/// Accepts only string constants and literals. The given string *MUST* be well-formed.
///
/// Examples:
///
/// ```
/// # use async_coap_uri::prelude::*;
/// let x = uri_ref!("a/b/c?q=foobar#frag");
/// assert_eq!(x.scheme(),None);
/// assert_eq!(x.raw_authority(),None);
/// assert_eq!(x.raw_path(),"a/b/c");
/// assert_eq!(x.raw_query(),Some("q=foobar"));
/// assert_eq!(x.raw_fragment(),Some("frag"));
/// ```
///
/// ```
/// # use async_coap_uri::prelude::*;
/// let x = uri_ref!("http://example.com");
/// assert_eq!(x.scheme(),Some("http"));
/// assert_eq!(x.raw_authority(),Some("example.com"));
/// assert_eq!(x.raw_path(),"");
/// assert_eq!(x.raw_query(),None);
/// assert_eq!(x.raw_fragment(),None);
/// ```
///
/// Checks for correctness are performed at compile time:
///
/// ```compile_fail
/// # use async_coap_uri::prelude::*;
/// // This will not compile.
/// let x = uri_ref!("%00 invalid %ff");
/// ```
///
#[macro_export]
macro_rules! uri_ref {
    ( unsafe $S:expr ) => {{
        // We don't do any correctness checks when $S is preceded by `unsafe`.
        $crate::_uri_const!($S, $crate::UriRef)
    }};
    ( $S:expr ) => {{
        $crate::assert_uri_ref_literal!($S);
        $crate::_uri_const!($S, $crate::UriRef)
    }};
    ( ) => {
        $crate::uri_ref!("")
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! iuri_ref {
    ( unsafe $S:expr ) => {{
        // We don't do any correctness checks when $S is preceded by `unsafe`.
        $crate::_uri_const!($S, $crate::UriRef)
    }};
    ( $S:expr ) => {{
        assert_uri_ref_literal!($S);
        $crate::_uri_const!($S, $crate::UriRef)
    }};
    ( ) => {
        $crate::uri_ref!("")
    };
}

/// Creates a `&'static RelRef` from a string literal.
///
/// Accepts only string constants and literals. The given string *MUST* be well-formed.
///
/// Example:
///
/// ```
/// # use async_coap_uri::prelude::*;
/// let x = rel_ref!("a/b/c?q=foobar#frag");
/// assert_eq!(x.raw_path(),"a/b/c");
/// assert_eq!(x.raw_query(),Some("q=foobar"));
/// assert_eq!(x.raw_fragment(),Some("frag"));
/// ```
///
/// Checks for correctness are performed at compile time:
///
/// ```compile_fail
/// # use async_coap_uri::prelude::*;
/// // This will not compile.
/// let x = rel_ref!("%00 invalid %ff");
/// ```
///
/// Degenerate cases (strings that could be confused with URIs if parsed as URI-Refs)
/// will also not compile:
///
/// ```compile_fail
/// # use async_coap_uri::prelude::*;
/// // This will not compile because `//a/b/c` is
/// // a degenerate relative reference.
/// let x = rel_ref!("//a/b/c");
/// ```
///
/// ```compile_fail
/// # use async_coap_uri::prelude::*;
/// // This will not compile because `g:a:b:c` is
/// // a degenerate relative reference.
/// let x = rel_ref!("g:a:b:c");
/// ```
///
/// Both of those cases can be made to compile by adjusting them to no longer be degenerate:
///
/// ```
/// # use async_coap_uri::prelude::*;
/// let b = rel_ref!("/.//a/b/c"); // Prepending "/."
/// let a = rel_ref!("./g:a:b:c"); // Prepending "./"
/// let a = rel_ref!("g%3Aa:b:c"); // Convert first colon to "%3A"
/// ```
///
/// At runtime, `UriRef::from_str("g:a:b:c")` is allowed since in some circumstances it cannot
/// be avoided, but there is usually no good reason to have a degenerate `RelRef` literal.
/// In the rare case where such a thing is warranted (unit tests, for example), you can disable
/// compile-time verification by prepending the keyword `unsafe` to the string:
///
/// ```
/// # use async_coap_uri::prelude::*;
/// // Both of these will compile because the `unsafe` keyword
/// // disables the compile-time validity checks:
/// assert!(rel_ref!(unsafe "//a/b/c").is_degenerate());
/// assert!(rel_ref!(unsafe "g:a:b:c").is_degenerate());
/// ```
#[macro_export]
macro_rules! rel_ref {
    ( unsafe $S:expr ) => {{
        // We don't do any correctness checks when $S is preceded by `unsafe`.
        $crate::_uri_const!($S, $crate::RelRef)
    }};
    ( $S:expr ) => {{
        $crate::assert_rel_ref_literal!($S);
        $crate::_uri_const!($S, $crate::RelRef)
    }};
    ( ) => {
        $crate::rel_ref!("")
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! irel_ref {
    ( unsafe $S:expr ) => {{
        // We don't do any correctness checks when $S is preceded by `unsafe`.
        $crate::_uri_const!($S, $crate::RelRef)
    }};
    ( $S:expr ) => {{
        assert_rel_ref_literal!($S);
        $crate::_uri_const!($S, $crate::RelRef)
    }};
    ( ) => {
        $crate::rel_ref!("")
    };
}

/// Creates a `&'static Uri` from a string literal.
///
/// Accepts only string constants and literals. The given string *MUST* be well-formed.
///
/// Example:
///
/// ```
/// # use async_coap_uri::prelude::*;
/// let x = uri!("http://example.com");
/// assert_eq!(x.scheme(),Some("http"));
/// assert_eq!(x.raw_authority(),Some("example.com"));
/// ```
///
/// Checks for correctness are performed at compile time:
///
/// ```compile_fail
/// # use async_coap_uri::prelude::*;
/// // This will not compile.
/// let x = uri!("%00 invalid %ff");
/// ```
///
/// Passing something that is a valid URI-Reference but not a valid URI (i.e.: Missing scheme)
/// will also not compile:
///
/// ```compile_fail
/// # use async_coap_uri::prelude::*;
/// // This will not compile because "a/b/c" isn't a valid URI.
/// let x = uri!("a/b/c");
/// ```
///
#[macro_export]
macro_rules! uri {
    ( unsafe $S:expr ) => {{
        // We don't do any correctness checks when $S is preceded by `unsafe`.
        $crate::_uri_const!($S, $crate::Uri)
    }};
    ( $S:expr ) => {{
        $crate::assert_uri_literal!($S);
        $crate::_uri_const!($S, $crate::Uri)
    }};
    ( ) => {
        $crate::uri!("")
    };
}

#[doc(hidden)]
// This macro should be used by `async-coap-uri` and the uri macro is used for downstream crates.
//
// This prevents prevents the error:
// > macro-expanded `macro_export` macros from the current
// > crate cannot be referred to by absolute paths
//
// and allows downstream crates to use the uri macro without having to import
// the `assert_uri_literal` macro.
#[macro_export]
macro_rules! iuri {
    ( unsafe $S:expr ) => {{
        // We don't do any correctness checks when $S is preceded by `unsafe`.
        $crate::_uri_const!($S, $crate::Uri)
    }};
    ( $S:expr ) => {{
        assert_uri_literal!($S);
        $crate::_uri_const!($S, $crate::Uri)
    }};
    ( ) => {
        $crate::uri!("")
    };
}

/// Creates a `Option<UriRefBuf>` from the given string format and arguments.
///
/// The resulting string is checked at runtime to ensure it is well-formed.
#[cfg(feature = "std")]
#[macro_export]
macro_rules! uri_ref_format {
    ($($arg:tt)*) => ($crate::UriRefBuf::from_string(::std::format!($($arg)*)))
}

/// Creates a `Option<UriBuf>` from the given string format and arguments.
///
/// The resulting string is checked at runtime to ensure it is well-formed.
#[cfg(feature = "std")]
#[macro_export]
macro_rules! uri_format {
    ($($arg:tt)*) => ($crate::UriBuf::from_string(::std::format!($($arg)*)))
}

/// Creates a `Option<RelRefBuf>` from the given string format and arguments.
///
/// The resulting string is checked at runtime to ensure it is well-formed.
#[cfg(feature = "std")]
#[macro_export]
macro_rules! rel_ref_format {
    ($($arg:tt)*) => ($crate::RelRefBuf::from_string(::std::format!($($arg)*)))
}

#[doc(hidden)]
#[macro_export]
macro_rules! _impl_uri_traits {
    ( $C:ty ) => {
        impl<T: ::core::convert::AsRef<str> + ?::core::marker::Sized> ::core::cmp::PartialEq<T>
            for $C
        {
            fn eq(&self, other: &T) -> bool {
                ::core::cmp::PartialEq::eq(self.as_str(), other.as_ref())
            }
        }

        impl<T: ::core::convert::AsRef<str> + ?::core::marker::Sized> ::core::cmp::PartialOrd<T>
            for $C
        {
            fn partial_cmp(&self, other: &T) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(self.as_str(), other.as_ref())
            }
        }

        impl ::core::cmp::Ord for $C {
            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                ::core::cmp::Ord::cmp(self.as_str(), other.as_str())
            }
        }

        impl ::core::fmt::Debug for $C {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(concat!(stringify!($C), "<"))?;
                ::core::fmt::Display::fmt(self.as_str(), f)?;
                f.write_str(">")
            }
        }
        impl ::core::convert::AsRef<str> for $C {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }
        impl ::core::convert::AsRef<$C> for $C {
            fn as_ref(&self) -> &$C {
                &self
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _impl_uri_traits_base {
    ( $C:ty ) => {
        _impl_uri_traits!($C);

        impl ::core::convert::From<&$C> for ::std::string::String {
            fn from(x: &$C) -> Self {
                ::std::string::String::from(&x.0)
            }
        }

        impl ::core::convert::From<&$C> for $crate::UriRefBuf {
            fn from(x: &$C) -> Self {
                unsafe {
                    $crate::UriRefBuf::from_string_unchecked(::std::string::String::from(&x.0))
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_uri_traits {
    ( $C:ty ) => {
        _impl_uri_traits_base!($C);

        impl $crate::AnyUriRef for $C {
            fn components(&self) -> $crate::UriRawComponents<'_> {
                self.0.components()
            }

            fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            fn uri_type(&self) -> $crate::UriType {
                self.0.uri_type()
            }

            fn to_uri_ref_buf(&self) -> $crate::UriRefBuf {
                self.0.to_uri_ref_buf()
            }

            unsafe fn write_to_unsafe<W: ::core::fmt::Write + ?::core::marker::Sized>(
                &self,
                write: &mut W,
            ) -> ::core::fmt::Result {
                self.0.write_to_unsafe(write)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _impl_uri_buf_traits_base {
    ( $C:ty , $B:ty ) => {
        _impl_uri_traits!($C);

        impl ::core::convert::From<$C> for ::std::string::String {
            fn from(x: $C) -> Self {
                ::std::string::String::from(x.0)
            }
        }

        impl ::core::convert::From<&$C> for $C {
            fn from(x: &$C) -> Self {
                <$C as ::core::clone::Clone>::clone(x)
            }
        }

        impl ::std::borrow::ToOwned for $B {
            type Owned = $C;

            fn to_owned(&self) -> Self::Owned {
                unsafe {
                    <$C>::from_string_unchecked(<Self as ::std::string::ToString>::to_string(self))
                }
            }
        }

        impl ::core::borrow::Borrow<$B> for $C {
            fn borrow(&self) -> &$B {
                unsafe { <$B>::from_str_unchecked(self.as_str()) }
            }
        }

        impl $crate::AnyUriRef for $C {
            fn components(&self) -> $crate::UriRawComponents<'_> {
                let b: &$B = <Self as ::core::borrow::Borrow<$B>>::borrow(self);
                b.components()
            }

            fn is_empty(&self) -> bool {
                let b: &$B = <Self as ::core::borrow::Borrow<$B>>::borrow(self);
                b.is_empty()
            }

            fn uri_type(&self) -> $crate::UriType {
                let b: &$B = <Self as ::core::borrow::Borrow<$B>>::borrow(self);
                b.uri_type()
            }

            fn to_uri_ref_buf(&self) -> $crate::UriRefBuf {
                let b: &$B = <Self as ::core::borrow::Borrow<$B>>::borrow(self);
                b.to_uri_ref_buf()
            }

            unsafe fn write_to_unsafe<W: ::core::fmt::Write + ?::core::marker::Sized>(
                &self,
                write: &mut W,
            ) -> ::core::fmt::Result {
                let b: &$B = <Self as ::core::borrow::Borrow<$B>>::borrow(self);
                b.write_to_unsafe(write)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_uri_buf_traits {
    ( $C:ty , $B:ty) => {
        _impl_uri_buf_traits_base!($C, $B);

        impl ::core::convert::AsRef<::std::string::String> for $C {
            fn as_ref(&self) -> &std::string::String {
                ::core::convert::AsRef::<::std::string::String>::as_ref(&self.0)
            }
        }

        impl ::core::convert::AsRef<$crate::UriRefBuf> for $C {
            fn as_ref(&self) -> &$crate::UriRefBuf {
                ::core::convert::AsRef::<$crate::UriRefBuf>::as_ref(&self.0)
            }
        }

        impl ::core::convert::From<$C> for $crate::UriRefBuf {
            fn from(x: $C) -> Self {
                ::core::convert::Into::<Self>::into(x.0)
            }
        }
    };
}
