#![feature(type_alias_impl_trait)]
#![no_std]
#![no_main]

extern crate alloc;
mod encoder;

use core::sync::atomic::AtomicU32;

// use cortex_m::interrupt::Mutex;
use defmt::info;
use embassy_executor::Spawner;

use embassy_rp::peripherals::{PIO0, PIO1};
use embassy_rp::{bind_interrupts, pio};
use pio::{InterruptHandler, Pio};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    PIO1_IRQ_0 => InterruptHandler<PIO1>;
});

static ENC_LEFT: AtomicU32 = AtomicU32::new(0);
static ENC_RIGHT: AtomicU32 = AtomicU32::new(0);

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let Pio {
        mut common, sm0, ..
    } = Pio::new(p.PIO0, Irqs);

    let mut encoder = encoder::PioEncoder::new(&mut common, sm0, p.PIN_4, p.PIN_5);

    let mut count = 0;
    loop {
        info!("Count: {}", count);
        count += match encoder.read().await {
            encoder::Direction::Clockwise => 1,
            encoder::Direction::CounterClockwise => -1,
        };
    }
}
