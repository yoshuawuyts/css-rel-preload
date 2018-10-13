# css-rel-preload
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

LoadCSS's `cssrelpreload.js` file.

- [Documentation][8]
- [Crates.io][2]
- [Releases][releases]

## Why?
If you want to use `rel=preload` tags to asynchronously load CSS in browsers, we
need to include a polyfill for backwards compatibility. This crate provides a
wrapper around [loadCSS](https://github.com/filamentgroup/loadCSS)'s
`rel-preload.js` file, providing that fallback. It's best included as an inline
string, or sent as part of every initial request using HTTP/2 PUSH.

## Examples
### Basic
```rust
use css_rel_preload;
use html_index;

let res = html_index::Builder::new()
  .raw_body("<body>hello world</body>")
  .inline_script(css_rel_preload::CSS_REL_PRELOAD)
  .style("/bundle.css")
  .build();
println!("{}", res);
```

## Installation
```sh
$ cargo add css-rel-preload
```

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

## References
- https://caniuse.com/#feat=link-rel-preload
- https://github.com/filamentgroup/loadCSS
- https://cdnjs.com/libraries/loadCSS

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/css-rel-preload.svg?style=flat-square
[2]: https://crates.io/crates/css-rel-preload
[3]: https://img.shields.io/travis/yoshuawuyts/css-rel-preload.svg?style=flat-square
[4]: https://travis-ci.org/yoshuawuyts/css-rel-preload
[5]: https://img.shields.io/crates/d/css-rel-preload.svg?style=flat-square
[6]: https://crates.io/crates/css-rel-preload
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/css-rel-preload

[releases]: https://github.com/yoshuawuyts/css-rel-preload/releases
[contributing]: https://github.com/yoshuawuyts/css-rel-preload/.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/yoshuawuyts/css-rel-preload/labels/good%20first%20issue
[help-wanted]: https://github.com/yoshuawuyts/css-rel-preload/labels/help%20wanted
