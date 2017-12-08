# resusage_poller

`Rust` executable to poll for CPU usage (per core) based on parsing of
`/proc/stat` and logs to `Fluentd` unified logging layer.

`resup` is the executable short form for Resource Usage Poller.

Requires Linux and supports compilation on stable `rustc`, and also works for
target `x86_64-unknown-linux-musl`.

## Installation of `Rust` environment

Follow the instructions from [`rustup`](https://www.rustup.rs/). In order to
build for target `x86_64-unknown-linux-musl`, the following command should be
run **after** the installation of the toolchain:

```bash
rustup target add x86_64-unknown-linux-musl
```

The benefit building for target `x86_64-unknown-linux-musl` is that the compiled
executable is fully statically linked, _i.e_. does not depend on any system
shared objects and is very portable across all Linux distributions.

## Recommended Build Command

```bash
cargo build --release --target x86_64-unknown-linux-musl
```

## Run Example

Polls for:

* (Default `Fluentd` server at: `127.0.0.1:24224`)
* Interval: 1 second
* `Fluentd` tag: `elastic.rs`

```bash
./target/x86_64-unknown-linux-musl/release/resup -i 1 -t elastic.rs
```

Note the program loops forever until `CTRL-C` is pressed.

## JSON Log Example

```json
"cpu_loads": {
    "0": {
        "user": 0.020408163,
        "nice": 0,
        "system": 0.010204081,
        "interrupt": 0,
        "idle": 0.96938777
    },
    ...
}
```
