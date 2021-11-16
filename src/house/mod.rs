use crate::room::Room;
use crate::device::Device;
use crate::error::CustomError;

pub struct House {
    name: String,
    rooms: Vec<Room>,
}

impl House {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.trim().to_string(),
            rooms: vec![],
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn rooms(&self) -> &Vec<Room> {
        &self.rooms
    }

    pub fn room(&self, room_name: &str) -> Result<&Room, CustomError> {
        if let Some(pos) = self.rooms.iter().position(|item| item.name().eq(room_name)) {
            return Ok(&self.rooms[pos]);
        }

        Err(CustomError::NotFound(format!("room: {}", room_name)))
    }

    pub fn room_mut(&'_ mut self, room_name: &str) -> Result<&mut Room, CustomError> {
        if let Some(pos) = self.rooms.iter().position(|item| item.name().eq(room_name)) {
            return Ok(&mut self.rooms[pos]);
        }

        Err(CustomError::NotFound(format!("room: {}", room_name)))
    }

    pub fn add_room(&mut self, room: Room) -> Result<&Room, CustomError> {
        let room_name = room.name().clone();
        if let Some(_) = self.rooms.iter().position(|item| item.name().eq(&room_name)) {
            return Err(CustomError::NotUnique(format!("room: {}", room.name())));
        }

        self.rooms.push(room);

        self.room(&room_name)
    }

    pub fn remove_room(&mut self, name: &str) -> Result<(), CustomError> {
        if let Some(pos) = self.rooms.iter().position(|item| item.name().eq(&name)) {
            self.rooms.remove(pos);
            return Ok(());
        }

        return Err(CustomError::NotFound(format!("room: {}", name)));
    }

    pub fn get_device(&self, room_name: &String, device_name: &String) -> Result<&dyn Device, CustomError> {
        self.room(room_name)?.device(device_name)
    }

    pub fn add_device(&mut self, room_name: &String, device: Box<dyn Device>) -> Result<&dyn Device, CustomError> {
        self.room_mut(room_name)?.add_device(device)
    }

    pub fn remove_device(&mut self, room_name: &String, device_name: &String) -> Result<(), CustomError> {
        self.room_mut(room_name)?.remove_device(device_name)
    }

    pub fn device_report() {
        todo!()
    }
}

