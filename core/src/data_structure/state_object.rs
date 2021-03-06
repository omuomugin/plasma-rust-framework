extern crate ethereum_types;
extern crate rlp;

use bytes::Bytes;
use ethereum_types::Address;
use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};

#[derive(Clone, Debug, PartialEq)]
pub struct StateObject {
    predicate: Address,
    parameters: Vec<u8>,
}

impl StateObject {
    pub fn new(predicate: Address, parameters: &Bytes) -> StateObject {
        StateObject {
            predicate,
            parameters: parameters.to_vec(),
        }
    }
}

impl Encodable for StateObject {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(2);
        s.append(&self.predicate);
        s.append(&self.parameters);
    }
}

impl Decodable for StateObject {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        let predicate: Address = rlp.val_at(0)?;
        let parameters: Vec<u8> = rlp.val_at(1)?;
        Ok(StateObject {
            predicate,
            parameters,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Bytes;
    use super::DecoderError;
    use super::StateObject;
    use ethereum_types::Address;

    #[test]
    fn test_rlp_encode() {
        let parameters_bytes = Bytes::from(&b"parameters"[..]);
        let _state_object = StateObject::new(Address::zero(), &parameters_bytes);
        let encoded = rlp::encode(&_state_object);
        let _decoded: StateObject = rlp::decode(&encoded).unwrap();
        assert_eq!(_decoded.predicate, _state_object.predicate);
    }

    #[test]
    fn fail_to_decode() {
        let animal = "cat";
        let encoded = rlp::encode(&animal);
        let decoded: Result<StateObject, DecoderError> = rlp::decode(&encoded);
        assert_eq!(decoded.is_err(), true);
    }

}
