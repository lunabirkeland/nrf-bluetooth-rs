use bobbin_bits::U4;
use embassy_nrf::radio::ble;

use super::Address;

/// The mode of the advertisement.
/// 0b11 is reserved for future use.
pub enum AdvMode {
    NonConnectableNonScannable = 0b00,
    ConnectableNonScannable = 0b01,
    NonConnectableScannable = 0b10,
}

pub struct ExtendedHeader {
    /// AdvA
    /// The advertiser's device address.
    advertiser_address: Option<Address>,
    /// TargetA
    /// The advertiser's device address.
    target_address: Option<Address>,
    /// CTEInfo
    /// Constant Tone Extension
    /// TODO own data type
    cte_info: Option<u8>,
    /// AdvDataInfo (ADI)
    /// Advertising Data ID (first 12 bits) Advertising Set ID (remaining 4 bits).
    /// Contains the Advertising Data ID (DID) which is set by the advertiser
    /// to indicate to the scanner whether it can assume that the data contents
    /// in the AdvData are a duplicate to that sent in an earlier packet.
    /// Contains the Advertising Set ID (SID) which is set by the advertiser's
    /// host to identify an advertising set transmitted by this device.
    /// TODO own data type
    advertiser_data_info: Option<[u8; 2]>,
    /// AuxPtr
    /// Indicates that some or all of the advertisement data is in a subsequent
    /// auxiliary packet and describes this packet.
    /// TODO
    aux_ptr: Option<[u8; 3]>,
    /// SyncInfo
    /// Indicates the presence of, and describes, a periodic advertising train (using
    /// [AdvertisingPdu::AuxSyncInd] or [AdvertisingPdu::AuxSyncSubeventInd] PDU's).
    /// TODO
    sync_info: Option<[u8; 18]>,
    /// TxPower
    /// TODO
    tx_power: Option<u8>,
    /// Additional Controller Advertising Data
    /// TODO
    acad: Option<()>
}

impl ExtendedHeader {
    pub fn flag_bits(&self) -> u8 {
        let mut bits= 0u8;
        if self.advertiser_address.is_some() {
            bits += 0b0000_0001;
        }
        if self.target_address.is_some() {
            bits += 0b0000_0010;
        }
        if self.cte_info.is_some() {
            bits += 0b0000_0100;
        }
        if self.advertiser_data_info.is_some() {
            bits += 0b0000_1000;
        }
        if self.aux_ptr.is_some() {
            bits += 0b0001_0000;
        }
        if self.sync_info.is_some() {
            bits += 0b0010_0000;
        }
        if self.tx_power.is_some() {
            bits += 0b0100_0000;
        }

        bits
    }
    
}

/// Struct representing the Common Extended Advertising Payload Format.
pub struct CommonExtendedAdvertisingPayload {
    // extended_header_length: U6,
    advertiser_mode: AdvMode,
    extended_header: ExtendedHeader,
    advertiser_data: [u8; 254],
    advertiser_data_length: [u8; 254]
}

pub struct LLData {
    // TODO
}

