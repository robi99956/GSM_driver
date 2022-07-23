mod app_config;
use crate::app_config::AppConfig;

fn main() {
    let config = AppConfig::new_from_cli_args();
    println!("config = {:?}", config); 
}