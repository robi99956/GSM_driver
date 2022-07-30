use std::convert::From;

#[derive(Debug)]
pub enum GsmModules {
    UC20G,
    EC200T,
    EC200A,
    NotSpecified,
    NotSupported,
}

impl<T: Into<String>> From<Option<T>> for GsmModules {
    fn from(src: Option<T>) -> Self {
        return if let Some(src) = src {
            match src.into().as_str() {
            "UC20G" => GsmModules::UC20G,
            "EC200T" => GsmModules::EC200T,
            "EC200A" => GsmModules::EC200A,
            _ => GsmModules::NotSupported
            }
        } else {
            GsmModules::NotSpecified
        };
    }
}