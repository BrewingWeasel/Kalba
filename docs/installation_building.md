# Building from source

Note: building from source requires more technical know-how than the other installation methods.

Dependencies: (for Ubuntu):
`libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf pnpm cargo`

You will also need cargo-tauri installed, which can be achieved with `cargo install tauri-cli`.

Python (with a version over 3.8) is also needed if you plan on using the stanza feature.

To start, clone the repository:

```sh
git clone https://github.com/BrewingWeasel/kalba
cd kalba
```

From there, build the project with cargo-tauri

```sh
cargo tauri build
```
