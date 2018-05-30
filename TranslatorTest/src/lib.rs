#[macro_use]
extern crate translator;

use std::os::raw::c_char;
use std::ptr;

pub trait Encodable {
    ///Encode takes in the message, and generates the ring binary
    fn encode(&self) -> [u16;5];
    fn encode_with_checksum(&self) -> [u8;6];
}
fn build_b0(message_type: i16, origin_addr: i16) -> u16 {
    let mut b0: i16 = 0;
    b0 = b0 | ((message_type & 0x07) << 4);
    b0 = b0 | (origin_addr & 0x0F);
    b0 = b0 | 0x80;
    return b0 as u16;
}
fn build_mes_with_checksum(message: [u16;5]) -> [u8;6] {
    let mut final_message: [u8;6] = [0;6];
    let mut chksm: u8 = 0xff;
    for index in 0..5 {
        let u8val = message[index] as u8;
        chksm = chksm ^ u8val;
        final_message[index] = u8val;
    }
    final_message[5] = chksm;
    return final_message;
}

#[repr(C)]
#[derive(Clone, Copy, Translate)]
pub struct SomeStruct {
    //pub raw_message: [i16;5],
    pub foo: i32,
    pub bar: Baz,
    pub foobar: [u8;5]
}

#[repr(C)]
#[derive(Clone, Copy, Translate)]
pub struct Baz {
    pub bob: f32
}

#[repr(C)]
#[derive(Clone, Copy, Translate)]
pub struct CommonModel {
    //pub raw_message: [i16;5],
    pub message_type: i16,
    pub sub_type: i16,
    pub sub_name: *mut c_char,
    pub origin_addr: i16,
    pub raw_message: [u8;5],
    pub foobar: *mut i16,
    pub baz: *const Type2_Common,
    pub bob: i16,
}

#[repr(C)]
#[derive(Translate)]
pub struct Type2_Common {
    pub is_extended: bool,
    pub dest_addr: i16,
    pub call_pos: i16,
    pub is_rcc: bool,
    pub is_ruhc: bool,
    pub is_rdhc: bool,
    pub is_rec: bool,
    pub destination_group_val: i16,
    pub destination_group: *mut c_char,
    pub is_fec: bool,
    pub is_fdhc: bool,
    pub is_fuhc: bool,
    pub is_fcc: bool,
    //pub call_type: *mut c_char,
}
impl Type2_Common {
    pub const DEST_GRP_NOT_DUPK: i16 = 0;
    pub const DEST_GRP_LOW_ZONE: i16 = 1;
    pub const DEST_GRP_HIGH_ZONE: i16 = 2;
}

#[repr(C)]
#[derive(Translate)]
pub struct Type2_00 {
    pub common_model: CommonModel,
    pub type2_common: Type2_Common,
    pub cancellation_state_val: i16,
    pub cancellation_state: *mut c_char,
}
impl Encodable for Type2_00 {
    fn encode(&self) -> [u16;5] {
        let mut message: [u16;5] = [0,0,0,0,0];
        message[0] = build_b0(self.common_model.message_type, self.common_model.origin_addr);
        //byte 1 is subtype (which is 0), callset and dest addr
        //subtype
        message[1] |= (0x80 | ((self.type2_common.is_extended as i16) << 4)) as u16;
        //dest addr
        message[1] |=( 0x80 | self.type2_common.dest_addr) as u16;
        //byte 2 is front ehs, fdhc, fuhc, fcc
        message[2] = (0x80 | ((self.type2_common.is_fec as i16) << 3) | ((self.type2_common.is_fdhc as i16) << 2)
            | ((self.type2_common.is_fuhc as i16) << 1) | (self.type2_common.is_fcc as i16)) as u16;
        //byte 3 is cancellation state, dest group, rear ehs, rdhc, ruhc, rcc
        message[3] = (0x80 | ((self.cancellation_state_val & 0x01) << 6) 
            | ((self.type2_common.destination_group_val & 0x03) << 4)
            | ((self.type2_common.is_rec as i16) << 3)
            | ((self.type2_common.is_rdhc as i16) << 2)
            | ((self.type2_common.is_ruhc as i16) << 1)
            | (self.type2_common.is_rcc as i16)) as u16;
        //byte 4 is call p[osition
        message[4] = (0x80 | (self.type2_common.call_pos & 0x7F)) as u16;
        return message;
    }
    fn encode_with_checksum(&self) -> [u8;6] {
        build_mes_with_checksum(self.encode())
    }
}
impl Type2_00 {
    pub const CALL_CANCELED_BEFORE_SERVED: i16 = 0;
    pub const CALL_CANCELED_AND_SERVED: i16 = 1;
}

#[derive(Translate)]
struct __FinalizeTranslatorStruct__{}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
