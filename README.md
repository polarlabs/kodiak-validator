# kodiak-validator

[![GitHub Top Language]][lang]
[![Static unsafe]][unsafe]
[![crates.io License]][license-mit]
[![GitHub License]][license-apache]


[![GitHub Latest Release]][github-releases]
[![GitHub Commits]][github-commits]


[![Code Coverage]][codecov]
[![GitHub Build Status]][github-actions-cargo-test]
[![Static docs coverage]][docs]
[![docs.rs]][docs]
[![Libraries.io Dep Status]][libraries]


[![GitHub Security Schedule]][github-actions-cargo-audit-on-schedule]
[![GitHub Security Push]][github-actions-cargo-audit-on-push]


[![GitHub Open Issues]][github-issues]
[![GitHub Closed Issues]][github-issues]


[![crates.io Latest]][crates]
[![crates.io Recent]][crates]

[Code Coverage]: https://img.shields.io/codecov/c/github/polarlabs/kodiak-validator?label=code%20coverage&logo=codecov&logoColor=ffffff&style=flat-square
[codecov]: https://codecov.io/github/polarlabs/kodiak-validator

[crates.io Recent]: https://img.shields.io/crates/dr/kodiak-validator?logo=docs.rs&color=67001f&style=flat-square
[crates.io Latest]: https://img.shields.io/crates/v/kodiak-validator?label=latest&logo=docs.rs&style=flat-square
[crates]: https://crates.io/crates/kodiak-validator

[crates.io License]: https://img.shields.io/crates/l/kodiak-validator?color=007ec6&style=flat-square
[GitHub License]: https://img.shields.io/github/license/polarlabs/kodiak-validator?color=007ec6&style=flat-square
[license-mit]: https://choosealicense.com/licenses/mit/
[license-apache]: https://choosealicense.com/licenses/apache-2.0/

[Static docs coverage]: https://img.shields.io/badge/docs%20coverage-100%25-success.svg?logo=rust&logoColor=ffffff&style=flat-square
[docs.rs]: https://img.shields.io/docsrs/kodiak-validator?logo=docs.rs&style=flat-square
[docs]: https://docs.rs/kodiak-validator

[GitHub Build Status]: https://img.shields.io/github/workflow/status/polarlabs/kodiak-validator/workflow-cargo-test?logo=github&label=tests&style=flat-square
[github-actions-cargo-test]: https://github.com/polarlabs/kodiak-validator/actions/workflows/cargo-test.yml

[GitHub Security Schedule]: https://img.shields.io/github/workflow/status/polarlabs/kodiak-validator/workflow-cargo-audit-on-schedule?logo=clockify&logoColor=ffffff&label=security%20audit&style=flat-square
[github-actions-cargo-audit-on-schedule]: https://github.com/polarlabs/kodiak-validator/actions/workflows/cargo-audit-on-schedule.yml

[GitHub Security Push]: https://img.shields.io/github/workflow/status/polarlabs/kodiak-validator/workflow-cargo-audit-on-push?logo=github&label=security%20audit&style=flat-square
[github-actions-cargo-audit-on-push]: https://github.com/polarlabs/kodiak-validator/actions/workflows/cargo-audit-on-push.yml

[GitHub Top Language]: https://img.shields.io/github/languages/top/polarlabs/kodiak-validator?color=dea584&logo=rust&style=flat-square
[lang]: https://www.rust-lang.org/

[GitHub Latest Release]: https://img.shields.io/github/v/release/polarlabs/kodiak-validator?include_prereleases&sort=semver&logo=github&label=latest&style=flat-square
[github-releases]: https://github.com/polarlabs/kodiak-validator/releases

[GitHub Commits]: https://img.shields.io/github/commits-since/polarlabs/kodiak-validator/latest?include_prereleases&sort=semver&logo=github&style=flat-square
[github-commits]: https://github.com/polarlabs/kodiak-validator/commits

[GitHub Open Issues]: https://img.shields.io/github/issues-raw/polarlabs/kodiak-validator?logo=github&style=flat-square
[GitHub Closed Issues]: https://img.shields.io/github/issues-closed-raw/polarlabs/kodiak-validator?logo=github&style=flat-square
[github-issues]: https://github.com/polarlabs/kodiak-validator/issues

[Libraries.io Dep Status]: https://img.shields.io/librariesio/github/polarlabs/kodiak-validator?logo=libraries.io&logoColor=ffffff&style=flat-square
[libraries]: https://libraries.io/cargo/kodiak-validator

[Static unsafe]: https://img.shields.io/badge/unsafe-forbidden-success.svg?logo=rust&logoColor=ffffff&style=flat-square
[unsafe]: (https://github.com/rust-secure-code/safety-dance/)

Validate data you handle with these powerful, yet easy to use validators.

This library crate is a building block of the Kodiak project, thus the naming of the crate.
Although, Kodiak has some quite specific requirements for its validators, `kodiak-validator` is kept generic,
provides value on its own and might be of interest for other projects as well.

A validator[^Wikipedia] checks the validity or syntactical correctness of a piece of data, 
e.g. a fragment of code or a  document or data.

When looking for a collection of validators, `kodiak-validator` might be a good fit
for you. So, feel free to use it. If you consider using `kodiak-validator` in your 
project but are missing functionality or have any other concerns, don't hesitate 
to file an issue on GitHub.

We are looking forward to your feedback.

---

You may be looking for:

- [API documentation](https://docs.rs/kodiak-validator/)
- [Release notes](https://github.com/polarlabs/kodiak-validator/releases)

---

# Impressions

todo: show two examples of how to use validators offered by kodiak-validator

Provide additional examples in EXAMPLES.md and link to it.

# TL;DR

Kodiak's specific requirements regarding its validators:
- No additional dependencies.
- Usable in WebAssembly.
- Provide meaningful feedback why validation fails.

# Known issues / limitations
- 🏗️ Version 0.1.0 does not yet power other projects, so API has not yet proven it's power in practice.
- 🚧 Code is fully covered by unit tests, however, some integration tests are still missing.
- Documentation has still some room for improvement.
- 🐧 Version 0.1.x is developed and tested on Linux only.

# Roadmap and future considerations

## Version 0.4.0 (planned)
- Add validators for ip, ipv4, ipv6 and mac address.
- Use no-std + no-alloc?

## Version 0.3.0 (planned)
- Add validators for length, range and url.
- Publish Software Bill of Material (SBOM) together with any future release.
- Use LLVM engine of Tarpaulin to check code coverage.

## Version 0.2.0 (planned)
- Add validators for integer, decimal, number. 
- Improve documentation (see [CHECKLIST_RUST_API_CONFORMANCE](CHECKLIST_RUST_API_CONFORMANCE.md))
- Review and follow Rust API Guidelines.

## Version 0.1.0 (in progress)
- Validate domains including top-level domains and e-mail addresses (simplified implementation).
- Initial release.

# Additional resources

- Homepage polarlabs: [polarlabs.io](https://www.polarlabs.io)
- Crate: [crates.io/kodiak-validator](https://crates.io/crates/kodiak-validator)
- API documentation: [docs.rs/kodiak-validator](https://docs.rs/kodiak-validator/)

# Contributing

See [CONTRIBUTING](CONTRIBUTING.md) for more details.

# Appendix

## References

[^Wikipedia]: [Validator](https://en.wikipedia.org/wiki/Validator)

## Cargo Geiger Safety Report

```
Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols: 
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Dependency

0/0        0/0          0/0    0/0     0/0      :) kodiak-validator 0.1.0

0/0        0/0          0/0    0/0     0/0
```

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or https://opensource.org/licenses/Apache-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
