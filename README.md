# monoxide

<p><img src="tools/playground/public/icon.svg" width="120" title="Monoxide Banner"></p>

> A love letter to _[Iosevka]_.

[Iosevka]: https://github.com/be5invis/Iosevka

Monoxide is an experimental project to build a monospace typeface in Rust.

The immediate goal of the project is to create a typeface that can be used to
edit monoxide itself with: after all, who doesn't love the ouroboros?

So far, we have achieved the first working version of:

- A Rust-based eDSL to describe the typeface design.
- A backend that can generate real TTF files.
- A hot-reloadable playground powered by Axum and Vue to enable editing the
  design and previewing the results in real time.
- An expanding set of [glyphs][playground] that will hopefully cover our basic
  programming needs soon™.

[playground]: https://pangpang-studio.github.io/monoxide/

## Prerequisites

Please make sure you have the following installed on your machine:

- A recent [Rust toolchain](https://rustup.rs) for most of the project.
  - [`dx`](https://crates.io/crates/dioxus-cli) v0.7+ is recommended for
    enabling live programming with `subsecond`-based hotpatching.
    Whenever possible, use the same `dx` version as monoxide's `dioxus-devtools`
    dependency.
- PNPM for the JavaScript part of the project, namely the WebUI.

## Development

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

## Distribution

To generate a TTF file from the current design, run:

```console
> cargo run
```

To generate a static playground build for e.g. GitHub Pages, run:

```console
> cargo xtask ssg
```
