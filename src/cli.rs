use clap::Parser;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[clap(version = VERSION, about = "", long_about = None)]
pub struct Args {
    #[clap(short = 'w', long = "window_size")]
    pub duration: Option<u32>,
    #[clap(short='u', long="user")]
    pub user: Option<String>,
    #[clap(long="host")]
    pub host: Option<String>,
    #[clap(short='d', long="domain")]
    pub domain: Option<String>
}
