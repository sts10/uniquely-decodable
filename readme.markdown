# Checks for unique decodability

This repo is an informal collection of functions that check whether a given list of code words is **uniquely decodabel**.

I _think_ that best way to do this is to implement [the Sardinas–Patterson algorithm](https://en.wikipedia.org/wiki/Sardinas%E2%80%93Patterson_algorithm), but it might not be the only way?

So far this project includes two implementations of Sardinas-Patterson:
* The one Sam Schlinkert wrote for [Tidy](https://github.com/sts10/) (`src/schlinkert.rs`)
* [An implementation by GitHub user @Colfenor](https://github.com/Colfenor/sardinas-patterson), based on [this paper by Micahel Rodeh](https://ieeexplore.ieee.org/document/1056535) (`src/colfenor_rodeh.rs`)

## Running tests
```
cargo test
```

## Running benchmarks
This project uses [Criterion.rs](https://github.com/bheisler/criterion.rs) v0.5 to benchmark.
```
cargo bench
```

## Adding your own procedure/implementation

I 100% welcome and encourage you to contribute to this project, either with your own implementation of Sardinas-Patterson, or some other way of checking unique decodability. See the current modules for requirements.

## Reference URLs

* https://en.wikipedia.org/wiki/Sardinas%E2%80%93Patterson_algorithm
* https://www.youtube.com/watch?v=SkrLnr-KVOE
* [Dan Hales's blog post](https://towardsdatascience.com/the-sardinas-patterson-algorithm-in-simple-python-9718242752c3)
* https://github.com/creepteks/uniquelyDecodable
* [An interesting-looking paper?](https://core.ac.uk/download/pdf/82304586.pdf)
* [A fast test for unique decipherability based on suffix trees](https://ieeexplore.ieee.org/document/1056535) (paywalled that I can't get through!)
    * Here's [a Rust implementation](https://github.com/Colfenor/sardinas-patterson) supposedly based on the paper.
