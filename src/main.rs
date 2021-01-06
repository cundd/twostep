#![feature(llvm_asm)]
#![feature(const_panic)]
#![no_std]
#![no_main]
#![cfg_attr(not(feature = "std"), no_std)]

use arduino::prelude::*;
use arduino_uno as arduino;
use arduino_uno::adc;
use arduino_uno::adc::Adc;
use arduino_uno::hal::port::mode::{Analog, Floating, Input, InputMode, Output, PullUp};
use arduino_uno::hal::port::portb::{PB3, PB5};
use arduino_uno::hal::port::portc::{PC4, PC5};
use arduino_uno::hal::port::{mode, Pin};
use arduino_uno::Serial;
use ufmt::uWrite;

mod clock_in;
mod dac_byte;
mod trigger;
mod trigger_state;
use dac_byte::DacByte;
mod dac;
use dac::Dac;
mod sequence;
use crate::clock_in::{ClockIn, StepCounterType};
use crate::sequence::Sequence;
use crate::trigger::{Trigger, TriggerMode};
use crate::trigger_state::TriggerState;
use void::ResultVoidExt;

const DELAY_TIME: u16 = 5;

// const SEQUENCES: [u8; 7] = [
//     0b11111111, 0b01001001, 0b01010101, 0b00001111, 0b11110000, 0b11001100, 0b11100101,
// ];
type SequencesType = [Sequence; 11];
const SEQUENCES: SequencesType = [
    seq!(
        DacByte::max(),
        DacByte::new(5),
        DacByte::new(5),
        DacByte::new(5),
        DacByte::min(),
    ),
    seq!(
        // 0b0½1½0½1½
        DacByte::min(),
        DacByte::half(),
        DacByte::max(),
        DacByte::half(),
        DacByte::min(),
        DacByte::half(),
        DacByte::max(),
        DacByte::new(1),
    ),
    seq!(
        // 0b11111111
        DacByte::max(),
        DacByte::max(),
        DacByte::max(),
        DacByte::max(),
        DacByte::max(),
        DacByte::max(),
        DacByte::max(),
        // DacByte::max(),
    ),
    seq!(
        // 0b01001001
        DacByte::min(),
        DacByte::max(),
        DacByte::min(),
        DacByte::min(),
        DacByte::max(),
        DacByte::min(),
        DacByte::min(),
        DacByte::max(),
    ),
    seq!(
        // 0b01010101
        DacByte::min(),
        DacByte::max(),
        DacByte::min(),
        DacByte::max(),
        DacByte::min(),
        DacByte::max(),
        DacByte::min(),
        DacByte::max(),
    ),
    seq!(
        // 0b00001111
        DacByte::min(),
        DacByte::min(),
        DacByte::min(),
        DacByte::min(),
        DacByte::max(),
        DacByte::max(),
        DacByte::max(),
        DacByte::max(),
    ),
    seq!(
        // 0b11110000
        DacByte::max(),
        DacByte::max(),
        DacByte::max(),
        DacByte::max(),
        DacByte::min(),
        DacByte::min(),
        DacByte::min(),
        DacByte::min(),
    ),
    seq!(
        // 0b11001100
        DacByte::max(),
        DacByte::max(),
        DacByte::min(),
        DacByte::min(),
        DacByte::max(),
        DacByte::max(),
        DacByte::min(),
        DacByte::min(),
    ),
    seq!(
        // 0b11100101
        DacByte::max(),
        DacByte::max(),
        DacByte::max(),
        DacByte::min(),
        DacByte::min(),
        DacByte::max(),
        DacByte::min(),
        DacByte::max(),
    ),
    seq!(
        DacByte::new(8),
        DacByte::new(8),
        DacByte::new(8),
        DacByte::new(12),
        DacByte::min(),
        DacByte::new(8),
        DacByte::new(8),
        DacByte::new(12),
    ),
    seq!(
        DacByte::min(),
        DacByte::min(),
        DacByte::min(),
        DacByte::min(),
        DacByte::min(),
        DacByte::min(),
        DacByte::min(),
        DacByte::min(),
    ),
];

