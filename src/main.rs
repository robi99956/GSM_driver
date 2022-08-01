mod app_config;
mod hw_selector;
mod power_controller;

use crate::app_config::AppConfig;
use crate::hw_selector::GsmModules;
use crate::power_controller::{PowerController, TogglePowerController};
use dummy::DummyGsmModule;
use gsm_interface::GsmAbstract;

use std::num::Wrapping;

use gpio_cdev::{LineRequestFlags};

type SessionId = u32;

enum GsmMessageCmd {
    OpenSession(u32),
    KeepAliveSession(SessionId),
    CloseSession(SessionId),
    IsAlive
}

enum GsmMessageAnswer {
    SessionOpened(SessionId),
    Success,
    Fail
}

#[derive(Clone, Copy)]
struct GsmSession {
    id: SessionId,
    ttl_s: u32
}

struct ManagedGSM<'a> {
    module: &'a (dyn GsmAbstract + 'a),
    power: &'a (dyn PowerController + 'a),

    current_sessions: Vec<GsmSession>,
    next_session_id: SessionId
}

impl<'a> ManagedGSM<'a> {
    pub fn new(module: &'a (impl GsmAbstract + 'a), power: &'a (impl PowerController + 'a)) -> Self {
        Self{
            module, power, 
            current_sessions: Vec::new(),
            next_session_id: 0
        }
    }

    fn new_session(&mut self, ttl_s: u32) -> Option<GsmSession> {
        let id = self.next_session_id;
        self.next_session_id = (Wrapping(self.next_session_id) + Wrapping(1)).0;
        return Some(GsmSession {id, ttl_s});
    }

    pub fn process_command(&mut self, cmd: GsmMessageCmd) -> GsmMessageAnswer {
        return match cmd {
            GsmMessageCmd::OpenSession(ttl_s) => {
                match self.new_session(ttl_s) {
                    Some(session) => {
                        self.current_sessions.push(session);
                        GsmMessageAnswer::SessionOpened(session.id)
                    },
                    None => GsmMessageAnswer::Fail
                }
            },
            GsmMessageCmd::KeepAliveSession(id) => {
                match self.current_sessions.iter_mut().find(|s| s.id == id) {
                    Some(s) => {
                        s.ttl_s += 10;
                        GsmMessageAnswer::Success
                    },
                    None => GsmMessageAnswer::Fail
                }
            },
            GsmMessageCmd::CloseSession(id) => {
                self.current_sessions.retain(|s| s.id != id);
                GsmMessageAnswer::Success
            },
            GsmMessageCmd::IsAlive => {
                if self.current_sessions.is_empty() {
                    GsmMessageAnswer::Fail
                } else {
                    GsmMessageAnswer::Success
                }
            }
        };
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

    let mut device = ManagedGSM::new(&gsm, &power_controller);
    device.process_command(GsmMessageCmd::OpenSession(10));

}