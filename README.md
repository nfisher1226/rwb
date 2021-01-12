# RWB
Web browser written in Rust using a webkit2gtk interface. Minimal keyboard
driven interface.

This is an early WIP. It is usable for browsing the web but lacks the ability
to handle downloads, keep cookies, use any custom settings, etc.

### Building
#### Requirements
You will need a recent Rust toolchain including Cargo. You will also need the
gtk+ and webkit2gtk libraries installed. Building is via cargo:
```cargo build --release```
After building, just copy the executable to somewhere in your path. If any of
this does not make sense to you, then you are probably not going to enjoy this
browser anyway...

### Keyboard shortcuts
* Ctrl-O - open url
* Ctrl-Shift-O - open url in new tab
* Ctrl-H - go back
* Ctrl-L - go forward
* Ctrl-T - new tab with default url
* Ctrl-N - new blank tab
* Ctrl-Shift-J - next tab
* Ctrl-Shift-K - previous tab
* Ctrl-W - close tab
* Ctrl-Q - close browser
