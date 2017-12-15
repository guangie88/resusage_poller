# resusage_poller

This is an experimental project.

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
./target/x86_64-unknown-linux-musl/release/resup -i 1s -t elastic.rs
```

Note the program loops forever until `CTRL-C` is pressed.

## JSON Log Example

```json
{
  "count": 6,
  "avg_busy_perc": 19.00171,
  "avg_idle_perc": 80.833275,
  "cpu_loads": {
    "4": {
      "index": 4,
      "user": 0.16,
      "nice": 0.0,
      "system": 0.01,
      "interrupt": 0.0,
      "busy": 0.17,
      "idle": 0.83
    },
    "1": {
      "index": 1,
      "user": 0.11881188,
      "nice": 0.0,
      "system": 0.01980198,
      "interrupt": 0.0,
      "busy": 0.13861386,
      "idle": 0.8613861
    },
    "0": {
      "index": 0,
      "user": 0.16190477,
      "nice": 0.0,
      "system": 0.028571429,
      "interrupt": 0.0,
      "busy": 0.1904762,
      "idle": 0.8095238
    },
    "3": {
      "index": 3,
      "user": 0.24752475,
      "nice": 0.0,
      "system": 0.02970297,
      "interrupt": 0.0,
      "busy": 0.27722773,
      "idle": 0.7128713
    },
    "2": {
      "index": 2,
      "user": 0.15841584,
      "nice": 0.0,
      "system": 0.01980198,
      "interrupt": 0.0,
      "busy": 0.17821781,
      "idle": 0.8217822
    },
    "5": {
      "index": 5,
      "user": 0.16494845,
      "nice": 0.0,
      "system": 0.020618556,
      "interrupt": 0.0,
      "busy": 0.185567,
      "idle": 0.814433
    }
  }
}
```
