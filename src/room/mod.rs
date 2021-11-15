use crate::device::Device;
use crate::error::CustomError;

pub struct Room<'a> {
    name: String,
    devices: Vec<&'a dyn Device>,
}

impl Room<'_> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.trim().to_string(),
            devices: vec![],
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn devices(&self) -> &Vec<&dyn Device> {
        &self.devices
    }

    pub fn device(&self, device_name: &str) -> Result<&dyn Device, CustomError> {
        if let Some(pos) = self.devices.iter().position(|item| item.name().eq(device_name)) {
            return Ok(self.devices[pos]);
        }

        Err(CustomError::NotFound(format!("room: {}, device: {}", self.name(), device_name)))
    }

    pub fn add_device(&mut self, device: &dyn Device) -> Result<&dyn Device, CustomError> {
        let device_name = device.name().clone();
        if let Some(_) = self.devices.iter().position(|item| item.name().eq(&device_name)) {
            return Err(CustomError::NotUnique(format!("device: {}", device_name)));
        }

        self.devices.push(device);

        self.device(&device_name)
    }

    pub fn remove_device(&mut self, device_name: &str) -> Result<(), CustomError> {
        if let Some(pos) = self.devices.iter().position(|item| item.name().eq(device_name)) {
            self.devices.remove(pos);
            return Ok(());
        }

        return Err(CustomError::NotFound(format!("device: {}", device_name)))

    }
}
