# Game Orbits Library

Library for doing simplified orbital mechanics calculations in real time for video games. This is
done by using basic keplerian/elliptical orbits, and providing the means to translate keplerian
orbits into relative direction vectors for rendering in a game.

This library is built to be completely game engine agnostic. It provides
functions needed to feed it various orbital data as floating point numbers, and receive vectors
back representing positions and directions in game engine space from various
frames of reference. The intention is to also write a wrapper library for godot
in order to use this for a personal project of mine in the Godot engine, but it
should be theoretically possible to use this in any engine that allows you to
write a wrapper library around rust code or binaries.

This library may also be expanded with additional functionality for things like
generating meshes at runtime for different texture projections, and tools for
projecting astrometric data from NASA and other organizations into textures for
use in games.

### Usability

I do not recommend using this library in your projects currently. It's a bit of
a mess and the interface isn't very friendly and will likely change drastically
as I start to actually use it for its intended purpose in my own games and make
improvements based on that. However if you would like to use it eventually and
have feedback on something you'd like the library to do, or on how you'd like
the interface with the library to ideally function, I would love to hear it!
Also, feedback/advice on how to more cleanly integrate the game engine feature
flags would be very welcome.

## Tests

Due to a limitation of `cargo` combined with the fact that I use the Bevy engine
to render my examples, unit tests require compiling bevy to run. This won't
include Bevy or require Bevy for any projects that use this library, as bevy is
only compiled in the `dev` profile when running examples or tests, but it does
mean the first time compiling unit tests will take a significant amount of time
as it compiles Bevy along with the tests. Building the library itself or its
documentation should be quite fast.
