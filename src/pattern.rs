use super::utils::get_byte_count_from_int;

/// Pattern
/// This is a structure to hold each entry in the data table at
/// the end of the file.
pub struct Pattern {
    pub bytes_to_desc_addr: u8,
    pub addr: u32,
    pub bytes_to_desc_count: u8,
    pub count: u32,
    pub character: u8,
}

impl Pattern {
    /// new_serialize
    /// this takes an address, number of times it occurs, and the target byte
    /// and serializes it into a structure we can use to write the table
    pub fn new_serialize(addr: u32, count: u32, character: u8) -> Pattern {
        Pattern {
            bytes_to_desc_addr: get_byte_count_from_int(addr),
            addr,
            bytes_to_desc_count: get_byte_count_from_int(count),
            count,
            character,
        }
    }

    /// new_deserialize
    /// this takes an address, number of times it occurs, and the target byte
    /// as well as the number to desc the address and count. since we already have these in memory,
    /// it is simpler and more efficent to store these here than to do the calculations again.
    pub fn new_deserialize(
        bytes_to_desc_addr: u8,
        addr: u32,
        bytes_to_desc_count: u8,
        count: u32,
        character: u8,
    ) -> Pattern {
        Pattern {
            bytes_to_desc_addr,
            addr,
            bytes_to_desc_count,
            count,
            character,
        }
    }
}
