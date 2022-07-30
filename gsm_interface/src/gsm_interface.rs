#[allow(dead_code)]
#[derive(Debug)]
pub enum GsmError {
    Ping,
    NetworkRegister,
    DHCP,
    ModuleCommunication
}

pub type Result = std::result::Result<(), GsmError>;

pub trait GsmAbstract {
    // is module connected to OS
    fn probe(&self) -> bool;
    fn connect(&self) -> Result;
    fn disconnect(&self) -> Result;
}
