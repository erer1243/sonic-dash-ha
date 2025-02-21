use std::error::Error;
use swss_common::KeyOpFieldValues;

pub fn encode(kfvs: &KeyOpFieldValues) -> Vec<u8> {
    bincode::serialize(kfvs).expect("Error serializing KeyOpFieldValues")
}

pub fn decode(data: &[u8]) -> Result<KeyOpFieldValues, Box<dyn Error>> {
    bincode::deserialize(data).map_err(|e| e.into())
}
