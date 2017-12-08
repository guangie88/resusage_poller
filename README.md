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
{
  "count": 6,
  "avg_busy": 0.16698907,
  "avg_idle": 0.8330109,
  "cpu_loads": {
    "5": {
      "index": 5,
      "user": 0.1010101,
      "nice": 0.0,
      "system": 0.01010101,
      "interrupt": 0.0,
      "busy": 0.11111111,
      "idle": 0.8888889
    },
    "3": {
      "index": 3,
      "user": 0.1632653,
      "nice": 0.0,
      "system": 0.010204081,
      "interrupt": 0.0,
      "busy": 0.17346938,
      "idle": 0.82653064
    },
    "1": {
      "index": 1,
      "user": 0.2020202,
      "nice": 0.0,
      "system": 0.02020202,
      "interrupt": 0.0,
      "busy": 0.22222222,
      "idle": 0.7777778
    },
    "0": {
      "index": 0,
      "user": 0.13725491,
      "nice": 0.0,
      "system": 0.029411765,
      "interrupt": 0.0,
      "busy": 0.16666667,
      "idle": 0.8333333
    },
    "2": {
      "index": 2,
      "user": 0.1388889,
      "nice": 0.0,
      "system": 0.055555556,
      "interrupt": 0.0,
      "busy": 0.19444445,
      "idle": 0.8055556
    },
    "4": {
      "index": 4,
      "user": 0.13402061,
      "nice": 0.0,
      "system": 0.0,
      "interrupt": 0.0,
      "busy": 0.13402061,
      "idle": 0.8659794
    }
  }
}
```
