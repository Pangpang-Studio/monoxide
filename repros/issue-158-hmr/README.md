# issue-158-hmr repro

This repo reproduces:
<https://github.com/Pangpang-Studio/monoxide/issues/158>

## Prerequisites

- Rust toolchain installed
- `dx` (`dioxus-cli`) installed

## Reproduction (quick)

1. Open a terminal in this folder.

2. Start the dev server with hotpatch + verbose logs:

```console
dx serve --verbose --hotpatch --port 3812 \
  -p mini-font \
  --example playground \
  --features=playground \
  --args "serve --port 3032"
```

3. Wait until the first successful build and app launch log appears.

4. Edit `crates/mini-font/src/lib.rs` and change:

```rust
let value = 42_u64;
```

to:

```rust
let value = 43_u64;
```

5. Save the file and watch the `dx serve` logs.

## Expected failure log

You should see lines like:

```console
Patch rebuild: changed_crates=["mini_font"], modified_crates={"mini_font", "mini_monoxide", "playground"}
replaying crates: ["mini_font", "mini_monoxide"]
Build failed: Missing rustc args for replay: 'mini_monoxide'
```
