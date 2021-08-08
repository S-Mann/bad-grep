# Table of Contents
- [Table of Contents](#table-of-contents)
- [Introduction](#introduction)
- [Installation](#installation)
- [Example](#example)
# Introduction

This is my take on [grep].
I wrote it in a way so that [vscode line navigation] works for grep.
This is grep(without all its features) with one extra feature written in rust.

# Installation

> Note: You will need rust and cargo to build the binaries for your platform.

- You can easily build the binaries using:

```sh
$ git clone git@github.com:S-Mann/bad-grep.git
$ cd ./bad-grep
$ cargo install --path .
```

- Then you can run it using:

```sh
$ bad-grep -q "grep" -f ./README.md
```

# Example

![example](./documentation/bad-grep-example.png)

[grep]: https://man7.org/linux/man-pages/man1/grep.1.html
[vscode line navigation]: https://code.visualstudio.com/docs/getstarted/tips-and-tricks#_navigate-to-a-specific-line
