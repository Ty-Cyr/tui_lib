# tui_lib

## A library that makes it easier to write Tui Programs automatically configurig the terminal to recieve single characters, and mouse events. As well as displaying formated text.

This project, sets up an interface to interact with the terminal. It is supposed to work on Windows, Linux, and Macos.

### This Project Is Still in Early Development. Breaking changes may still occur.

## Usage

- Never Create two simeltaneous copies of a TuiTerminal. The program will block on the creation of the second due to a mutex.
- Always create the instance of TuiTerminal as mutable. Most methods require a mutable reference.

```rust
    // Create a tui_terminal while remaining in main buffer.
    let mut tui_terminal = TuiTerminal::new(TuiMode::Standard);

    // Create a tui_terminal while switching to alternate buffer.
    let mut tui_terminal = TuiTerminal::new(TuiMode::FullScreen);

    // Output Line To Console
    tui_terminal.println("Hello World!");

    // Output Text To Console
    tui_terminal.print("Hello World!");
    tui_terminal.print("\n");

    // Change Font Settings
    tui_terminal.set_font_color(Color::White);
    tui_terminal.set_background_color(Color::RGB(255, 255, 255));
    tui_terminal.set_bold(ThreeBool::True);
    tui_terminal.set_underlined(ThreeBool::True);
    tui_terminal.set_italics(ThreeBool::True);
    tui_terminal.set_inverted(ThreeBool::True);
    tui_terminal.set_blinking(ThreeBool::True);
    tui_terminal.set_cursor(CursorMode::BlinkingBlock);

    // Output Text with formating
    tui_terminal.println("Hello World!".set_font_color(Color::CC256(255)));
    tui_terminal.println("Hello World!".set_background_color(Color::BrightWhite));
    tui_terminal.println("Hello World!".set_underlined(ThreeBool::True));
    tui_terminal.println("Hello World!".set_italics(ThreeBool::True));
    tui_terminal.println("Hello World!".set_blinking(ThreeBool::True));
```

## Testing

When writing tests for this library be sure to ensure the tests are running on a single thread. Some functions rely on coordinated input and output. Rust's parrellel tests can create race conditions that throw off the results.

## Bug Reports

If you find an issue do not hesitate to report it.
