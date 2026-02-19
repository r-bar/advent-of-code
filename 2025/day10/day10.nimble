# Package

version     = "0.1.0"
author      = "Ryan Barth"
description = "day 10"
license     = "MIT"
srcDir      = "src"
binDir      = "build"
installExt  = @["nim"]
bin         = @["part1", "part2"]


# Dependencies

requires "nim >= 2.2.6"

requires "fusion >= 1.2"
requires "malebolgia >= 1.3.2"
requires "weave >= 0.4.10"