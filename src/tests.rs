use super::*;
use house::House;
use room::Room;
use device::{Device, DeviceType};

#[test]
// 1) Дом имеет название и содержит несколько помещений.
fn test_1() {
    let house = House::new("My house");

    assert_eq!(house.name(), "My house");
    assert_eq!(house.rooms().len(), 0);
}

#[test]
// 2) Библиотека позволяет запросить список помещений,
// добавлять и удалять помещения в доме.
fn test_2() {
    let mut house = House::new("My house");

    let room = Room::new("First room");
    let _ = house.add_room(room);
    let room = Room::new("Second room");
    let _ = house.add_room(room);
    let room = Room::new("Third room");
    let _ = house.add_room(room);

    let _ = house.remove_room("Second room");

    assert_eq!(house.rooms().len(), 2);
    assert_eq!(house.room("First room").unwrap(), &Room::new("First room"));
    assert_eq!(house.room("Third room").unwrap(), &Room::new("Third room"));
}

#[test]
// 3) Помещение имеет уникальное название
// и содержит несколько устройств.
fn test_3() {
    let mut house = House::new("My house");

    let room = Room::new("First room");
    let _ = house.add_room(room);
    let room = Room::new("First room");
    if let Ok(_) = house.add_room(room) {
        panic!();
    }

    let device = Device::new("First device", DeviceType::Thermometer, "Thermometer");
    let _ = house.room_mut("First room").unwrap().add_device(device);
    let device = Device::new("Second device", DeviceType::Socket, "Socket");
    let _ = house.room_mut("First room").unwrap().add_device(device);

    assert_eq!(house.room("First room").unwrap().devices().len(), 2);
}

#[test]
// 4) Устройство имеет уникальное в рамках помещения название.
fn test_4() {
    let mut house = House::new("My house");

    let room = Room::new("First room");
    let _ = house.add_room(room);

    let device = Device::new("First device", DeviceType::Thermometer, "Thermometer");
    let _ = house.room_mut("First room").unwrap().add_device(device);
    let device = Device::new("First device", DeviceType::Socket, "Socket");
    if let Ok(_) = house.room_mut("First room").unwrap().add_device(device) {
        panic!();
    }
    assert_eq!(house.room("First room").unwrap().devices().len(), 1);
}

#[test]
// 5) Библиотека позволяет добавлять, получать и удалять любое устройство в доме.
// Получать список устройств в помещении.
fn test_5() {
    let mut house = House::new("My house");

    let room = Room::new("First room");
    let _ = house.add_room(room);

    let device = Device::new("First device", DeviceType::Thermometer, "Thermometer");
    let _ = house.room_mut("First room").unwrap().add_device(device);
    let device = Device::new("Second device", DeviceType::Socket, "Socket");
    let _ = house.room_mut("First room").unwrap().add_device(device);

    let room = Room::new("Second room");
    let _ = house.add_room(room);

    let device = Device::new("First device", DeviceType::Thermometer, "Thermometer");
    let _ = house.room_mut("Second room").unwrap().add_device(device);
    let device = Device::new("Second device", DeviceType::Socket, "Socket");
    let _ = house.room_mut("Second room").unwrap().add_device(device);

    let device = house.room("First room").unwrap().device("First device").unwrap();
    let device_name = device.name().clone();
    let _ = house.room_mut("First room").unwrap().remove_device(device_name.as_str());

    assert_eq!(house.room("First room").unwrap().devices().len(), 1);
    assert_eq!(house.room("Second room").unwrap().devices().len(), 2);
}
