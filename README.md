# Droll - A CLI Dice Roller for Pen and Paper Games
## How To Install
To compile from source, clone the repository, navigate to the root directory and run
```
cargo build --release
```
## Formatting
Once droll is installed, call it via command line using 'droll' followed by dice roll notation
such as below:
```
droll {number of dice}d{number of faces}*{number of results to display}
```
**Note**: The number of results will default to 1 when not specified. Furthermore, the use of this
argument is for situations where you might need to drop the lowest die. Rather than ```4d6```,
use ```1d6*4``` to display 4 results of 1d6, then drop the lowest of the results.

droll also accepts **multiple arguments**. If you for example ran ```droll 1d20 4d6```, you would get a
list of results with the original argument preceding each result, which would look like:
```
[1d20*1]: [16]
[4d6*1]: [10]
```
## Development
### Dependencies
droll is a small program written in Rust, and its only dependencies are the standard library and fastrand
for generating the results themselves.

### Licensing
This project is licensed under the GNU General Public License v3.0 (GPL-3.0).

You may redistribute and/or modify it under the terms of the GNU General Public License
as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

See the [LICENSE](./LICENSE) file for details.