/// Advertising Physical Channel Protocol Data Unit (PDU).
/// Despite the name it is also used on the periodic physical channel.
pub enum AdvertisingPdu {
    /// ADV_IND
    /// Used in connectable and scannable undirected advertising events.
    AdvInd {
        /// Indicates whether the advertiser’s address in the [AdvertisingPdu::AdvInd::advertiser_a]
        /// field is public (false) or random (true).
        tx_add: bool,
        /// Indicates if the advertiser supports the LE Channel Selection Algorithm #2 feature.
        ch_sel: bool,
        /// Contains the advertiser’s public or random device address as 
        /// indicated by [AdvertisingPdu::AdvInd::tx_add].
        advertiser_a: [u8; 6],
        /// Contains Advertising Data from the advertiser’s host, can be empty.
        /// Will be truncated to the length defined in [AdvertisingPdu::AdvInd::advertiser_data_length].
        advertiser_data: [u8; 31],
        /// Length of [AdvertisingPdu::AdvInd::advertiser_data] can be 0-31.
        /// Any number higher than 31 will be clamped.
        advertiser_data_length: u8,
    },
    /// ADV_DIRECT_IND
    /// Used in connectable directed advertising events.
    AdvDirectInd {
        /// Indicates whether the advertiser’s address in the [AdvertisingPdu::AdvDirectInd::advertiser_a]
        /// field is public (false) or random (true).
        tx_add: bool,
        /// Indicates whether the advertiser’s address in the [AdvertisingPdu::AdvDirectInd::target_a].
        /// field is public (false) or random (true).
        rx_add: bool,
        /// Indicates if the advertiser supports the LE Channel Selection Algorithm #2 feature.
        ch_sel: bool,
        /// Contains the advertiser’s public or random device address as
        /// indicated by [AdvertisingPdu::AdvDirectInd::tx_add].
        advertiser_a: [u8; 6],
        /// Contains the target’s public or random device address as
        /// indicated by [AdvertisingPdu::AdvDirectInd::rx_add].
        target_a: [u8; 6],
    },
    /// ADV_NONCONN_IND
    /// Used in non-connectable and non-scannable undirected advertising events.
    AdvNonconnInd {
        /// Contains the advertiser’s device address, tx_add in the header
        /// will be set to 0 if it is public, and 1 if it is random.
        advertiser_address: Address,
        /// Contains Advertising Data from the advertiser’s host, can be empty.
        /// Will be truncated to the length defined in [AdvertisingPdu::AdvNonconnInd::advertiser_data_length].
        advertiser_data: [u8; 31],
        /// Length of [AdvertisingPdu::AdvNonconnInd::advertiser_data] can be 0-31.
        /// Any number higher than 31 will be clamped.
        advertiser_data_length: u8,
    },
    /// ADV_SCAN_IND
    /// Used in scannable undirected advertising events.
    AdvScanInd {
        /// Indicates whether the advertiser’s address in the [AdvertisingPdu::AdvScanInd::advertiser_a]
        /// field is public (false) or random (true).
        tx_add: bool,
        /// Contains the advertiser’s public or random device address as 
        /// indicated by [AdvertisingPdu::AdvScanInd::tx_add].
        advertiser_a: [u8; 6],
        /// Contains Advertising Data from the advertiser’s host, can be empty.
        /// Will be truncated to the length defined in [AdvertisingPdu::AdvScanInd::advertiser_data_length].
        advertiser_data: [u8; 31],
        /// Length of [AdvertisingPdu::AdvScanInd::advertiser_data] can be 0-31.
        /// Any number higher than 31 will be clamped.
        advertiser_data_length: u8,
    },
    /// ADV_EXT_IND
    /// TODO
    AdvExtInd,
    /// AUX_ADV_IND
    /// TODO
    AuxAdvInd,
    /// AUX_SYNC_IND
    /// TODO
    AuxSyncInd,
    /// AUX_CHAIN_IND
    /// TODO
    AuxChainInd,
    /// AUX_SYNC_SUBEVENT_IND
    /// TODO
    AuxSyncSubeventInd,
    /// AUX_SYNC_SUBEVENT_RSP
    /// TODO
    AuxSyncSubeventRsp,
    /// ADV_DECISION_IND
    /// TODO
    AdvDecisionInd,
    /// SCAN_REQ
    /// Scan request PDU.
    ScanReq {
        /// Indicates whether the scanners’s address in the [AdvertisingPdu::ScanReq::scan_a]
        /// field is public (false) or random (true).
        tx_add: bool,
        /// Indicates whether the advertiser’s address in the [AdvertisingPdu::ScanReq::advertiser_a]
        /// field is public (false) or random (true).
        rx_add: bool,
        /// Contains the scanner’s public or random device address as 
        /// indicated by [AdvertisingPdu::ScanReq::tx_add].
        scan_a: [u8; 6],
        /// Contains the advertiser’s public or random device address as 
        /// indicated by [AdvertisingPdu::ScanReq::rx_add].
        advertiser_a: [u8; 6],
    },
    /// AUX_SCAN_REQ
    /// Scan request PDU.
    AuxScanReq {
        /// Indicates whether the scanners’s address in the [AdvertisingPdu::ScanReq::scan_a]
        /// field is public (false) or random (true).
        tx_add: bool,
        /// Indicates whether the advertiser’s address in the [AdvertisingPdu::ScanReq::advertiser_a]
        /// field is public (false) or random (true).
        rx_add: bool,
        /// Contains the scanner’s public or random device address as 
        /// indicated by [AdvertisingPdu::ScanReq::tx_add].
        scan_a: [u8; 6],
        /// Contains the advertiser’s public or random device address as 
        /// indicated by [AdvertisingPdu::ScanReq::rx_add].
        advertiser_a: [u8; 6],
    },
    /// SCAN_RSP
    /// Scan response PDU.
    ScanRsp {
        /// Indicates whether the advertiser’s address in the [AdvertisingPdu::ScanRsp::advertiser_a]
        /// field is public (false) or random (true).
        tx_add: bool,
        /// Contains the advertiser’s public or random device address as 
        /// indicated by [AdvertisingPdu::ScanRsp::tx_add].
        advertiser_a: [u8; 6],
        /// Contains Advertising Data from the advertiser’s host, can be empty.
        /// Will be truncated to the length defined in [AdvertisingPdu::ScanRsp::scan_rsp_data_length].
        scan_rsp_data: [u8; 31],
        /// Length of [AdvertisingPdu::ScanRsp::scan_rsp_data] can be 0-31.
        /// Any number higher than 31 will be clamped.
        scan_rsp_data_length: u8,
    },
    /// AUX_SCAN_RSP
    /// TODO
    AuxScanRsp,
    /// CONNECT_IND
    /// Sent by the Link Layer in the Initiating state and received by the
    /// Link Layer in the Advertising state.
    ConnectInd {
        /// InitA
        /// Contains the initiator’s device address, tx_add in the header
        /// will be set to 0 if it is public, and 1 if it is random.
        inititator_address: Address,
        /// AdvA
        /// Contains the advertiser’s device address, rx_add in the header
        /// will be set to 0 if it is public, and 1 if it is random.
        advertiser_address: Address,
        /// LLData
        ll_data: LLData,
    },
    /// AUX_CONNECT_REQ
    /// Sent by the Link Layer in the Initiating state and received by the
    /// Link Layer in the Advertising state.
    /// TODO
    AuxConnectReq,
    /// AUX_CONNECT_RSP
    /// Sent by the Link Layer in the Advertising state and received by the
    /// Link Layer in the Initiating state.
    /// TODO
    AuxConnectRsp,
}

