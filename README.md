# Data-Racer

## ⚠️ WARNING ⚠️
**This project is in beta and very experimental. You should not use or treat this as a cryptographically secure random number generator.
Everything said in this document beyond this point is a not proven to be true.**

## Overview
Data-racer is an experimental project aimed at generating random numbers using a unique approach based on data races. It leverages the inherent unpredictability of data race conditions in multithreaded environments to generate a random and non-deterministic behaviour. However, it's important to note that this approach is highly experimental and may have security implications that are not yet fully understood.

## Getting Started
To use Data-racer in your project you can add the following lines to your Cargo.toml:

```toml
[dependencies]
data-racer = "*"
```

## Example Usage
To generate a random number you can use the following example:
```rust
use rng::DataraceRNG;

fn main() {

    let mut random_number_generator = DataraceRNG::default(); // Initialize the random number generator and make it mutable
    let random_i32: i32 = random_number_generator.gen(); // Generate a random i32

    println!("You generated the number {}. So random!", random_i32);
}
```

You can also generate almost any kind of primitive type

```rust
use rng::DataraceRNG;

fn main() {
    let mut random_number_generator = DataraceRNG::default(); // Initialize the random number generator and make it mutable

    // Generate (almost) any kind of primitive data
    let random_i32: i32 = random_number_generator.gen();
    let random_u128: u128 = random_number_generator.gen();
    let random_u32: u32 = random_number_generator.gen();
    let random_char: char = random_number_generator.gen();
    let random_bool: bool = random_number_generator.gen();
}
```

You can also fill Slices of (almost) any primitive types!
```rust
use rng::DataraceRNG;

fn main() {
    let mut random_number_generator = DataraceRNG::default(); // Initialize the random number generator and make it mutable

    // Initialize a mutable slice
    let mut char_slice: [char; 128] = ['a'; 128];
    let mut bool_slice: [bool; 128] = [true; 128];
    let mut u32_slice: [u32; 128] = [0; 128];

    random_number_generator.fill(&mut char_slice);
    random_number_generator.fill(&mut bool_slice);
    random_number_generator.fill(&mut u32_slice);

}
```

## Contributing
Please feel free to contribute by creating a pull request to submit the code you would like to be included.

You are very welcome to give us bug fixes and improvements in the form of a [GitHub Pull Request](https://github.com/victoralan2/data-racer/pulls).

## License
This project is licensed under the MIT License - see the [LICENSE](https://github.com/victoralan2/data-racer/blob/master/LICENSE.txt) file for details.

