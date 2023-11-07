# storm-os
Very basic OS being created following this blog: https://os.phil-opp.com/

## Setup
If you haven't done this before, you will need to run `rustup install nightly` then `rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu` (will vary depending on the OS you are currently running) for Rust nightly to work.

* Install the `bootimage` tool with `cargo install bootimage`.

* Add the llvm tools rustup component: `rustup component add llvm-tools-preview`

* Run `cargo bootimage` and it should create a bootable disk image.

## Running in QEMU
Run `cargo run`. If it doesn't work, you may need to install QEMU for your system.

If it works, you should see this.

![StormOS working no way](./.screenshots/storm-os.png)

## Optional

If you are using `rust-analyzer` with VSCode, it is recommended to add this to your `settings.json` file in order to avoid errors being shown that aren't actually happening.
```
{
    "rust-analyzer.checkOnSave.allTargets": false,
}
```
