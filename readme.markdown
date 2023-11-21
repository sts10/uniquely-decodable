# Checks for unique decodability

This repo is an informal collection of functions that (attempt to) check whether a given list of code words is **uniquely decodable**.

**Note**: If you're **just looking to check if a word list is uniquely decodable**, I'd point you to my [Word List Auditor](https://github.com/sts10/wla) tool.

## A hypothesis
I _think_ that best way to do check if a list is uniquely decodable is to implement [the Sardinas–Patterson algorithm](https://en.wikipedia.org/wiki/Sardinas%E2%80%93Patterson_algorithm), but it might not be the only way?

## Included implementations of Sardinas-Patterson
So far this project includes two implementations of Sardinas-Patterson:
* The one Sam Schlinkert wrote for [Tidy](https://github.com/sts10/) (`src/schlinkert.rs`)
* [An implementation by GitHub user @Colfenor](https://github.com/Colfenor/sardinas-patterson), apparently based on [this paper by Michael Rodeh](https://ieeexplore.ieee.org/document/1056535) (`src/colfenor_rodeh.rs`). Seems to be very fast detecting if a list is not uniquely decodable, but struggles with longer (8,000-word) uniquely decodable lists?

## Running tests
```
cargo test
```

## Running benchmarks
This project uses [Criterion.rs](https://github.com/bheisler/criterion.rs) v0.5 to benchmark.
```
cargo bench
```
FYI running all benchmarks can take a few minutes, depending on which are commented in and out.

## Adding your own procedure/implementation

I 100% welcome and encourage you to contribute to this project, either with your own implementation of Sardinas-Patterson, or some other way of checking for unique decodability. See the current modules for requirements.

## Potentially helpful links

* [Sardinas-Patterson algorithm Wikipedia page](https://en.wikipedia.org/wiki/Sardinas%E2%80%93Patterson_algorithm)
* [A YouTube video explaining Sardinas-Patterson](https://www.youtube.com/watch?v=SkrLnr-KVOE)
* [A blog post about implementation Sardinas-Patterson in Python](https://towardsdatascience.com/the-sardinas-patterson-algorithm-in-simple-python-9718242752c3)
* [An implementation of Sardinas-Patterson in C++](https://github.com/creepteks/uniquelyDecodable)
* [An interesting-looking paper?](https://core.ac.uk/download/pdf/82304586.pdf)
