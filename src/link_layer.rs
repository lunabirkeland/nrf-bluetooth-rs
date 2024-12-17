use channel::Channel;
pub use device_address::DeviceAddress;
use embassy_nrf::{peripherals::RADIO, radio::ble};

mod device_address;
mod channel;
mod packet;

#[derive(Default, Clone, Copy)]
pub enum State {
    #[default]
    Standby,
    Advertising,
    Scanning,
    Initiating,
    Connection(Role),
    Synchronization,
    IsochronousBroadcasting
}

#[derive(Clone, Copy)]
pub enum Role {
    Central,
    Peripheral
}

/// Error returned by [`LinkLayer::change_state`]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[derive(defmt::Format)]
#[non_exhaustive]
pub enum ChangeStateError {
    InvalidChange,
}

#[derive(Clone, Copy)]
#[non_exhaustive]
pub struct LinkLayer {
    state: State,
    device_address: DeviceAddress,
    channel: Option<Channel>,
}

impl LinkLayer {
    pub fn new(state: State, device_address: DeviceAddress) -> Self {
        Self { state, device_address, channel: None }
    }

    pub fn change_state(&mut self, state: State) -> Result<(), ChangeStateError> {
        match (self.state, state) {
            (State::Standby, State::Connection(_)) => Err(ChangeStateError::InvalidChange),
            (State::Standby, _) | (_, State::Standby) |
            (State::Advertising, State::Connection(Role::Peripheral)) |
            (State::Initiating, State::Connection(Role::Central)) => {
                self.state = state;

                Ok(())
            },
            _ => Err(ChangeStateError::InvalidChange)
        }

    }

    pub fn poll(&self, radio: &mut ble::Radio<'static, RADIO>) {
        if let Some(channel) = self.channel {
            radio.set_frequency(channel.freq());
        }

        match self.state {
            State::Standby => {},
            State::Advertising => {

            }
            _ => unimplemented!()
        }
    }
}
