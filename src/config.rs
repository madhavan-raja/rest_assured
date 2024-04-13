use clap::Args;

#[derive(Args, Clone)]
pub struct Config {
    #[clap(default_value = "rest_assured.yaml")]
    pub file: String,
}
