#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", deny(warnings))]

#[macro_use]
extern crate failure;
extern crate fruently;
extern crate self_meter;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use failure::Error;
use fruently::fluent::Fluent;
use fruently::forwardable::JsonForwardable;
use self_meter::Meter;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;

#[derive(Debug, Fail)]
enum FluentError {
    #[fail(display = "")] InnerFluentError { e: fruently::error::FluentError },
}

impl From<fruently::error::FluentError> for FluentError {
    fn from(e: fruently::error::FluentError) -> FluentError {
        FluentError::InnerFluentError { e: e }
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(StructOpt, Debug)]
#[structopt(name = "resup", about = "Resources Usage Poller")]
struct MainConfig {
    #[structopt(short = "a", long = "addr",
                default_value = "127.0.0.1:24224", help = "Fruentd hostname")]
    addr: String,

    #[structopt(short = "t", long = "tag",
                help = "Tag to use for Fruentd logging")]
    tag: String,

    #[structopt(parse(try_from_str), short = "i", long = "interval",
                help = "Interval in seconds")]
    interval: u64,
}

fn run_impl(addr: &str, tag: &str) -> Result<()> {
    let fluent = Fluent::new(addr, tag);

    fluent
        .post("")
        .map_err(|e| -> FluentError { e.into() })?;

    Ok(())
}

fn run() -> Result<()> {
    let config = MainConfig::from_args();
    let interval = Duration::from_secs(config.interval);

    let scan_interval: Duration = Duration::from_secs(1);
    let mut meter = Meter::new(scan_interval)?;

    loop {
        if let Err(e) = run_impl(&config.addr, &config.tag) {
            eprintln!("resup run ERROR: {}", e);
        }

        thread::sleep(interval);
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("resup main ERROR: {}", e);
    }
}
