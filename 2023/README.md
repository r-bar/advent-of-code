# Advent of Code 2023

https://adventofcode.com/2023

Days will have implementations in Rust, Python, or both depending how much time
I had that day. The goal is to get quicker at prototyping in Rust for small
command line applications like this.

## Add a new challenge folder
```
export COOKIE=session=53616c7...
just template-day 5
```
This command will automatically create a `README.md` with the prompt(s). 
The challenge input will be saved to `input.txt`.

This command communicates with the Advent of Code servers to fetch this data.
**The given day must be live** before the folder will be able to be templated.
The download also requires your session cookie to fetch your personalized data.

This cookie is fairly long lived and can be extracted from the `Cookie` header
for any request to adventofcode.com after you are logged in. The set the
`COOKIE` environment variable with this session value

Requirements:
* [just](https://github.com/casey/just)
* curl
* [pup](https://github.com/ericchiang/pup)
* GNU sed
* pandoc

## Run the Rust code (day 5, part 1)
```
cd day05
cargo run --bin d5p1 -- [arg] ...
```

## Requirements
* Python 3.11
* Rust 2021 (1.64.0)
