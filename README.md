# monoxide

## Prerequisites

Please make sure you have the following installed on your machine:

- A recent [Rust toolchain](https://rustup.rs) for most of the project.
  - [`dx`](https://crates.io/crates/dioxus-cli) v0.7+ is recommended for
    enabling live programming with `subsecond`-based hotpatching.
    Whenever possible, use the same `dx` version as monoxide's `dioxus-devtools`
    dependency.
- PNPM for the JavaScript part of the project, namely the WebUI.

## Getting Started

Launching the playground is as simple as:

```console
> pnpm i
> cargo xtask dev
```

If you want to develop the WebUI at the same time, run:

```console
> cargo xtask dev --also-webui
```

And if you want to edit the Rust part outside of `monoxide-font`:

```console
> cargo xtask dev --watch
```

See `cargo xtask dev --help` for more knobs to tweak.
