<div align="center">

# synthahol-dx7

Library to read presets for the
[Yamaha DX7](https://en.wikipedia.org/wiki/Yamaha_DX7)
synthesizer

[![crates.io][crates.io-badge]][crates.io]
[![Docs][docs-badge]][docs]
[![Workflows][workflows-badge]][workflows]
</div>

## Overview

This is a library to read presets for the 
[Yamaha DX7](https://en.wikipedia.org/wiki/Yamaha_DX7)
hardware synthesizer.

## Reading a Preset

```rust
use synthahol_dx7::Bank;

let bank = Bank::read_file("rom1a.syx").unwrap();

println!("This bank contains:");
for preset in bank {
    println!(preset.name);
}
```

## Issues

If you have any problems with or questions about this project, please contact
the developers by creating a
[GitHub issue](https://github.com/softdevca/synthahol-dx7/issues).

## Contributing

You are invited to contribute to new features, fixes, or updates, large or
small; we are always thrilled to receive pull requests, and do our best to
process them as fast as we can.

The copyrights of contributions to this project are retained by their
contributors. No copyright assignment is required to contribute to this
project.

## License

Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.

[crates.io]: https://crates.io/crates/synthahol-dx7
[crates.io-badge]: https://img.shields.io/crates/v/synthahol-dx7?logo=rust&logoColor=white&style=flat-square
[docs]: https://docs.rs/synthahol-dx7
[docs-badge]: https://docs.rs/synthahol-dx7/badge.svg
[workflows]: https://github.com/softdevca/synthahol-dx7/actions/workflows/ci.yml
[workflows-badge]: https://github.com/softdevca/synthahol-dx7/actions/workflows/ci.yml/badge.svg
