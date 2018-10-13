#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]

/// Include the CSS preload script.
pub const CSS_REL_PRELOAD: &'static str = include_str!("./css-rel-preload.js");
