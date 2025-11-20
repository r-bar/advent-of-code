# Advent of Code 2025

https://adventofcode.com/2025

This year's goal is to gain familiarity with the Roc programming language.

## Add a new challenge folder

```sh
export COOKIE=session=53616c7...
just part1
```

This command will automatically create a `README.md` with the prompt(s). 
The challenge input will be saved to `input.txt`.

This command communicates with the Advent of Code servers to fetch this data.
**The given day must be live** before the folder will be able to be templated.
The download also requires your session cookie to fetch your personalized data.

This cookie is fairly long lived and can be extracted from the `Cookie` header
for any request to adventofcode.com after you are logged in. Then set the
environment variable with this session value.

Most of the `just` commands accept the day number as the first argument. This
only needs to be specified if the command is not run on the same day as the
challenge.

## Run the code

Example for day 5, part 1:

```sh
cd day05
roc p1.roc -- input.txt
```

When ready to submit an answer use `just answer`. Continue to part 2 using `just
part2` to update the day's README with the extended prompt.

```sh
just answer $(roc p1.roc -- input.txt)
just part2
```


## Requirements
* [Roc](https://roc-lang.org/) version:
```
roc nightly pre-release, built from commit d73ea109 on Tue 09 Sep 2025 09:02:08 AM UTC
```
* [mise](https://mise.jdx.dev/): optional for tool management
* [just](https://github.com/casey/just)
* curl
* [pup](https://github.com/ericchiang/pup)
* GNU sed
* pandoc