impl AdvertisingPdu {
    /// Returns the PDU bytes as an array of length 258, and the length of the
    /// payload, which is the total length - 2.
    pub fn bytes(&self) -> ([u8; 258], u8) {
        match self {
            Self::AdvNonconnInd {
                advertiser_address,
                advertiser_data,
                advertiser_data_length
            } => {
                // 2 byte header, 6 byte address, 
                let payload_length = 6 + advertiser_data_length;
                let header = Self::header(self.pdu_type_bits(),
                    false, 
                    matches!(advertiser_address, Address::Random(_)),
                    false,
                    payload_length
                );

                let mut bytes = [0u8; 258];
                bytes[0..2].clone_from_slice(&header);
                bytes[2..8].clone_from_slice(advertiser_address.bytes());
                bytes[8..39].clone_from_slice(advertiser_data);

                (bytes, payload_length)
            }
            _ => unimplemented!()
        }
    }

    /// Returned in little endian format.
    fn header(pdu_type_bits: U4, ch_sel: bool, tx_add: bool, rx_add: bool, length: u8) -> [u8; 2] {
        [((pdu_type_bits as u8) << 4) + ((ch_sel as u8) << 2) + ((tx_add as u8) << 1) + (rx_add as u8), length]
    }

    fn pdu_type_bits(&self) -> U4 {
        match self {
            Self::AdvInd { .. } => U4::B0000,
            Self::AdvDirectInd { .. } => U4::B0001,
            Self::AdvNonconnInd { .. } => U4::B0010,
            Self::ScanReq { .. } => U4::B0011,
            Self::AuxScanReq { .. } => U4::B0011,
            Self::ScanRsp { .. } => U4::B0100,
            Self::ConnectInd { .. } => U4::B0101,
            Self::AuxConnectReq => U4::B0101,
            Self::AdvScanInd { .. } => U4::B0110,
            Self::AdvExtInd => U4::B0111,
            Self::AuxAdvInd => U4::B0111,
            Self::AuxScanRsp => U4::B0111,
            Self::AuxSyncInd => U4::B0111,
            Self::AuxChainInd => U4::B0111,
            Self::AuxSyncSubeventInd => U4::B0111,
            Self::AuxSyncSubeventRsp => U4::B0111,
            Self::AuxConnectRsp => U4::B1000,
            Self::AdvDecisionInd => U4::B1001,
        }
    }

