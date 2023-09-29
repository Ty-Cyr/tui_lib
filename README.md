# tui_lib

## A library that makes it easier to write Tui Programs automatically configurig the terminal to recieve single characters, and mouse events. As well as displaying formated text.

This project, sets up an interface to interact with the terminal. It is supposed to work on Windows, Linux, and Macos.

`let x = y;`

## Testing

When writing tests for this library be sure to ensure the tests are running on a single thread. Some functions rely on coordinated input and output. Rust's parrellel tests can create race conditions that throw off the results.