#[arduino::entry]
fn main() -> ! {
    let mut app = App::default();
    app.run()
}

#[derive(Clone)]
struct State {
    // #[deprecated]
    // step_counter: StepCounterType,
    sequence_pointer: usize,
    last_sequence_change_state: bool,
    // #[deprecated]
    // last_trigger_state: bool,
    trigger_interval: Option<u32>,
    auto_trigger_interval_countdown: u32,
    last_trigger_time: Option<u32>,
}

struct App {
    // trigger_input: PB4<Input<Floating>>,
    sequence_change_input: PC5<Input<PullUp>>,
    // builtin_led: PB5<Output>,
    step_output_pins: [Pin<Output>; 8],
    adc: Adc,
    a5: PC4<Analog>,
    dac: Dac,
    // trigger_out: PB2<Output>,
    sequence_change_output: PB3<Output>,
    serial: SerialWrapper<Floating>,
    sequences: &'static SequencesType,
    state: State,
    trigger: Trigger,
    clock_in: ClockIn,
}
impl Default for App {
    fn default() -> Self {
        let dp = arduino::Peripherals::take().unwrap();

        let mut pins = arduino::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
        let sequence_change_input = pins.a5.into_pull_up_input(&mut pins.ddr);
        let trigger_input = pins.d12.into_floating_input(&mut pins.ddr);
        // let adc6: u16 = nb::block!(adc.read(&mut adc::channel::ADC6)).void_unwrap();

        let builtin_led = pins.d13.into_output(&mut pins.ddr);
        let trigger_out = pins.d10.into_output(&mut pins.ddr);
        let sequence_change_output = pins.d11.into_output(&mut pins.ddr);
        let step_output_pins: [Pin<mode::Output>; 8] = [
            pins.d2.into_output(&mut pins.ddr).downgrade(),
            pins.d3.into_output(&mut pins.ddr).downgrade(),
            pins.d4.into_output(&mut pins.ddr).downgrade(),
            pins.d5.into_output(&mut pins.ddr).downgrade(),
            pins.d6.into_output(&mut pins.ddr).downgrade(),
            pins.d7.into_output(&mut pins.ddr).downgrade(),
            pins.d8.into_output(&mut pins.ddr).downgrade(),
            pins.d9.into_output(&mut pins.ddr).downgrade(),
        ];

        let serial = SerialWrapper {
            // debug: if cfg!(feature = "debug") { true } else { false },
            debug: true,
            serial: arduino::Serial::new(
                dp.USART0,
                pins.d0,
                pins.d1.into_output(&mut pins.ddr),
                57600.into_baudrate(),
            ),
        };

        let mut adc = adc::Adc::new(dp.ADC, Default::default());
        let a5 = pins.a4.into_analog_input(&mut adc);

        let a0 = pins.a0.into_output(&mut pins.ddr);
        let a1 = pins.a1.into_output(&mut pins.ddr);
        let a2 = pins.a2.into_output(&mut pins.ddr);
        let a3 = pins.a3.into_output(&mut pins.ddr);

        let dac = Dac::new(a0, a1, a2, a3);
        // let trigger = Trigger::new(trigger_input, trigger_out, TriggerMode::Hold);
        // let trigger = Trigger::new(trigger_input, trigger_out, TriggerMode::Pulse);
        let clock_in = ClockIn::new(trigger_input);
        let trigger = Trigger::new(trigger_out, builtin_led, TriggerMode::Follow);

        App {
            // builtin_led,
            step_output_pins,
            // trigger_input,
            sequence_change_input,
            sequence_change_output,
            adc,
            a5,
            serial,
            sequences: &SEQUENCES,
            // trigger_out,
            state: State {
                sequence_pointer: 0,
                last_sequence_change_state: false,
                trigger_interval: None,
                auto_trigger_interval_countdown: 0,
                last_trigger_time: None,
            },
            dac,
            trigger,
            clock_in,
        }
    }
}
impl App {
    fn run(&mut self) -> ! {
        ufmt::uwriteln!(&mut self.serial, "Hello from Arduino!\r").void_unwrap();
        let mut run_counter: u32 = 0;

        loop {
            run_counter += 1;
            self.run_loop(run_counter)
        }
    }

