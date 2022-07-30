use gpio_cdev::LineHandle;
use std::time::Duration;
use std::thread::sleep;

pub trait PowerController {
    fn set_power(&self, state: bool);
}

pub struct DummyPowerController;

impl PowerController for DummyPowerController {
    fn set_power(&self, _state: bool) {
        
    }
}

pub struct SimplePowerController {
    gpio: LineHandle
}

impl SimplePowerController {
    pub fn new(gpio: LineHandle) -> Self {
        Self { gpio }
    }
}

impl PowerController for SimplePowerController {
    fn set_power(&self, state: bool) {
        let _ = self.gpio.set_value(state as u8);
    }
}

// Heresy, but some shields for RPi does this
pub struct TogglePowerController<'a> {
    gpio: LineHandle,
    get_state: Box<dyn Fn() -> bool + 'a>, 
    toggle_duration_ms: u64
}

impl<'a> TogglePowerController<'a> {
    pub fn new(gpio: LineHandle, get_state: impl Fn() -> bool + 'a, toggle_duration_ms: u64) -> Self {
        Self {gpio, get_state: Box::new(get_state), toggle_duration_ms}
    }
}

impl PowerController for TogglePowerController<'_> {
    fn set_power(&self, state: bool) {
        if state != (self.get_state)() {
            let _ = self.gpio.set_value(1);
            sleep(Duration::from_millis(self.toggle_duration_ms));
            let _ = self.gpio.set_value(0);
        }
    }
}