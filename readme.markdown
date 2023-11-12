# Checks for unique decodability

This repo is an informal collection of implementations of [the Sardinas–Patterson algorithm](https://en.wikipedia.org/wiki/Sardinas%E2%80%93Patterson_algorithm), which is a test for **unique decodability**.

So far it includes my slow implementation (`src/schlinkert.rs`) and [an implementation by GitHub user @Colfenor](https://github.com/Colfenor/sardinas-patterson), based on [this paper by Micahel Rodeh](https://ieeexplore.ieee.org/document/1056535) (`src/colfenor_rodeh.rs`).

I'm hoping to add a large number of tests and sufficient benchmarks soon.

## Reference URLs

* https://en.wikipedia.org/wiki/Sardinas%E2%80%93Patterson_algorithm
* https://www.youtube.com/watch?v=SkrLnr-KVOE
* [Dan Hales's blog post](https://towardsdatascience.com/the-sardinas-patterson-algorithm-in-simple-python-9718242752c3)
* https://github.com/creepteks/uniquelyDecodable
* [An interesting-looking paper?](https://core.ac.uk/download/pdf/82304586.pdf)
* [A fast test for unique decipherability based on suffix trees](https://ieeexplore.ieee.org/document/1056535) (paywalled that I can't get through!)
    * Here's [a Rust implementation](https://github.com/Colfenor/sardinas-patterson) supposedly based on the paper.
