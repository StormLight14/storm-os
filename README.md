# storm-os
Very basic OS being created following this blog: https://os.phil-opp.com/

## Setup
If you haven't done this before, you will need to run `rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu` (will vary depending on the OS you are currently running) for Rust nightly to work.

Add `bootloader` as a dependency with `cargo add bootloader`, then install `bootimage` with `cargo install bootimage`.

Run `cargo build` and it should work with no errors.

If you are using `rust-analyzer` with VSCode, it is recommended to add this to your `settings.json` file in order to avoid errors being shown that aren't actually happening.
```
{
    "rust-analyzer.checkOnSave.allTargets": false,
}
```