    fn run_loop(&mut self, run_counter: u32) {
        // let w = 0b00001111;
        // let w = 3; // 0.892V
        //            // let w = 7; // 2.06V
        //            // let w = 14; // = 4.12V
        //            // let w = 0b00001000;
        //            // let w = 0b00000010;
        //            // let w = 0b00010000;
        // self.dac.set(DacByte::new(w));
        // self.dac.set(DacByte::max());
        // self.print_bits(w);
        // ufmt::uwrite!(&mut self.serial, " ").void_unwrap();

        if cfg!(feature = "test_adc") {
            // let adc6: u16 = nb::block!(self.adc.read(&mut self.a5)).void_unwrap();
            let adc6: u16 = nb::block!(self.adc.read(&mut self.a5)).void_unwrap();
            self.print_bits(adc6 as u8);
            ufmt::uwrite!(&mut self.serial, " ").void_unwrap();
            ufmt::uwrite!(&mut self.serial, "adc6:{}", adc6).void_unwrap();
            ufmt::uwriteln!(&mut self.serial, " ").void_unwrap();
        }

        self.state = self.check_sequence_change();

        let sequence = self.get_sequence();
        let (trigger_state, step_counter) = self.clock_in.check(sequence);
        ufmt::uwriteln!(&mut self.serial, "ts:{}", trigger_state).void_unwrap();

        // If `auto_trigger` is enabled
        if cfg!(feature = "auto_trigger") {
            if let Some(trigger_interval) = self.state.trigger_interval {
                self.state.auto_trigger_interval_countdown -= 1;
                ufmt::uwriteln!(
                    &mut self.serial,
                    "tic:{} mod {} rc:{} ti:{}",
                    self.state.auto_trigger_interval_countdown,
                    run_counter % trigger_interval,
                    run_counter,
                    trigger_interval
                )
                .void_unwrap();
                if self.state.auto_trigger_interval_countdown <= 0 {
                    // Reset the countdown
                    self.state.auto_trigger_interval_countdown = trigger_interval;
                    ufmt::uwrite!(&mut self.serial, "at").void_unwrap();

                    self.trigger_step(step_counter, sequence);

                    // arduino::delay_ms(DELAY_TIME);
                    return;
                }
            }
        }

        self.trigger.check(trigger_state, step_counter, sequence);

        if trigger_state == TriggerState::Rise {
            ufmt::uwriteln!(&mut self.serial, "t ").void_unwrap();

            if cfg!(feature = "auto_trigger") {
                if let Some(last_trigger_time) = self.state.last_trigger_time {
                    let i = run_counter - last_trigger_time;
                    self.state.trigger_interval = Some(i);
                    // Reset the countdown
                    self.state.auto_trigger_interval_countdown = i;
                    ufmt::uwrite!(
                        &mut self.serial,
                        " uti: {:#?}",
                        run_counter - last_trigger_time
                    )
                    .void_unwrap();
                }
                self.state.last_trigger_time = Some(run_counter);
                ufmt::uwrite!(&mut self.serial, " ultt: {:#?}", run_counter).void_unwrap();
            }

            self.trigger_step(step_counter, sequence);
        }
        // arduino::delay_ms(DELAY_TIME);
    }

    fn get_sequence(&mut self) -> Sequence {
        self.sequences[self.state.sequence_pointer]
    }