    pub fn permitted(&self, mode: ble::Mode) -> bool {
        matches!((self, mode), 
            (Self::AdvInd { .. }, ble::Mode::BLE_1MBIT) | 
            (Self::AdvDirectInd { .. }, ble::Mode::BLE_1MBIT) |
            (Self::AdvNonconnInd { .. }, ble::Mode::BLE_1MBIT) |
            (Self::ScanReq { .. }, ble::Mode::BLE_1MBIT) |
            (Self::AuxScanReq { .. }, ble::Mode::BLE_1MBIT | ble::Mode::BLE_2MBIT | ble::Mode::BLE_LR125KBIT | ble::Mode::BLE_LR500KBIT) |
            (Self::ScanRsp { .. }, ble::Mode::BLE_1MBIT) |
            (Self::ConnectInd { .. }, ble::Mode::BLE_1MBIT) |
            (Self::AuxConnectReq, ble::Mode::BLE_1MBIT | ble::Mode::BLE_2MBIT | ble::Mode::BLE_LR125KBIT | ble::Mode::BLE_LR500KBIT) |
            (Self::AdvScanInd { .. }, ble::Mode::BLE_1MBIT) |
            (Self::AdvExtInd, ble::Mode::BLE_1MBIT | ble::Mode::BLE_LR125KBIT | ble::Mode::BLE_LR500KBIT) |
            (Self::AuxAdvInd, ble::Mode::BLE_1MBIT | ble::Mode::BLE_2MBIT | ble::Mode::BLE_LR125KBIT | ble::Mode::BLE_LR500KBIT) |
            (Self::AuxScanRsp, ble::Mode::BLE_1MBIT | ble::Mode::BLE_2MBIT | ble::Mode::BLE_LR125KBIT | ble::Mode::BLE_LR500KBIT) |
            (Self::AuxSyncInd, ble::Mode::BLE_1MBIT | ble::Mode::BLE_2MBIT | ble::Mode::BLE_LR125KBIT | ble::Mode::BLE_LR500KBIT) |
            (Self::AuxChainInd, ble::Mode::BLE_1MBIT | ble::Mode::BLE_2MBIT | ble::Mode::BLE_LR125KBIT | ble::Mode::BLE_LR500KBIT) |
            (Self::AuxSyncSubeventInd, ble::Mode::BLE_1MBIT | ble::Mode::BLE_2MBIT | ble::Mode::BLE_LR125KBIT | ble::Mode::BLE_LR500KBIT) |
            (Self::AuxSyncSubeventRsp, ble::Mode::BLE_1MBIT | ble::Mode::BLE_2MBIT | ble::Mode::BLE_LR125KBIT | ble::Mode::BLE_LR500KBIT) |
            (Self::AuxConnectRsp, ble::Mode::BLE_1MBIT | ble::Mode::BLE_2MBIT | ble::Mode::BLE_LR125KBIT | ble::Mode::BLE_LR500KBIT) |
            (Self::AdvDecisionInd, ble::Mode::BLE_1MBIT | ble::Mode::BLE_LR125KBIT | ble::Mode::BLE_LR500KBIT)
        )

    }
}
