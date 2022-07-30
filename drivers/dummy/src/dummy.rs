use gsm_interface::{GsmAbstract, Result};

pub struct DummyGsmModule;

impl GsmAbstract for DummyGsmModule {
    fn probe(&self) -> bool {
        return true;    
    }

    fn connect(&self) -> Result {
        return Result::Ok(());        
    }

    fn disconnect(&self) -> Result {
        return Result::Ok(());
    }
}