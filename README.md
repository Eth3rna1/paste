# Paste

A Rust CLI tool for quick pasting text into a file

## Getting Started
Usage requires for the client to provide a unique file name (`<FILE>`) to write into. After which the client must specify a character (`<BREAK CHAR>`) that must be used at the end of the line, relative to its use.

### Example
Let's say the `<BREAK CHAR>` specified is ".";
```text
54: this is the last line I want to use .
                                        ^
                                        |
                        Character being used; signifies the end.
```
Ultimately, `<BREAK CHAR>` can be any character you please.
