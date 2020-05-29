# gatrig

`gatrig` is a small command line application used to trigger Github Actions.

## Usage

`gatrig` takes a Github Repository as an argument and sends a dispatch trigger to rerun your Github Actions Workflow.

```bash
$ gatrig joshfinnie/pushfile
Starting event...

Successfully triggered Github Action.
```
```bash
$ gatrig --help
gatrig 0.1.0

USAGE:
    gatrig <repo>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <repo>
```

*Note*: You do need to modify your Github Action workflow to trigger on a dispatch. This is done by adding one to you workflow `.yml`:

```
name: Run on Repo Dispatch
on:
  repository_dispatch:
  push:
...
```

## Installation using binaries

Download the executable from the [Github Releases page](https://github.com/joshfinnie/gatrig/releases), and move it into your path:

```bash
$ wget https://github.com/joshfinnie/gatrig/releases/download/v0.1.2/gatrig_darwin
$ mv gatrig_darwin /usr/local/bin/gatrig
$ chmod +x /usr/local/bin/gatrig
```

## Installation building from source

`gatrig` is built onto of the [Rust programming language](https://www.rust-lang.org/); make sure you have all the prerequrisits installed for Rust before proceeding.
You'll need to clone the repo and then build the code using `cargo`.

```bash
$ git clone https://github.com/joshfinnie/gatrig.git
$ cd gatrig
$ cargo build --release
$ mv target/release/gatrig /usr/local/bin/gatrig
$ chmod +x /usr/local/bin/gatrig
```

## Issues

If you find any issues with `gatrig` please submit a [ticket on Github](https://github.com/joshfinnie/gatrig/issues)

## License

MIT License

Copyright (c) 2020 Josh Finnie

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
