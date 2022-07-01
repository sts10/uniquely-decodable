I'm trying to implement [the Sardinas–Patterson algorithm](https://en.wikipedia.org/wiki/Sardinas%E2%80%93Patterson_algorithm) myself in Rust. My goal is to be able to determine if a given word list is uniquely decodable or not. 

Ultimately I'd like to fold it into [Tidy](https://github.com/sts10/tidy). 

## Some links

https://www.youtube.com/watch?v=8YNEVyHCIjs

https://towardsdatascience.com/the-sardinas-patterson-algorithm-in-simple-python-9718242752c3

https://github.com/Colfenor/sardinas-patterson

## Notes

> As soon as one of the S i {\displaystyle S_{i}} S_{i} contains a word from C or the empty word, then the algorithm terminates and answers that the given code is not uniquely decodable. Otherwise, once a set S i {\displaystyle S_{i}} S_{i} equals a previously encountered set S j {\displaystyle S_{j}} S_j with j < i {\displaystyle j<i} j<i, then the algorithm would enter in principle an endless loop. Instead of continuing endlessly, it answers that the given code is uniquely decodable. 
