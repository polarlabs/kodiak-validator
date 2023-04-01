#![forbid(unsafe_code)]
// todo: #![deny(missing_docs)]
// Lints for rustdoc
#![deny(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]
//#![deny(missing_doc_code_examples)] (unstable)
#![deny(rustdoc::invalid_codeblock_attributes)]
//#![deny(rustdoc::invalid_html_tags)] (unstable)
#![deny(rustdoc::invalid_rust_codeblocks)]
#![deny(rustdoc::bare_urls)]
#![warn(rustdoc::private_doc_tests)]
/*
Collection of useful rustdoc options awaiting their implementation.
#![doc(html_logo_url = "https://example.com/logo.jpg")]
#![doc(html_favicon_url = "https://example.com/favicon.ico")]
*/
#![deny(clippy::all)]
//todo: #![deny(warnings)]

pub mod domain;
pub mod email_address;
