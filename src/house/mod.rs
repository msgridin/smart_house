use crate::room::Room;
use crate::device::Device;
use crate::error::RoomError;

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

    pub fn room(&self, room_name: &str) -> Result<&Room, RoomError> {
        if let Some(pos) = self.rooms.iter().position(|item| item.name().eq(room_name)) {
            return Ok(&self.rooms[pos]);
        }

        Err(RoomError::NotFound(room_name.to_string()))
    }

    pub fn room_mut(&'_ mut self, room_name: &str) -> Result<&mut Room, RoomError> {
        if let Some(pos) = self.rooms.iter().position(|item| item.name().eq(room_name)) {
            return Ok(&mut self.rooms[pos]);
        }

        Err(RoomError::NotFound(room_name.to_string()))
    }

    pub fn add_room(&mut self, room: Room) -> Result<&Room, RoomError> {
        let room_name = room.name().clone();
        if self.rooms.iter().any(|item| item.name().eq(&room_name)) {
            return Err(RoomError::NotUnique(room.name().clone()));
        };

        self.rooms.push(room);

        self.room(&room_name)
    }

    pub fn remove_room(&mut self, name: &str) -> Result<(), RoomError> {
        if let Some(pos) = self.rooms.iter().position(|item| item.name().eq(&name)) {
            self.rooms.remove(pos);
            return Ok(());
        }

        Err(RoomError::NotFound(name.to_string()))
    }

    pub fn get_device(&self, room_name: &str, device_name: &str) -> Result<&dyn Device, RoomError> {
        Ok(self.room(room_name)?.device(device_name)?)
    }

    pub fn add_device(&mut self, room_name: &str, device: Box<dyn Device>) -> Result<&dyn Device, RoomError> {
        Ok(self.room_mut(room_name)?.add_device(device)?)
    }

    pub fn remove_device(&mut self, room_name: &str, device_name: &str) -> Result<(), RoomError> {
        Ok(self.room_mut(room_name)?.remove_device(device_name)?)
    }

    pub fn device_report() {
        todo!()
    }
}

