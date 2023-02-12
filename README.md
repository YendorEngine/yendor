# Yendor

## Yendor is a set of opinionated tools for the bevy engine

Yendor startewd out to provide tools for building roguelikes and other games with a grid-based design. It is now becoming more of a set of opinionated tools for the bevy engine. It is inspired by the [bracket-lib](https://github.com/amethyst/bracket-lib), and [doryen-rs](https://github.com/jice-nospam/doryen-rs)

## Compatible Bevy versions

The main branch is compatible with the latest Bevy release

Compatibility of `yendor-lib` versions:
| `yendor-lib` | `bevy` |
| :-- | :-- |
| `0.1.0` | `0.9` |

## Libraries Used

Bevy is only possible because of the hard work put into these foundational technologies:

- [bevy](https://github.com/bevyengine/bevy): a refreshingly simple data-driven game engine built in Rust
- [noise-rs](https://github.com/Razaekel/noise-rs): a Rust library to generate smoothly varying noise for textural use and graphical display.
- [bevy_egui](https://github.com/amethyst/bracket-lib): This crate provides a Egui integration for the Bevy game engine.
- [pathfinding](https://github.com/samueltardieu/pathfinding): This crate implements several pathfinding, flow, and graph algorithms in Rust.

## Thanks and Alternatives

And as always, a special shoutout to [thebracket](https://github.com/thebracket) for his [bracket-lib](https://github.com/amethyst/bracket-lib) game engine which began my journey into the world of roguelikes. He is my inspiration for this project.

## License

`yendor-lib` is free, open source and permissively licensed!
All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.

The [assets](assets) included in this repository (for our [examples](./examples/README.md)) typically fall under different open licenses, but most are free for commercial use.

See [CREDITS.md](CREDITS.md) for the details of the licenses of those files.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
