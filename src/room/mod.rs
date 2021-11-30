use crate::device::Device;
use crate::error::DeviceError;

pub struct Room {
    name: String,
    devices: Vec<Box<dyn Device>>,
}

impl Room {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.trim().to_string(),
            devices: vec![],
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn devices(&self) -> &Vec<Box<dyn Device>> {
        &self.devices
    }

    pub fn device(&self, device_name: &str) -> Result<&dyn Device, DeviceError> {
        if let Some(pos) = self.devices.iter().position(|item| item.name().eq(device_name)) {
            return Ok(&*self.devices[pos]);
        }

        Err(DeviceError::NotFound(device_name.to_string(), self.name.to_string()))
    }

    pub fn device_mut(&mut self, device_name: &str) -> Result<&mut dyn Device, DeviceError> {
        if let Some(pos) = self.devices.iter().position(|item| item.name().eq(device_name)) {
            return Ok(&mut *self.devices[pos]);
        }

        Err(DeviceError::NotFound(device_name.to_string(), self.name.to_string()))
    }

    pub fn add_device(&mut self, device: Box<dyn Device>) -> Result<&dyn Device, DeviceError> {
        let device_name = device.name().clone();
        if self.devices.iter().any(|item| item.name().eq(&device_name)) {
            return Err(DeviceError::NotUnique(device_name, self.name.to_string()));
        }

        self.devices.push(device);

        self.device(&device_name)
    }

    pub fn remove_device(&mut self, device_name: &str) -> Result<(), DeviceError> {
        if let Some(pos) = self.devices.iter().position(|item| item.name().eq(device_name)) {
            self.devices.remove(pos);
            return Ok(());
        }

        Err(DeviceError::NotFound(device_name.to_string(), self.name.to_string()))

    }
}
