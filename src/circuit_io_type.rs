use anyhow::Result;
use ark_r1cs_std::R1CSVar;
use simpleworks::gadgets::Record;

pub use CircuitIOType::{
    SimpleRecord, SimpleUInt128, SimpleUInt16, SimpleUInt32, SimpleUInt64, SimpleUInt8,
};

#[derive(Clone, Debug)]
pub enum CircuitIOType {
    SimpleUInt8(crate::UInt8Gadget),
    SimpleUInt16(crate::UInt16Gadget),
    SimpleUInt32(crate::UInt32Gadget),
    SimpleUInt64(crate::UInt64Gadget),
    SimpleUInt128(crate::UInt128Gadget),
    SimpleRecord(Record),
}

impl CircuitIOType {
    // TODO: This is temporary, we need to find a better way to handle this method.
    pub fn value(&self) -> Result<String> {
        match self {
            SimpleUInt8(value) => Ok(value.value()?.to_string()),
            SimpleUInt16(value) => Ok(value.value()?.to_string()),
            SimpleUInt32(value) => Ok(value.value()?.to_string()),
            SimpleUInt64(value) => Ok(value.value()?.to_string()),
            SimpleUInt128(value) => Ok(value.value()?.to_string()),
            _ => todo!(),
        }
    }
}
