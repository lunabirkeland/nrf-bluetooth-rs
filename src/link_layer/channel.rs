use embassy_nrf::{peripherals::RNG, rng::Rng};

#[derive(Clone, Copy)]
pub enum Channel {
    /// General purpose channel
    _0,
    /// General purpose channel
    _1,
    /// General purpose channel
    _2,
    /// General purpose channel
    _3,
    /// General purpose channel
    _4,
    /// General purpose channel
    _5,
    /// General purpose channel
    _6,
    /// General purpose channel
    _7,
    /// General purpose channel
    _8,
    /// General purpose channel
    _9,
    /// General purpose channel
    _10,
    /// General purpose channel
    _11,
    /// General purpose channel
    _12,
    /// General purpose channel
    _13,
    /// General purpose channel
    _14,
    /// General purpose channel
    _15,
    /// General purpose channel
    _16,
    /// General purpose channel
    _17,
    /// General purpose channel
    _18,
    /// General purpose channel
    _19,
    /// General purpose channel
    _20,
    /// General purpose channel
    _21,
    /// General purpose channel
    _22,
    /// General purpose channel
    _23,
    /// General purpose channel
    _24,
    /// General purpose channel
    _25,
    /// General purpose channel
    _26,
    /// General purpose channel
    _27,
    /// General purpose channel
    _28,
    /// General purpose channel
    _29,
    /// General purpose channel
    _30,
    /// General purpose channel
    _31,
    /// General purpose channel
    _32,
    /// General purpose channel
    _33,
    /// General purpose channel
    _34,
    /// General purpose channel
    _35,
    /// General purpose channel
    _36,
    /// Primary advertising channel
    _37,
    /// Primary advertising channel
    _38,
    /// Primary advertising channel
    _39,
}

/// Error returned by [Channel::from_index`]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[derive(defmt::Format)]
pub enum ChannelFromIndexError {
    OutOfRangeError,
}

impl Channel {
    pub fn random_primary_advertising(rng: &mut Rng<'static, RNG>) -> Self {
        unimplemented!()
    }

    pub fn random_general_purpose(rng: &mut Rng<'static, RNG>) -> Self {
        unimplemented!()
    }

    pub fn from_index(index: u8) -> Result<Self, ChannelFromIndexError> {
        match index {
            0 => Ok(Self::_0),
            1 => Ok(Self::_1),
            2 => Ok(Self::_2),
            3 => Ok(Self::_3),
            4 => Ok(Self::_4),
            5 => Ok(Self::_5),
            6 => Ok(Self::_6),
            7 => Ok(Self::_7),
            8 => Ok(Self::_8),
            9 => Ok(Self::_9),
            10 => Ok(Self::_10),
            11 => Ok(Self::_11),
            12 => Ok(Self::_12),
            13 => Ok(Self::_13),
            14 => Ok(Self::_14),
            15 => Ok(Self::_15),
            16 => Ok(Self::_16),
            17 => Ok(Self::_17),
            18 => Ok(Self::_18),
            19 => Ok(Self::_19),
            20 => Ok(Self::_20),
            21 => Ok(Self::_21),
            22 => Ok(Self::_22),
            23 => Ok(Self::_23),
            24 => Ok(Self::_24),
            25 => Ok(Self::_25),
            26 => Ok(Self::_26),
            27 => Ok(Self::_27),
            28 => Ok(Self::_28),
            29 => Ok(Self::_29),
            30 => Ok(Self::_30),
            31 => Ok(Self::_31),
            32 => Ok(Self::_32),
            33 => Ok(Self::_33),
            34 => Ok(Self::_34),
            35 => Ok(Self::_35),
            36 => Ok(Self::_36),
            37 => Ok(Self::_37),
            38 => Ok(Self::_38),
            39 => Ok(Self::_39),
            _ => Err(ChannelFromIndexError::OutOfRangeError),
        }
    }

    pub fn freq(&self) -> u32 {
        match self {
            Channel::_37 => 2402,
            Channel::_0 => 2404,
            Channel::_1 => 2406,
            Channel::_2 => 2408,
            Channel::_3 => 2410,
            Channel::_4 => 2412,
            Channel::_5 => 2414,
            Channel::_6 => 2416,
            Channel::_7 => 2418,
            Channel::_8 => 2420,
            Channel::_9 => 2422,
            Channel::_10 => 2424,
            Channel::_38 => 2426,
            Channel::_11 => 2428,
            Channel::_12 => 2430,
            Channel::_13 => 2432,
            Channel::_14 => 2434,
            Channel::_15 => 2436,
            Channel::_16 => 2438,
            Channel::_17 => 2440,
            Channel::_18 => 2442,
            Channel::_19 => 2444,
            Channel::_20 => 2446,
            Channel::_21 => 2448,
            Channel::_22 => 2450,
            Channel::_23 => 2452,
            Channel::_24 => 2454,
            Channel::_25 => 2456,
            Channel::_26 => 2458,
            Channel::_27 => 2460,
            Channel::_28 => 2462,
            Channel::_29 => 2464,
            Channel::_30 => 2466,
            Channel::_31 => 2468,
            Channel::_32 => 2470,
            Channel::_33 => 2472,
            Channel::_34 => 2474,
            Channel::_35 => 2476,
            Channel::_36 => 2478,
            Channel::_39 => 2480,
        }
    }
}


