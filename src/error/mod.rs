use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum LibError {
    RoomError(RoomError),
}

impl Error for LibError {}

impl From<RoomError> for LibError {
    fn from(err: RoomError) -> Self {
        LibError::RoomError(err)
    }
}

impl Display for LibError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let cause = match self {
            LibError::RoomError(err) => { format!("Room error. {}", err) }
        };
        write!(f, "({})", cause)
    }
}

#[derive(Debug)]
pub enum RoomError {
    NotFound(String),
    NotUnique(String),
    DeviceError(DeviceError),
}

impl Error for RoomError {}

impl From<DeviceError> for RoomError {
    fn from(err: DeviceError) -> Self {
        RoomError::DeviceError(err)
    }
}

impl Display for RoomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let string = match self {
            RoomError::NotFound(name) => { format!("Not found {}", name) }
            RoomError::NotUnique(name) => { format!("Not unique {}", name) }
            RoomError::DeviceError(err) => { format!("{}", err) }
        };
        write!(f, "({})", string)
    }
}

#[derive(Debug)]
pub enum DeviceError {
    NotFound(String, String),
    NotUnique(String, String),
}

impl Error for DeviceError {}

impl Display for DeviceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let string = match self {
            DeviceError::NotFound(device_name, room_name) => { format!("Device {} not found in room {}", device_name, room_name) }
            DeviceError::NotUnique(device_name, room_name) => { format!("Device {} not unique in room {}", device_name, room_name) }
        };
        write!(f, "({})", string)
    }
}

