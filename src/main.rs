#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", deny(warnings))]

#[macro_use]
extern crate failure;
extern crate fruently;
extern crate humantime;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate systemstat;

use failure::Error;
use fruently::fluent::Fluent;
use fruently::forwardable::JsonForwardable;
use humantime::Duration;
use std::collections::HashMap;
use std::thread;
use structopt::StructOpt;
use systemstat::{CPULoad, Platform};
use systemstat::platform::PlatformImpl;

#[derive(Debug, Fail)]
enum FluentError {
    #[fail(display = "")]
    InnerFluentError { e: fruently::error::FluentError },
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
    #[structopt(short = "a", long = "addr", default_value = "127.0.0.1:24224",
                help = "Fluentd hostname")]
    addr: String,

    #[structopt(long = "off", help = "Turn off Fluentd logging")]
    fluent_off: bool,

    #[structopt(short = "t", long = "tag",
                help = "Tag to use for Fruentd logging")]
    tag: String,

    #[structopt(parse(try_from_str), short = "i", long = "interval",
                help = "Interval to get resource usage")]
    interval: Duration,
}

#[derive(Serialize, Deserialize, Debug)]
struct FlattenedCpuLoad {
    index: usize,
    user: f32,
    nice: f32,
    system: f32,
    interrupt: f32,
    busy: f32,
    idle: f32,
}

impl FlattenedCpuLoad {
    fn from_cpu_load_with_index(index: usize, c: &CPULoad) -> FlattenedCpuLoad {
        FlattenedCpuLoad {
            index: index,
            user: c.user,
            nice: c.nice,
            system: c.system,
            interrupt: c.interrupt,
            busy: c.user + c.nice + c.system,
            idle: c.idle,
        }
    }
}

type FlattenedCpuLoads = HashMap<usize, FlattenedCpuLoad>;

#[derive(Serialize, Deserialize, Debug)]
struct CpuLoadWrap {
    count: usize,
    avg_busy_perc: f32,
    avg_idle_perc: f32,
    cpu_loads: FlattenedCpuLoads,
}

impl CpuLoadWrap {
    fn from_cpu_load_defs(cpu_load_defs: FlattenedCpuLoads) -> CpuLoadWrap {
        CpuLoadWrap {
            count: cpu_load_defs.len(),
            avg_busy_perc: cpu_load_defs.values().fold(
                0.0,
                |acc, c| acc + c.busy,
            ) / cpu_load_defs.len() as f32 * 100.0,
            avg_idle_perc: cpu_load_defs.values().fold(
                0.0,
                |acc, c| acc + c.idle,
            ) / cpu_load_defs.len() as f32 * 100.0,
            cpu_loads: cpu_load_defs,
        }
    }
}

fn run_impl(
    pf: &PlatformImpl,
    addr: &str,
    tag: &str,
    interval: Duration,
    fluent_off: bool,
) -> Result<()> {
    let cpu_loads = pf.cpu_load()?;

    // required to sleep before .done() is invoked
    thread::sleep(*interval);
    let cpu_loads = cpu_loads.done()?;

    let cpu_loads: HashMap<usize, FlattenedCpuLoad> = cpu_loads
        .into_iter()
        .enumerate()
        .map(|(i, cpu_load)| {
            (i, FlattenedCpuLoad::from_cpu_load_with_index(i, &cpu_load))
        })
        .collect();

    let cpu_load_wrap = CpuLoadWrap::from_cpu_load_defs(cpu_loads);

    if cfg!(debug_assertions) {
        println!("{}", serde_json::to_string_pretty(&cpu_load_wrap)?);
    }

    if !fluent_off {
        Fluent::new(addr, tag).post(&cpu_load_wrap).map_err(
            |e| -> FluentError { e.into() },
        )?;
    }

    Ok(())
}

fn run() -> Result<()> {
    let config = MainConfig::from_args();
    let pf = PlatformImpl::new();

    loop {
        if let Err(e) = run_impl(
            &pf,
            &config.addr,
            &config.tag,
            config.interval,
            config.fluent_off,
        )
        {
            eprintln!("resup run ERROR: {}", e);
        }
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("resup main ERROR: {}", e);
    }
}
