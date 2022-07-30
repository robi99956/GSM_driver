mod app_config;
mod hw_selector;
mod power_controller;

use crate::app_config::AppConfig;
use crate::hw_selector::GsmModules;
use crate::power_controller::{PowerController, TogglePowerController};
use dummy::DummyGsmModule;
use gsm_interface::GsmAbstract;

use gpio_cdev::{LineRequestFlags};

struct ManagedGSM<'a> {
    module: Box<dyn GsmAbstract + 'a>,
    power: Box<dyn PowerController + 'a>
}

impl<'a> ManagedGSM<'a> {
    pub fn new(module: impl GsmAbstract + 'a, power: impl PowerController + 'a) -> Self {
        Self{
            module: Box::new(module),
            power: Box::new(power)
        }
    }
}

fn main() {
    let config = AppConfig::new_from_cli_args();
    println!("config = {:?}", config); 
    println!("module = {:?}", GsmModules::from(config.device_type));

    let mut chip = gpio_cdev::Chip::new("/dev/gpiochip0").unwrap();
    let line = chip
        .get_line(0)
        .unwrap()
        .request(LineRequestFlags::OUTPUT, 0, "GSM")
        .unwrap();
    let gsm= DummyGsmModule;

    let power_controller = TogglePowerController::new(line, || {
        gsm.probe()
    }, 500);

    gsm.connect().unwrap();

    power_controller.set_power(false);
}