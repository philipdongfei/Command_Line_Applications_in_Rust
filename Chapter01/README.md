#1.7 Packaging and distributing a Rust tool
## Building binary releases on CI
a free CI(continuous integration) service like [Travis CI](https://travis-ci.com/)
one example of building binaries using this approach.
[Linux and macOS]( 
https://github.com/rustwasm/wasm-pack/blob/51e6351c28fbd40745719e6d4a7bf26dadd30c85/.travis.yml#L74-L91)
[Windows]( https://github.com/rustwasm/wasm-pack/blob/51e6351c28fbd40745719e6d4a7bf26dadd30c85/.appveyor.yml)

another way is to use pre-built(Docker) images that contain all the tools we need to build binaries.The [trust](https://github.com/japaric/trust) project contains scripts that you can include in your project as well as instructions on how to set this up.

It uses [cross](https://github.com/rust-embedded/cross) internally, which works similar to cargo but forwards commands to a cargo process inside a Docker container.

