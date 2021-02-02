#![feature(llvm_asm)]
#![feature(const_panic)]
#![feature(abi_avr_interrupt)]
#![no_std]
#![no_main]

// mod prerendered;
// use prerendered::Ws2812;
// use crate::ws2812::prerendered::Ws2812;

mod app;
mod clock;
mod color;
mod dac;
mod dac_byte;
mod led_controller;
mod millis;
mod scheduler;
mod sequence;
mod sequence_controller;
mod serial_wrapper;
mod trigger;
mod trigger_state;

use crate::app::{AppBuilder, AppBuilderTrait};
use crate::clock::{Clock, ClockFactory};
use crate::dac_byte::DacByte;
use crate::sequence::Sequence;
use crate::trigger::TriggerFactory;
use arduino_uno as arduino;
use arduino_uno::hal::port::mode::Output;
use arduino_uno::hal::port::portb::PB5;
use arduino_uno::hal::port::portc::PC4;
use embedded_hal::digital::v2::OutputPin;
use void::ResultVoidExt;
use ws2812_spi as ws2812;

const DELAY_TIME: u32 = 5;
const STEP_LED_COUNT: usize = 5;
const RGB_LED_COUNT: usize = 8;

const USE_INTERNAL_CLOCK: bool = true;

const SEQUENCES: [Sequence; 12] = [
    seq!(0, 3, 5, 8, 9, 10, 12, 15),
    seq!(15, 5, 5, 5, 0),
    seq!(0, 7, 15, 7, 0, 7, 15, 1,),    // 0b0½1½0½1½
    seq!(15, 15, 15, 15, 15, 15, 15,),  // 0b11111111
    seq!(0, 15, 0, 0, 15, 0, 0, 15,),   // 0b01001001
    seq!(0, 15, 0, 15, 0, 15, 0, 15,),  // 0b01010101
    seq!(0, 0, 0, 0, 15, 15, 15, 15,),  // 0b00001111
    seq!(15, 15, 15, 15, 0, 0, 0, 0,),  // 0b11110000
    seq!(15, 15, 0, 0, 15, 15, 0, 0,),  // 0b11001100
    seq!(15, 15, 15, 0, 0, 15, 0, 15,), // 0b11100101
    seq!(8, 8, 8, 12, 0, 8, 8, 12),
    seq!(0, 0, 0, 0, 0, 0, 0, 0),
];

#[arduino::entry]
fn main() -> ! {
    let clock_factory: ClockFactory<Clock> = ClockFactory::new();
    let trigger_factory = TriggerFactory::new();
    let mut app = AppBuilder::build(clock_factory, trigger_factory);
    app.run()
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let mut builtin_led: PB5<Output> = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
    builtin_led.set_high().void_unwrap();

    let mut a4: PC4<Output> = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
    a4.set_high().void_unwrap();

    let mut serial: arduino::Serial<arduino::hal::port::mode::Floating> =
        unsafe { core::mem::MaybeUninit::uninit().assume_init() };

    ufmt::uwriteln!(&mut serial, "Firmware panic!\r").void_unwrap();

    if let Some(loc) = info.location() {
        ufmt::uwriteln!(
            &mut serial,
            "  At {}:{}:{}\r",
            loc.file(),
            loc.line(),
            loc.column(),
        )
        .void_unwrap();
    }

    loop {
        builtin_led.set_high().void_unwrap();
        arduino::delay_ms(600);
        builtin_led.set_low().void_unwrap();
        arduino::delay_ms(150);
        builtin_led.set_high().void_unwrap();
        arduino::delay_ms(150);
        builtin_led.set_low().void_unwrap();
        arduino::delay_ms(150);
    }
}
