use embassy_nrf::{peripherals::RNG, rng::Rng};

#[derive(Clone, Copy)]
pub enum DeviceAddressType {
    Public,
    Static,
    NonResolvablePrivate,
    ResolvablePrivate,
    ReservedPrivate,
}

/// Error returned by [`DeviceAddress::from_static`]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[derive(defmt::Format)]
pub enum CreateAddressError {
    InvalidStartBits,
    RandZeroOneMissing,
}

#[derive(Clone, Copy)]
pub struct DeviceAddress {
    address_type: DeviceAddressType,
    /// Stored in Little Endian format
    bytes: [u8; 6],
}

impl DeviceAddress {
    pub fn address_type(&self) -> DeviceAddressType {
        self.address_type
    }

    pub fn from_public(_bytes: [u8; 6]) -> Self {
        unimplemented!()
        // Self { bytes, address_type: DeviceAddressType::Public }
    }

    /// sets the device address to the static address provided 
    /// device address stored in Little Endian format
    /// checks if 2 most significant bits are correct (0b11)
    pub fn from_static(bytes: [u8; 6]) -> Result<Self, CreateAddressError> {
        if bytes[5] & 0b0000_0011 != 0b0000_0011 {
            return Err(CreateAddressError::InvalidStartBits)
        }

        if !contains_0_and_1(bytes, 0, 45) {
            return Err(CreateAddressError::RandZeroOneMissing)
        }

        Ok(Self { bytes, address_type: DeviceAddressType::Static })
    }

    /// generates a random static device address using the rng provided
    pub fn random_static(rng: &mut Rng<'_, RNG>) -> Self {
        let mut bytes = [0u8; 6];

        // ensures that the random part of the address contains at least one bit that is 0
        // and at least one bit that is one
        while !contains_0_and_1(bytes, 0, 45) {
            rng.blocking_fill_bytes(&mut bytes);
            bytes[5] |= 0b0000_0011;
        }

        Self { bytes, address_type: DeviceAddressType::Static }
    }
}

/// returns whether a byte array contains at least one 0 and at least one 1
/// in its bits within an inclusive range given
fn contains_0_and_1(bytes: [u8; 6], start: u8, end: u8) -> bool {
    let mut contains_0 = false;
    let mut contains_1 = false;

    let start_byte_index = start / 8;
    let end_byte_index = end / 8;

    let mut index = start_byte_index;

    if start_byte_index == end_byte_index {
        let byte = bytes[index as usize];
        let mask: u8 = (0b1111_1111 >> (start % 8)) & (0b1111_1111 << (end % 8));

        if byte & mask != 0b0000_0000 {
            contains_1 = true;
        }

        if byte | (mask ^ 0b1111_1111) != 0b1111_1111 {
            contains_0 = true;
        }
    }

    // check start byte
    {
        let byte = bytes[index as usize];
        let mask: u8 = 0b1111_1111 >> (start % 8);

        if byte & mask != 0b0000_0000 {
            contains_1 = true;
        }

        if byte | (mask ^ 0b1111_1111) != 0b1111_1111 {
            contains_0 = true;
        }
    }

    index += 1;

    while index < end_byte_index {
        let byte = bytes[index as usize];
        if byte != 0b0000_0000 {
            contains_1 = true;
        }

        if byte != 0b1111_1111 {
            contains_0 = true;
        }

        index += 1;
    }

    // check end byte
    {
        let byte = bytes[index as usize];
        let mask: u8 = 0b1111_1111 << (end % 8);

        if byte & mask != 0b0000_0000 {
            contains_1 = true;
        }

        if byte | (mask ^ 0b1111_1111) != 0b1111_1111 {
            contains_0 = true;
        }
    }

    contains_0 && contains_1
}
