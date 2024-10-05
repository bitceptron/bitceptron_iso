# bitceptron ISO

This is currently a work in progress. Our target is to build a suite of easy-to-use bitcoin tools concentrated on bridging the gap between technical rigor and usability, via commonsensical abstractions.

# Build from source

## Prerequisites

1. Install rust if you have not already: https://www.rust-lang.org/tools/install
2. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
3. Install the tailwind css cli: https://tailwindcss.com/docs/installation

## MacOS and Linux

1. Install dioxus-cli by running the following command:

```bash
cargo install dioxus-cli@0.6.0-alpha.2
```

2. Download this repository from github.
3. Enter into the downloaded repository's root folder.
4. Copy the contents of `Dioxus_mac_linux.toml` into `Dioxus.toml`.
5. Open a terminal and run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

6. To build bundled executables, open another terminal in the project root and run the following command:

```bash
dx bundle --release --platform desktop
```

7. You can find the executables in the following path:

```bash
[repo root]/dist/bundle/
```

## Windows

1. Install dioxus-cli by running the following command:

```bash
cargo install dioxus-cli@0.5.4
```

2. Download this repository from github.
3. Enter into the downloaded repository's root folder.
4. Copy the contents of `Dioxus_windows.toml` into `Dioxus.toml`.
5. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

6. To build bundled executables run the following command:

```bash
dx bundle --release --platform desktop
```

7. You can find the executables in the following path:

```bash
[repo root]/dist/bundle/
```

# Development

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the tailwind css cli: https://tailwindcss.com/docs/installation
3. Open a terminal and run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

4. Open another terminal in the project root and run the following command:

```bash
dx serve --hot-reload --platform desktop
```
