#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use structopt::StructOpt;

mod core;

#[derive(StructOpt, Debug)]
#[structopt(name = "TT-DNS", about = "Yet another clean dns forwarder")]
struct Opt {
    //#[structopt(short = "u", long = "upstream", default_value = "udp://8.8.8.8:53")]
    #[structopt(short = "u", long = "upstream", default_value = "8.8.8.8:53")]
    UPSTREAM: String,

    //#[structopt(short = "l", long = "listen", default_value = "udp://127.0.0.1:53")]
    #[structopt(short = "l", long = "listen", default_value = "127.0.0.1:53")]
    LISTEN: String,

}

fn main() {
    let opt = Opt::from_args();
    core::run(&opt.LISTEN, &opt.UPSTREAM).unwrap();
}
