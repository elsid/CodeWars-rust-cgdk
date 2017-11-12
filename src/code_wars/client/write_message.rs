use std::io;
use byteorder::{ByteOrder, WriteBytesExt};
use code_wars::model::{ActionType, Move, VehicleType};
use super::message::Message;

pub trait WriteMessage: WriteBytesExt {
    fn write_message<B: ByteOrder>(&mut self, value: &Message) -> io::Result<()> {
        self.write_message_id(value.get_id())?;
        self.write_message_content::<B>(value)
    }

    fn write_message_id(&mut self, value: i8) -> io::Result<()> {
        self.write_i8(value)
    }

    fn write_message_content<B: ByteOrder>(&mut self, value: &Message) -> io::Result<()> {
        match value {
            &Message::UnknownMessage => unimplemented!(),
            &Message::GameOver => unimplemented!(),
            &Message::AuthenticationToken(ref v) => self.write_authentication_token::<B>(v),
            &Message::TeamSize(v) => self.write_i32::<B>(v),
            &Message::ProtocolVersion(v) => self.write_i32::<B>(v),
            &Message::GameContext(ref _v) => unimplemented!(),
            &Message::PlayerContext(ref _v) => unimplemented!(),
            &Message::MoveMessage(ref v) => self.write_move::<B>(v),
        }
    }

    fn write_authentication_token<B: ByteOrder>(&mut self, value: &String) -> io::Result<()> {
        self.write_i32::<B>(value.len() as i32)?;

        for b in value.bytes() {
            self.write_u8(b)?;
        }

        Ok(())
    }

    fn write_move<B: ByteOrder>(&mut self, value: &Move) -> io::Result<()> {
        self.write_bool(true)?;
        self.write_action_type(value.action())?;
        self.write_i32::<B>(value.group())?;
        self.write_f64::<B>(value.left())?;
        self.write_f64::<B>(value.top())?;
        self.write_f64::<B>(value.right())?;
        self.write_f64::<B>(value.bottom())?;
        self.write_f64::<B>(value.x())?;
        self.write_f64::<B>(value.y())?;
        self.write_f64::<B>(value.angle())?;
        self.write_f64::<B>(value.factor())?;
        self.write_f64::<B>(value.max_speed())?;
        self.write_f64::<B>(value.max_angular_speed())?;
        self.write_vehicle_type(value.vehicle_type())?;
        self.write_i64::<B>(value.facility_id())?;
        self.write_i64::<B>(value.vehicle_id())?;
        Ok(())
    }

    fn write_action_type(&mut self, value: ActionType) -> io::Result<()> {
        self.write_i8(value as i8)
    }

    fn write_vehicle_type(&mut self, value: VehicleType) -> io::Result<()> {
        self.write_i8(value as i8)
    }

    fn write_bool(&mut self, value: bool) -> io::Result<()> {
        self.write_u8(if value { 1 } else { 0 })
    }
}

impl<W: WriteBytesExt> WriteMessage for W {}

#[test]
fn test_write_bool_false() {
    let mut buffer = vec![];
    buffer.write_bool(false).unwrap();
    assert_eq!(buffer, vec![0u8]);
}

#[test]
fn test_write_bool_true() {
    let mut buffer = vec![];
    buffer.write_bool(true).unwrap();
    assert_eq!(buffer, vec![1u8]);
}

#[test]
fn test_write_message_authentication_token() {
    use byteorder::LittleEndian;
    let message = Message::AuthenticationToken("foo".to_string());
    let mut buffer = vec![];
    buffer.write_message::<LittleEndian>(&message).unwrap();
    assert_eq!(buffer, vec![
        2u8,
        3u8, 0u8, 0u8, 0u8,
        102u8, 111u8, 111u8,
    ]);
}

#[test]
fn test_write_message_protocol_version() {
    use byteorder::LittleEndian;
    let message = Message::ProtocolVersion(42);
    let mut buffer = vec![];
    buffer.write_message::<LittleEndian>(&message).unwrap();
    assert_eq!(buffer, vec![
        4u8,
        42u8, 0u8, 0u8, 0u8,
    ]);
}

#[test]
fn test_write_message_move() {
    use byteorder::LittleEndian;
    let mut move_ = Move::new();
    move_
        .set_action(ActionType::ClearAndSelect)
        .set_group(1)
        .set_left(2.0)
        .set_top(3.0)
        .set_right(4.0)
        .set_bottom(5.0)
        .set_x(6.0)
        .set_y(7.0)
        .set_angle(8.0)
        .set_factor(9.0)
        .set_max_speed(10.0)
        .set_max_angular_speed(11.0)
        .set_vehicle_type(VehicleType::Tank)
        .set_facility_id(12)
        .set_vehicle_id(13);
    let message = Message::MoveMessage(move_);
    let mut buffer = vec![];
    buffer.write_message::<LittleEndian>(&message).unwrap();
    assert_eq!(buffer, vec![
        7u8,
        1u8,
        1u8,
        1u8, 0u8, 0u8, 0u8,
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 64u8,
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 8u8, 64u8,
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 16u8, 64u8,
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 20u8, 64u8,
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 24u8, 64u8,
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 28u8, 64u8,
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 32u8, 64u8,
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 34u8, 64u8,
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 36u8, 64u8,
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 38u8, 64u8,
        5u8,
        12u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        13u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    ]);
}
