use gsm_interface::{GsmAbstract, Result};

pub struct UC20GDriver;

impl GsmAbstract for UC20GDriver {
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