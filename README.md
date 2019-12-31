selection_sort [![Build Status](https://travis-ci.org/dmjio/selection_sort.svg?branch=master)](https://travis-ci.org/dmjio/selection_sort) ![crates.io](https://img.shields.io/crates/v/selection_sort.svg)
=============================

[Selection Sort](https://en.wikipedia.org/wiki/Selection_sort) implemented in [Rust](https://www.rust-lang.org/)

```rust
use selection_sort::selection_sort;

fn main () {
  let mut vec = vec![1, 3, 2];
  selection_sort(&mut vec);
  println!("{:?}", vec);
  assert!(vec == [1, 2, 3]);
}
// [1,2,3]
```
