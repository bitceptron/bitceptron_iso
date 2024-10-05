# bitceptron ISO

This is currently a work in progress. Our target is to build a suite of easy-to-use bitcoin tools concentrated on bridging the gap between technical rigor and usability, via commonsensical abstractions.

We are using [Dioxus](https://github.com/DioxusLabs/dioxus) as our GUI. 

# Collaboration

If you want to be part of this project, please do let us know by sending an email to [bitceptron@gmail.com](mailto:bitceptron@gmail.com).

# Usage

As the first part, we have started with tychentropy. You can throw an n-sided dice where n >= 2 and make entropy for your BIP39 mnemonic. You can mix your physical entropy with RNGs from ring and rand, two leading RNGs in rust ecosystem. Tychentropy also performs statistical tests (those which need at least 128 bits) on your entropy to see if there is any divergance from a random distribution those test can detect (By no means exuhstive).

<img width="1268" alt="Screenshot 1403-07-14 at 09 42 33" src="https://github.com/user-attachments/assets/7463600c-54c0-4c5b-9a34-5b3efd5f6bae">

<img width="1312" alt="Screenshot 1403-07-14 at 09 42 54" src="https://github.com/user-attachments/assets/8ca70847-8c26-4f6d-b3b3-55f015c7ca54">

<img width="1312" alt="Screenshot 1403-07-14 at 09 44 38" src="https://github.com/user-attachments/assets/6abadac8-0a47-4599-a649-bbf3df05631d">

<img width="1312" alt="Screenshot 1403-07-14 at 09 45 17" src="https://github.com/user-attachments/assets/9555e751-40f1-438d-85fd-b0e29fb21e67">


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
5. In `Cargo.toml` edit out the pre-release versioning in `[package]`, `[version]` section, e.g, edit `version = "0.0.0-alpha.1"` to `version = "0.0.0"`.
6. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

7. To build bundled executables run the following command:

```bash
dx bundle --release --platform desktop
```

8. You can find the executables in the following path:

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
