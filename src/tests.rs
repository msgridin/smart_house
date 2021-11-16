use super::*;
use house::House;
use room::Room;
use device::{Thermometer, Socket};

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
    assert_eq!(house.room("First room").unwrap().name(), Room::new("First room").name());
    assert_eq!(house.room("Third room").unwrap().name(), Room::new("Third room").name());
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

    let device = Thermometer::new("First device", "Thermometer");
    let _ = house.room_mut("First room").unwrap().add_device(Box::new(device));
    let device = Socket::new("Second device", "Socket");
    let _ = house.room_mut("First room").unwrap().add_device(Box::new(device));

    assert_eq!(house.room("First room").unwrap().devices().len(), 2);
}

#[test]
// 4) Устройство имеет уникальное в рамках помещения название.
fn test_4() {
    let mut house = House::new("My house");

    let room = Room::new("First room");
    let _ = house.add_room(room);

    let device = Thermometer::new("First device", "Thermometer");
    let _ = house.room_mut("First room").unwrap().add_device(Box::new(device));
    let device = Socket::new("First device", "Socket");
    if let Ok(_) = house.room_mut("First room").unwrap().add_device(Box::new(device)) {
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

    let device = Thermometer::new("First device", "Thermometer");
    let _ = house.room_mut("First room").unwrap().add_device(Box::new(device));
    let device = Socket::new("Second device", "Socket");
    let _ = house.room_mut("First room").unwrap().add_device(Box::new(device));

    let room = Room::new("Second room");
    let _ = house.add_room(room);

    let device = Thermometer::new("First device", "Thermometer");
    let _ = house.room_mut("Second room").unwrap().add_device(Box::new(device));
    let device = Socket::new("Second device", "Socket");
    let _ = house.room_mut("Second room").unwrap().add_device(Box::new(device));

    let device = house.room("First room").unwrap().device("First device").unwrap();
    let device_name = device.name().clone();
    let _ = house.room_mut("First room").unwrap().remove_device(device_name.as_str());

    assert_eq!(house.room("First room").unwrap().devices().len(), 1);
    assert_eq!(house.room("Second room").unwrap().devices().len(), 2);
}

#[test]
// 6) Термометр позволяет узнать температуру.
fn test_6() {
    let mut house = House::new("My house");

    let room = Room::new("First room");
    let _ = house.add_room(room);

    let device = Thermometer::new("Thermometer", "");
    let _ = house.room_mut("First room").unwrap().add_device(Box::new(device));

    if let Ok(device) = house.room("First room").unwrap().device("Thermometer") {
        let thermometer = device.as_any().downcast_ref::<Thermometer>().unwrap();
        assert_eq!(thermometer.get_temperature(), 24.);
    }
}

#[test]
// 7) Умная розетка позволяет включать и выключать себя.
// Предоставляет информацию о текущей потребляемой мощности.
fn test_7() {
    let mut house = House::new("My house");

    let room = Room::new("First room");
    let _ = house.add_room(room);

    let device = Socket::new("Socket", "");
    let _ = house.room_mut("First room").unwrap().add_device(Box::new(device));

    if let Ok(device) = house.room_mut("First room").unwrap().device_mut("Socket") {
        let socket = device.as_any_mut().downcast_mut::<Socket>().unwrap();
        socket.on();
        socket.off();
        socket.on();
        let device_ref = house.room("First room").unwrap().device("Socket").unwrap().as_any().downcast_ref::<Socket>().unwrap();
        assert_eq!(device_ref.switcher(), true);
        assert_eq!(device_ref.get_capacity(), 500);
    }
}
