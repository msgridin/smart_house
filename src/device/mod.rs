pub trait Device {
    fn name(&self) -> &String;
}

#[derive(PartialEq, Debug)]
pub struct Thermometer {
    name: String,
    pub device_type: DeviceType,
    pub description: String
}
impl Thermometer {
    pub(crate) fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.trim().to_string(),
            device_type: DeviceType::Thermometer,
            description: description.to_string()
        }
    }
}

impl Device for Thermometer {
    fn name(&self) -> &String {
        &self.name
    }
}

#[derive(PartialEq, Debug)]
pub struct Socket {
    name: String,
    pub device_type: DeviceType,
    pub description: String
}
impl Socket {
    pub(crate) fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.trim().to_string(),
            device_type: DeviceType::Socket,
            description: description.to_string()
        }
    }
}

impl Device for Socket {
    fn name(&self) -> &String {
        &self.name
    }
}

#[derive(PartialEq, Debug)]
pub enum DeviceType {
    Thermometer,
    Socket
}
