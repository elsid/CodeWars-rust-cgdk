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
