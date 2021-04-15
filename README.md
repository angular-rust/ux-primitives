<div align="center">

[![](https://dudochkin-victor.github.io/assets/ux-primitives/logo-wide.svg)](#top)
# UX Primitives

[![API Docs][docrs-badge]][docrs-url]
[![Crates.io][crates-badge]][crates-url]
[![Code coverage][codecov-badge]][codecov-url]
[![Tests][tests-badge]][tests-url]
[![MPL-2.0 licensed][license-badge]][license-url]
[![Gitter chat][gitter-badge]][gitter-url]
[![loc][loc-badge]][loc-url]
</div>

[docrs-badge]: https://img.shields.io/docsrs/ux-primitives?style=flat-square
[docrs-url]: https://docs.rs/ux-primitives/
[crates-badge]: https://img.shields.io/crates/v/ux-primitives.svg?style=flat-square
[crates-url]: https://crates.io/crates/ux-primitives
[license-badge]: https://img.shields.io/badge/license-MPL--2.0-blue.svg?style=flat-square
[license-url]: https://github.com/angular-rust/ux-primitives/blob/master/LICENSE
[gitter-badge]: https://img.shields.io/gitter/room/angular_rust/community.svg?style=flat-square
[gitter-url]: https://gitter.im/angular_rust/community
[tests-badge]: https://img.shields.io/github/workflow/status/angular-rust/ux-primitives/Tests?label=tests&logo=github&style=flat-square
[tests-url]: https://github.com/angular-rust/ux-primitives/actions/workflows/tests.yml
[codecov-badge]: https://img.shields.io/codecov/c/github/angular-rust/ux-primitives?logo=codecov&style=flat-square&token=RRKF1UAOSR
[codecov-url]: https://codecov.io/gh/angular-rust/ux-primitives
[loc-badge]: https://img.shields.io/tokei/lines/github/angular-rust/ux-primitives?style=flat-square
[loc-url]: https://github.com/angular-rust/ux-primitives

UX Primitives is a core graphic and color abstraction for Angular Rust.

**Angular Rust** is a high productivity, `platform-agnostic` frontend framework for the [Rust language](https://www.rust-lang.org/). It now supports desktop and web development. Angular Rust currently uses GTK for desktop development and WebAssembly for web development. We are planning to add support for mobile development.

![Angular Rust structure](https://dudochkin-victor.github.io/assets/angular-rust/structure.svg)

## Features

- [x] Graphic abstraction for cairo and web canvas, implemented in [UX Animate](https://github.com/angular-rust/ux-animate)
- [x] Most used color spaces: RGB, RGBA, HSL, HSV, LAB, CMY, CMYK
- [x] Color conversion: RGB to HSL, etc.
- [x] Support for color quantization to make the color closer to the palette
- [x] Palette `Open Color`


## Color scheme for UI design

UX Primitives contain powerfull palette for easy of use and more professional look for your applications. UX Primitives use the [Open Color](https://yeun.github.io/open-color) scheme optimized for UI like font, background, border, etc.

- All colors shall have adequate use
- Provide general color for UI design
- All colors will be beautiful in itself and harmonious
- At the same brightness level, the perceived brightness will be constant

## Available Colors

![available colors](https://dudochkin-victor.github.io/assets/ux-primitives/open-color.svg)

## Quick Start

Install UX Primitives:

	cargo add ux-primitives

## Learn More

* [Manual, Docs, etc](https://angular-rust.github.io/)
* [Samples](https://github.com/angular-rust/ux-samples)
* [Apps using Angular Rust](https://github.com/angular-rust/ux-primitives/wiki/Apps-in-the-Wild)
* [Articles Featuring Angular Rust](https://github.com/angular-rust/ux-primitives/wiki/Articles)

## Community

 [![](https://img.shields.io/badge/Facebook-1877F2?style=for-the-badge&logo=facebook&logoColor=white)](https://www.facebook.com/groups/angular.rust) 
 [![](https://img.shields.io/badge/Stack_Overflow-FE7A16?style=for-the-badge&logo=stack-overflow&logoColor=white)](https://stackoverflow.com/questions/tagged/angular-rust) 
 [![](https://img.shields.io/badge/YouTube-FF0000?style=for-the-badge&logo=youtube&logoColor=white)](https://www.youtube.com/channel/UCBJTkSl_JWShuolUy4JksTQ) 
 [![](https://img.shields.io/badge/Medium-12100E?style=for-the-badge&logo=medium&logoColor=white)](https://medium.com/@angular.rust) 
 [![](https://img.shields.io/gitter/room/angular_rust/angular_rust?style=for-the-badge)](https://gitter.im/angular_rust/community)


## Contributing

We believe the wider community can create better code. The first tool for improving the community is to tell the developers about the project by giving it a star. More stars - more members.

  [![](https://dudochkin-victor.github.io/assets/star-me-wide.svg)](https://github.com/angular-rust/ux-primives#top)

Angular Rust is a community effort and we welcome all kinds of contributions, big or small, from developers of all backgrounds. We want the Angular Rust community to be a fun and friendly place, so please review our [Code of Conduct](CODE_OF_CONDUCT.md) to learn what behavior will not be tolerated.

### New to Angular Rust?

Start learning about the framework by helping us improve our [documentation](https://angular-rust.github.io/). Pull requests which improve test coverage are also very welcome.

### Looking for inspiration?

Check out the community curated list of awesome things related to Angular Rust / WebAssembly at [awesome-angular-rust](https://github.com/angular-rust/awesome-angular-rust).

### Confused about something?

Feel free to drop into our [Gitter chatroom](https://gitter.im/angular_rust/community) or open a [new "Question" issue](https://github.com/angular-rust/ux-primitives/issues/new/choose) to get help from contributors. Often questions lead to improvements to the ergonomics of the framework, better documentation, and even new features!

### Ready to dive into the code?

After reviewing the [Contributing Code Guidelines](CONTRIBUTING.md), check out the ["Good First Issues"](https://github.com/angular-rust/ux-primitives/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) (they are eager for attention!). Once you find one that interests you, feel free to assign yourself to an issue and don't hesitate to reach out for guidance, the issues vary in complexity.

### Let's help each other!

Come help us on the [issues that matter that the most](https://github.com/angular-rust/ux-primitives/labels/%3Adollar%3A%20Funded%20on%20Issuehunt) and receive a small cash reward for your troubles. We use [Issuehunt](https://issuehunt.io/r/angular-rust/ux-primitives/) to fund issues from our Open Collective funds. If you really care about an issue, you can choose to add funds yourself! 

### Found a bug?

Please [report all bugs!](https://github.com/angular-rust/ux-primitives/issues/new/choose) We are happy to help support developers fix the bugs they find if they are interested and have the time.

## Todo
- [ ] Documentation
