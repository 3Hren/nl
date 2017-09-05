extern crate byteorder;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod serialize;

pub use self::serialize::NetlinkSerializer;

#[derive(Default, Serialize)]
pub struct ControlAttributes<'a> {
    family_id: Option<u16>,
    family_name: Option<&'a str>,
}

#[derive(Serialize)]
pub enum ControlMessage<'a> {
    /// Returned in response to a `GetFamily` request.
    NewFamily(ControlAttributes<'a>),
    DelFamily,
    GetFamily(ControlAttributes<'a>),
}

#[cfg(test)]
mod test {
    use serde::Serialize;
    use super::*;

    #[test]
    fn get_family_serialize() {
        let message = ControlMessage::GetFamily(ControlAttributes {
            family_name: Some("IPVS"),
            ..Default::default()
        });

        let mut se = NetlinkSerializer::new(vec![]);
        message.serialize(&mut se).unwrap();

        assert_eq!(vec![3, 1, 0, 0, 9, 0, 2, 0, 73, 80, 86, 83, 0, 0, 0, 0], se.into_inner());
    }
}