    fn trigger_step(&mut self, step_counter: StepCounterType, sequence: Sequence) {
        let step_pointer: u8 = 0b00000001 << step_counter;
        ufmt::uwriteln!(&mut self.serial, "{:#?}\r", step_pointer).void_unwrap();
        ufmt::uwriteln!(
            &mut self.serial,
            "seq:{:?} sp:{:?} sc:{}\r",
            sequence,
            step_pointer,
            step_counter,
        )
        .void_unwrap();

        let step = sequence.get_step(step_pointer);
        match step {
            None => self.dac.set(DacByte::new(0)),
            Some(step) => self.dac.set(step),
        }

        self.set_all_step_pins_low();
        if sequence.matches(step_pointer) {
            // self.builtin_led.set_high().void_unwrap();
            self.step_output_pins[step_counter].set_high().void_unwrap();
        } else {
            // self.builtin_led.set_low().void_unwrap();
            self.step_output_pins[step_counter].set_low().void_unwrap();
            // self.step_output_pins[step_counter].set_high().void_unwrap();
        }
    }

    fn set_all_step_pins_low(&mut self) {
        App::set_pins_low(&mut self.step_output_pins)
    }

    fn set_pins_low(output_pins: &mut [Pin<Output>]) {
        for output_pin in output_pins {
            output_pin.set_low().void_unwrap();
        }
    }

    fn check_sequence_change(&mut self) -> State {
        let last_sequence_change_trigger_state = self.state.last_sequence_change_state;
        let sequences = self.sequences;
        let mut new_sequence_pointer = self.state.sequence_pointer;
        let sequence_change_input: bool = self.sequence_change_input.is_low().void_unwrap();
        if sequence_change_input && false == last_sequence_change_trigger_state {
            new_sequence_pointer += 1;
            if sequences.len() <= new_sequence_pointer {
                new_sequence_pointer = 0;
            }

            self.clock_in.reset();
            self.flash_sequence(sequences[new_sequence_pointer]);

            ufmt::uwriteln!(
                &mut self.serial,
                "change sequence {}\r",
                sequences[new_sequence_pointer]
            )
            .void_unwrap();
        }

        let mut new_state = self.state.clone();

        new_state.last_sequence_change_state = sequence_change_input;
        new_state.sequence_pointer = new_sequence_pointer;

        new_state
    }

    fn flash_sequence(&mut self, sequence: Sequence) {
        // self.builtin_led.set_high().void_unwrap();
        self.sequence_change_output.set_high().void_unwrap();
        for (i, step_output_pin) in self.step_output_pins.iter_mut().enumerate() {
            let current_bit = 0b00000001 << i;
            if sequence.matches(current_bit) {
                step_output_pin.set_high().void_unwrap();
            } else {
                step_output_pin.set_low().void_unwrap();
            }
        }
    }

    fn print_bits(&mut self, input: u8) {
        ufmt::uwrite!(&mut self.serial, "0b").void_unwrap();
        for position in (0..8).rev() {
            let bit = if input & (1 << position) == 0 {
                '0'
            } else {
                '1'
            };
            ufmt::uwrite!(&mut self.serial, "{}", bit).void_unwrap();
        }
    }
}

// fn fmt_bits(b: u8) {
//     let mut buf: &str = "0b0000000";
//     for position in 7..0 {
//         b & (1 << position);
//         buf.bytes().nth()[1] = 'c';
//     }
// }

struct SerialWrapper<IMODE: InputMode> {
    debug: bool,
    serial: Serial<IMODE>,
}
impl<IMODE: InputMode> uWrite for SerialWrapper<IMODE> {
    type Error = void::Void;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        if self.debug {
            self.serial.write_str(s)
        } else {
            Ok(())
        }

        // if cfg!(feature = "debug") {
        //     if self.debug {
        //         self.serial.write_str(s)
        //     } else {
        //         Ok(())
        //     }
        // } else {
        //     Ok(())
        // }
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let mut builtin_led: PB5<Output> = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
    builtin_led.set_high().void_unwrap();

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
