# Game Orbits Library

[![Static Badge](https://img.shields.io/badge/Patreon-F96854?logo=patreon)](https://patreon.com/fernandogamedev)
[![Static Badge](https://img.shields.io/badge/Ko--Fi-72A5F2?logo=kofi)](https://ko-fi.com/fernando_gamedev)

Library for doing simplified orbital mechanics calculations in real time for video games, using
basic Keplerian orbital mechanics.

IMPORTANT: This library is NOT ready for use in your project, and I strongly recommend against doing
so. It's disorganized, badly made, and is going to change drastically during its development. At the
present time it exists just as a way to compartmentalize some complicated code I'm writing for a
science fiction game project to make it easier to maintain as I develop the game, and it's public
not to be used as a library for your own games but just so that anyone curious can look at how I
implemented the orbital mechanics that I did, and also just play around with the solar system
example. This isn't to say that it will never be in a state where it will be useful as a standalone
library for other peoples' projects, it just isn't in that state now and won't be for the near future.

That said, I will still happily accept feedback and constructive criticism in the Discussions
section, especially if it has to do with making this more rusty, more modular, or more useful as a
library to your needs. Just know that if you decide to rely on this code in its current state for a
project, you're probably in for a huge headache.

### Capabilities

This library currently contains the means to store a database of orbital bodies with their orbits
defined by Keplerian elements, with the database having additional functionality to calculate the
bodies' positions along their orbits at arbitrary times, and bodies' relative positions to each
other to aid in rendering them in game engines and "faking" a full scale solar system with visual
trickery manipulating direction and scale of rendered objects.

This library is built to be completely game engine agnostic. It provides
functions needed to feed it various orbital data as floating point numbers, and receive vectors
back representing positions and directions in game engine space from various
frames of reference. The intention is to also write a wrapper library for godot
in order to use this for a personal project of mine in the Godot engine, but it
should be theoretically possible to use this in any engine that allows you to
write a wrapper library around rust code or compiled artifacts.

### Examples

This library includes a few examples that showcase the library's use in the Bevy game engine. For
details on the individual examples and how to run them yourself, see the [examples README file](./examples/README.md).

### Tests

Due to a limitation of `cargo` combined with the fact that I use the Bevy engine to render my
examples, unit tests require compiling bevy to run. This won't include Bevy or require Bevy for any
projects that use this library, but it does mean the first time compiling unit tests will take a
significant amount of time as it compiles Bevy along with the tests. Building the library itself or
its documentation should still be quite fast sinze they don't make any use of Bevy. Use the
following to run tests:

```
cargo test --features bevy
```

### Feature flags

Currently, the library contains some wrapper structs for the Bevy and Godot engines that can be
included by using the `bevy` or `godot` feature flags respectively. To see what these flags add, see
the appropriate `feat_*.rs` file.

## References

- [*Orbital Mechanics*](http://www.braeunig.us/space/orbmech.htm) by Robert A. Braeunig
- [OrbiterWiki](https://www.orbiterwiki.org/wiki/Main_Page) for orbital parameters for various moons
- [Wikipedia](https://wikipedia.org) for its helpful diagrams of various concepts and generally being a great provider of free information
