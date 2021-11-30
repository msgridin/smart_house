use std::any::Any;

pub trait Device {
    fn name(&self) -> &String;
    fn device_type(&self) -> &DeviceType;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(PartialEq, Debug)]
pub struct Thermometer {
    name: String,
    device_type: DeviceType,
    pub description: String,
}

impl Thermometer {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.trim().to_string(),
            device_type: DeviceType::Thermometer,
            description: description.to_string(),
        }
    }

    pub fn get_temperature(&self) -> f32 {
        24.
    }
}

impl Device for Thermometer {
    fn name(&self) -> &String {
        &self.name
    }

    fn device_type(&self) -> &DeviceType {
        &self.device_type
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(PartialEq, Debug)]
pub struct Socket {
    name: String,
    device_type: DeviceType,
    pub description: String,
    switcher: bool
}

impl Socket {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.trim().to_string(),
            device_type: DeviceType::Socket,
            description: description.to_string(),
            switcher: false
        }
    }

    pub fn on(&mut self) {
        self.switcher = true;
    }

    pub fn off(&mut self) {
        self.switcher = false;
    }

    pub fn switch(&self) -> bool {
        self.switcher
    }

    pub fn get_capacity(&self) -> i32 {
        500
    }
}

impl Device for Socket {
    fn name(&self) -> &String {
        &self.name
    }

    fn device_type(&self) -> &DeviceType {
        &self.device_type
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(PartialEq, Debug)]
pub enum DeviceType {
    Thermometer,
    Socket,
}
