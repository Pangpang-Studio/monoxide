# monoxide

## Prerequisites

Please make sure you have the following installed on your machine:

- A recent Rust toolchain.
- PNPM for font scripting, WebUI, etc.

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

And if you want to edit the Rust part too:

```console
> cargo xtask dev --watch
```

See `cargo xtask dev --help` for more knobs to tweak.
