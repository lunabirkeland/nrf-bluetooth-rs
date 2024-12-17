#![no_std]
#![no_main]

use core::{cell::RefCell, ptr::addr_of_mut};
use embassy_executor::Spawner;
use embassy_nrf::{
    bind_interrupts, gpio::{AnyPin, Input, Level, Output, OutputDrive, Pin, Pull}, peripherals::{self, RADIO, RNG}, radio::{self, ble::{self, Mode, Radio}, TxPower}, rng::{self, Rng}
};
use embassy_sync::blocking_mutex::CriticalSectionMutex;
use embassy_time::Timer;
use heapless::{arc_pool, pool::arc::ArcBlock};
use link_layer::{DeviceAddress, LinkLayer};
use {defmt_rtt as _, panic_probe as _};

mod link_layer;

// Declare async tasks
#[embassy_executor::task]
async fn blink(pin: AnyPin) {
    let mut led = Output::new(pin, Level::Low, OutputDrive::Standard);

    loop {
        // Timekeeping is globally available, no need to mess with hardware timers.
        led.set_high();
        Timer::after_millis(150).await;
        led.set_low();
        Timer::after_millis(150).await;
    }
}

#[embassy_executor::task]
async fn ble_radio(mut radio: Radio<'static, RADIO>, rng: CriticalSectionMutex<RefCell<Rng<'static, RNG>>>) {

    radio.set_tx_power(TxPower::_0D_BM);
    radio.set_mode(Mode::BLE_1MBIT);
    // ble crc polynomial x24 + x10 + x9 + x6 + x4 + x3 + x + 1
    radio.set_crc_poly(0b1000_0000_0000_0011_0010_1101);
    // radio.set_crc_init();
    // radio.set_frequency();
    // radio.set_whitening_init();
    // radio.set_access_address();
    // radio.set_header_expansion();
    
    let link_layer = rng.lock(|cell| {
        LinkLayer::new(link_layer::State::Advertising, DeviceAddress::random_static(&mut cell.borrow_mut()))
    });
    
    loop {
        // Timekeeping is globally available, no need to mess with hardware timers.
        Timer::after_millis(150).await;
        link_layer.poll(&mut radio);
    }
}

bind_interrupts!(struct Irqs {
    RADIO => radio::InterruptHandler<peripherals::RADIO>;
    RNG => rng::InterruptHandler<peripherals::RNG>;
});

// impl Debug in 

// Main is itself an async task as well.
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut config = embassy_nrf::config::Config::default();
    config.hfclk_source = embassy_nrf::config::HfclkSource::ExternalXtal;
    // Initialize the embassy-nrf HAL.
    let p = embassy_nrf::init(config);

    // create the radio driver
    let radio = Radio::new(p.RADIO, Irqs);

    // create the rng driver
    let rng = CriticalSectionMutex::new(RefCell::new(Rng::new(p.RNG, Irqs)));

    // Spawned tasks run in the background, concurrently.
    spawner.spawn(blink(p.P0_06.degrade())).unwrap();
    spawner.spawn(ble_radio(radio, rng)).unwrap();

    let mut button = Input::new(p.P1_06, Pull::Up);
    let mut red_led = Output::new(p.P0_08, Level::Low, OutputDrive::Standard);
    loop {
        // Asynchronously wait for GPIO events, allowing other tasks
        // to run, or the core to sleep.
        button.wait_for_low().await;
        red_led.set_high();
        button.wait_for_high().await;
        red_led.set_low();
    }
}
