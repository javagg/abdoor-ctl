#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_time::Timer;
use embassy_stm32::{bind_interrupts, peripherals};

use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
async fn blinky(led: peripherals::PC13) {
    let mut led = Output::new(led, Level::High, Speed::Low);
    loop {
        // info!("high");
        led.set_high();
        Timer::after_millis(300).await;
        // info!("low");
        led.set_low();
        Timer::after_millis(300).await;
    }
}

// bind_interrupts!(struct Irqs {
//     RNG_LPUART1 => rng::InterruptHandler<peripherals::RNG>;

//     // TIM3 => embassy_stm32::usb::InterruptHandler<peripherals::TIM3>;
// });

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello DM542 and 57 Stepper");

    unwrap!(spawner.spawn(blinky(p.PC13)));
 
    let a = Input::new(p.PA1, Pull::Down);
    let b = Input::new(p.PA2, Pull::Down);
    loop {
        info!("a: {}, b: {}", a.get_level(), b.get_level());
        Timer::after_millis(500).await;
    }
}