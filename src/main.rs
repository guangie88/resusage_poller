#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", deny(warnings))]

#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate failure;
extern crate fruently;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate systemstat;

use failure::Error;
use fruently::fluent::Fluent;
use fruently::forwardable::JsonForwardable;
use std::thread;
use std::collections::HashMap;
use std::time::Duration;
use structopt::StructOpt;
use systemstat::{CPULoad, Platform};
use systemstat::platform::PlatformImpl;

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

#[derive(Serialize, Deserialize, Debug)]
struct CpuLoadDef {
    user: f32,
    nice: f32,
    system: f32,
    interrupt: f32,
    idle: f32,
}

impl CpuLoadDef {
    fn from_cpu_load(c: &CPULoad) -> CpuLoadDef {
        CpuLoadDef {
            user: c.user,
            nice: c.nice,
            system: c.system,
            interrupt: c.interrupt,
            idle: c.idle,
        }
    }
}

type CpuLoadDefs = HashMap<usize, CpuLoadDef>;

#[derive(Serialize, Deserialize, Debug, new)]
struct CpuLoadWrap {
    cpu_loads: CpuLoadDefs,
}

fn run_impl(
    pf: &PlatformImpl,
    addr: &str,
    tag: &str,
    interval: Duration,
) -> Result<()> {
    let cpu_loads = pf.cpu_load()?;

    // required to sleep before .done() is invoked
    thread::sleep(interval);
    let cpu_loads = cpu_loads.done()?;

    let cpu_loads: HashMap<usize, CpuLoadDef> = cpu_loads
        .into_iter()
        .enumerate()
        .map(|(i, cpu_load)| (i, CpuLoadDef::from_cpu_load(&cpu_load)))
        .collect();

    Fluent::new(addr, tag)
        .post(&CpuLoadWrap::new(cpu_loads))
        .map_err(|e| -> FluentError { e.into() })?;

    Ok(())
}

fn run() -> Result<()> {
    let config = MainConfig::from_args();
    let interval = Duration::from_secs(config.interval);
    let pf = PlatformImpl::new();

    loop {
        if let Err(e) = run_impl(&pf, &config.addr, &config.tag, interval) {
            eprintln!("resup run ERROR: {}", e);
        }
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("resup main ERROR: {}", e);
    }
}
