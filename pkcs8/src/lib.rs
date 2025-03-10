//! Pure Rust implementation of Public-Key Cryptography Standards (PKCS) #8:
//! Private-Key Information Syntax Specification ([RFC 5208]), with additional
//! support for PKCS#8v2 asymmetric key packages ([RFC 5958])
//!
//! # About PKCS#8
//! PKCS#8 is a format for cryptographic private keys, often containing pairs
//! of private and public keys.
//!
//! You can identify a PKCS#8 private key encoded as PEM (i.e. text) by the
//! following:
//!
//! ```text
//! -----BEGIN PRIVATE KEY-----
//! ```
//!
//! PKCS#8 private keys can optionally be encrypted under a password using
//! key derivation algorithms like PBKDF2 and [scrypt], and encrypted with
//! ciphers like AES-CBC. When a PKCS#8 private key has been encrypted,
//! it starts with the following:
//!
//! ```text
//! -----BEGIN ENCRYPTED PRIVATE KEY-----
//! ```
//!
//! PKCS#8 private keys can also be serialized in an ASN.1-based binary format.
//! The PEM text encoding is a Base64 representation of this format.
//!
//! # About this crate
//! This library provides generalized PKCS#8 support designed to work with a
//! number of different algorithms. It supports `no_std` platforms including
//! ones without a heap (albeit with reduced functionality).
//!
//! It supports decoding/encoding the following types:
//!
//! - [`EncryptedPrivateKeyInfo`]: (with `pkcs5` feature) encrypted key.
//! - [`PrivateKeyInfo`]: algorithm identifier and data representing a private key.
//!   Optionally also includes public key data for asymmetric keys.
//! - [`SubjectPublicKeyInfo`]: algorithm identifier and data representing a public key
//!   (re-exported from the [`spki`] crate)
//!
//! When the `alloc` feature is enabled, the following additional types are
//! available which provide more convenient decoding/encoding support:
//!
//! - [`EncryptedPrivateKeyDocument`]: (with `pkcs5` feature) heap-backed encrypted key.
//! - [`PrivateKeyDocument`]: heap-backed storage for serialized [`PrivateKeyInfo`].
//! - [`PublicKeyDocument`]: heap-backed storage for serialized [`SubjectPublicKeyInfo`].
//!
//! When the `pem` feature is enabled, it also supports decoding/encoding
//! documents from "PEM encoding" format as defined in RFC 7468.
//!
//! # Supported Algorithms
//! This crate has been written generically so it can be used to implement
//! PKCS#8 support for any algorithm.
//!
//! However, it's only tested against keys generated by OpenSSL for the
//! following algorithms:
//!
//! - ECC (`id-ecPublicKey`)
//! - Ed25519 (`id-Ed25519`)
//! - RSA (`id-rsaEncryption`)
//! - X25519 (`id-X25519`)
//!
//! Please open an issue if you encounter trouble using it with other
//! algorithms.
//!
//! # Encrypted Private Key Support
//! [`EncryptedPrivateKeyInfo`] supports decoding/encoding encrypted PKCS#8
//! private keys and is gated under the `pkcs5` feature. The corresponding
//! [`EncryptedPrivateKeyDocument`] type provides heap-backed storage
//! (`alloc` feature required).
//!
//! When the `encryption` feature of this crate is enabled, it provides
//! [`EncryptedPrivateKeyInfo::decrypt`] and [`PrivateKeyInfo::encrypt`]
//! functions which are able to decrypt/encrypt keys using the following
//! algorithms:
//!
//! - [PKCS#5v2 Password Based Encryption Scheme 2 (RFC 8018)]
//!   - Key derivation functions:
//!     - [scrypt] ([RFC 7914])
//!     - PBKDF2 ([RFC 8018](https://datatracker.ietf.org/doc/html/rfc8018#section-5.2))
//!       - SHA-2 based PRF with HMAC-SHA224, HMAC-SHA256, HMAC-SHA384, or HMAC-SHA512
//!       - SHA-1 based PRF with HMAC-SHA1, when the `sha1` feature of this crate is enabled.
//!   - Symmetric encryption: AES-128-CBC, AES-192-CBC, or AES-256-CBC
//!     (best available options for PKCS#5v2)
//!  
//! ## Legacy DES-CBC and DES-EDE3-CBC (3DES) support (optional)
//! When the `des-insecure` and/or `3des` features are enabled this crate provides support for
//! private keys encrypted with with DES-CBC and DES-EDE3-CBC (3DES or Triple DES) symmetric
//! encryption, respectively.
//!
//! ⚠️ WARNING ⚠️
//!
//! DES support is implemented to allow for decryption of legacy files.
//!
//! DES is considered insecure due to its short key size. New keys should use AES instead.
//!
//! # PKCS#1 support (optional)
//! When the `pkcs1` feature of this crate is enabled, this crate provides
//! a blanket impl of PKCS#8 support for types which impl the traits from the
//! [`pkcs1`] crate (e.g. `FromRsaPrivateKey`, `ToRsaPrivateKey`).
//!
//! # Minimum Supported Rust Version
//! This crate requires **Rust 1.51** at a minimum.
//!
//! [RFC 5208]: https://tools.ietf.org/html/rfc5208
//! [RFC 5958]: https://tools.ietf.org/html/rfc5958
//! [RFC 7914]: https://datatracker.ietf.org/doc/html/rfc7914
//! [PKCS#5v2 Password Based Encryption Scheme 2 (RFC 8018)]: https://tools.ietf.org/html/rfc8018#section-6.2
//! [scrypt]: https://en.wikipedia.org/wiki/Scrypt

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg",
    html_root_url = "https://docs.rs/pkcs8/0.7.6"
)]
#![forbid(unsafe_code, clippy::unwrap_used)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

mod error;
mod private_key_info;
mod traits;
mod version;

#[cfg(feature = "alloc")]
mod document;

#[cfg(feature = "pkcs5")]
pub(crate) mod encrypted_private_key_info;

pub use crate::{
    error::{Error, Result},
    private_key_info::PrivateKeyInfo,
    traits::{FromPrivateKey, FromPublicKey},
    version::Version,
};
pub use der::{self, asn1::ObjectIdentifier};
pub use spki::{AlgorithmIdentifier, SubjectPublicKeyInfo};

#[cfg(feature = "alloc")]
pub use crate::{
    document::{private_key::PrivateKeyDocument, public_key::PublicKeyDocument},
    traits::{ToPrivateKey, ToPublicKey},
};

#[cfg(feature = "pem")]
#[cfg_attr(docsrs, doc(cfg(feature = "pem")))]
pub use pem_rfc7468::LineEnding;

#[cfg(feature = "pkcs5")]
pub use encrypted_private_key_info::EncryptedPrivateKeyInfo;

#[cfg(feature = "pkcs1")]
pub use pkcs1;

#[cfg(feature = "pkcs5")]
pub use pkcs5;

#[cfg(all(feature = "alloc", feature = "pkcs5"))]
pub use crate::document::encrypted_private_key::EncryptedPrivateKeyDocument;

#[cfg(feature = "pem")]
use pem_rfc7468 as pem;
