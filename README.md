# Doodle Noodle
Turn your doodles into noodles on Android.\
![Doodle Noodle GIF](/media/doodle_noodle.gif)

## Build Instructions
1. `cargo build` the project for the correct target.
2. Move the compiled binary library to [`godot/lib`](./godot/lib/).
3. Rename the binary to the correct target (names can be found in [`native_library.gdnlib`](./godot/lib/native_library.gdnlib))
4. Enable [Godot Android custom builds](https://docs.godotengine.org/en/stable/tutorials/export/android_custom_build.html).
5. Test the project on Android.

## How does it work?
This project is a Godot 3.5 game that leverages the Rust programming language through GDNative for performance.

When interpreting the image, Rust checks the pixels and creates objects solely based on how bright the pixel is. When a *dark enough* pixel is found, a simple search algorithm is executed that searches for *dark enough* adjacent pixels (a simple grid based search algorithm).

### License
Licensed under the [MIT license](./LICENSE).
