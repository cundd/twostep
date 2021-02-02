mod app_builder;

use crate::clock::{Clock, ClockResult, ClockTrait, StepCounterType};
use crate::color::color_from_serial;
use crate::dac::Dac;
use crate::dac_byte::DacByte;
use crate::led_controller::LedController;
use crate::sequence::Sequence;
use crate::sequence_controller::{SequenceController, SequenceState};
use crate::serial_wrapper::SerialWrapper;
use crate::trigger::Trigger;
use crate::trigger_state::TriggerState;
use crate::{color, STEP_LED_COUNT};
pub use app_builder::AppBuilder;
pub use app_builder::AppBuilderTrait;
use arduino::prelude::*;
use arduino_uno as arduino;
use arduino_uno::adc::Adc;
use arduino_uno::hal::port::mode::{Analog, Floating, Output};
use arduino_uno::hal::port::portc::PC4;
use arduino_uno::hal::port::portd::PD4;
use arduino_uno::hal::port::Pin;
use embedded_hal::digital::v2::OutputPin;
use void::ResultVoidExt;

#[derive(Clone)]
struct State {
    trigger_interval: Option<u32>,
    auto_trigger_interval_countdown: u32,
    last_trigger_time: Option<u32>,
}

#[allow(unused)]
pub struct App<CLOCK: ClockTrait> {
    step_output_pins: [Pin<Output>; STEP_LED_COUNT],
    adc: Adc,
    dac: Dac,
    sequence_change_output: PD4<Output>,
    serial: SerialWrapper<Floating>,
    state: State,
    trigger: Trigger,
    clock_in: CLOCK,
    sequence_controller: SequenceController,
    led_controller: LedController<'static>,
    analog_input: Option<PC4<Analog>>,
}

impl App<Clock> {
    pub fn run(&mut self) -> ! {
        if cfg!(feature = "debug") {
            ufmt::uwriteln!(
                &mut self.serial.get_serial(),
                "Hello from Arduino with debug!\r"
            )
            .void_unwrap();
        } else {
            ufmt::uwriteln!(
                &mut self.serial.get_serial(),
                "Hello from Arduino without debug!\r"
            )
            .void_unwrap();
        }
        let mut run_counter: u32 = 0;

        self.initialize_leds();
        // self.test_colors();

        loop {
            run_counter += 1;
            self.run_loop(run_counter);
        }
    }

    fn run_loop(&mut self, _run_counter: u32) {
        if cfg!(feature = "test_adc") {
            if let Some(a) = self.analog_input.as_mut() {
                let adc_value: u16 = nb::block!(self.adc.read(&mut *a)).void_unwrap();
                self.print_bits(adc_value as u8);
                ufmt::uwriteln!(&mut self.serial, " adc_value:{}", adc_value).void_unwrap();
            }
        }

        let sequence_state = self.check_sequence_change();

        let sequence = sequence_state.sequence;
        let ClockResult {
            trigger_state,
            step_counter,
        } = self.clock_in.check(&mut self.serial, sequence);

        // If `auto_trigger` is enabled
        // if cfg!(feature = "auto_trigger") {
        //     if let Some(trigger_interval) = self.state.trigger_interval {
        //         self.state.auto_trigger_interval_countdown -= 1;
        //         ufmt::uwriteln!(
        //             &mut self.serial,
        //             "tic:{} mod {} rc:{} ti:{}",
        //             self.state.auto_trigger_interval_countdown,
        //             run_counter % trigger_interval,
        //             run_counter,
        //             trigger_interval
        //         )
        //         .void_unwrap();
        //         if self.state.auto_trigger_interval_countdown <= 0 {
        //             // Reset the countdown
        //             self.state.auto_trigger_interval_countdown = trigger_interval;
        //             ufmt::uwrite!(&mut self.serial, "at").void_unwrap();
        //
        //             self.trigger_step(step_counter, sequence, led_controller);
        //
        //             // arduino::delay_ms(DELAY_TIME);
        //             return;
        //         }
        //     }
        // }

        self.trigger.check_scheduled(
            crate::millis::millis(),
            trigger_state,
            step_counter,
            sequence,
        );
        self.trigger.check(trigger_state, step_counter, sequence);
        if trigger_state == TriggerState::Rise {
            ufmt::uwriteln!(&mut self.serial, "t ").void_unwrap();

            // if cfg!(feature = "auto_trigger") {
            //     if let Some(last_trigger_time) = self.state.last_trigger_time {
            //         let i = run_counter - last_trigger_time;
            //         self.state.trigger_interval = Some(i);
            //         // Reset the countdown
            //         self.state.auto_trigger_interval_countdown = i;
            //         ufmt::uwrite!(
            //             &mut self.serial,
            //             " uti: {:#?}",
            //             run_counter - last_trigger_time
            //         )
            //         .void_unwrap();
            //     }
            //     self.state.last_trigger_time = Some(run_counter);
            // }

            self.trigger_step(step_counter, sequence);
        } else if cfg!(feature = "auto_trigger") {
            ufmt::uwriteln!(&mut self.serial, "{} \r", step_counter).void_unwrap();

            // self.trigger_step(step_counter, sequence, led_controller);
            // arduino::delay_ms(2000);
        }
        // arduino::delay_ms(DELAY_TIME);
    }

    fn trigger_step(&mut self, step_counter: StepCounterType, sequence: Sequence) {
        let step_pointer: u8 = 0b00000001 << step_counter;
        self.set_dac(sequence, step_counter);

        self.set_all_step_pins_low();

        let output_pin = self.step_output_pins.get_mut(step_counter);
        let sequence_matches = sequence.matches(step_pointer);

        if let Some(pin) = output_pin {
            if sequence_matches {
                pin.set_high().void_unwrap();
            } else {
                pin.set_low().void_unwrap();
            };
        }
        self.led_controller
            .show_step(sequence, step_counter, sequence_matches)
            .unwrap();
    }

    fn set_dac(&mut self, sequence: Sequence, step_counter: StepCounterType) {
        let step_pointer: u8 = 0b00000001 << step_counter;

        match sequence.get_step(step_pointer) {
            None => self.dac.set(DacByte::new(0)),
            Some(step) => self.dac.set(step),
        }
    }

    fn set_all_step_pins_low(&mut self) {
        for output_pin in &mut self.step_output_pins {
            output_pin.set_low().void_unwrap();
        }
    }

    #[allow(unused)]
    fn set_pins_low(&self, output_pins: &mut [Pin<Output>]) {
        for output_pin in output_pins {
            output_pin.set_low().void_unwrap();
        }
    }

    fn check_sequence_change(&mut self) -> SequenceState {
        let sequence_state = self.sequence_controller.check_sequence_change();
        if sequence_state.did_change {
            ufmt::uwriteln!(
                &mut self.serial,
                "change sequence {}\r",
                sequence_state.sequence
            )
            .void_unwrap();

            self.clock_in.reset();

            self.sequence_change_output.set_high().void_unwrap();
            self.set_step_output_pins_for_sequence(sequence_state.sequence);

            self.led_controller.show_sequence(sequence_state.sequence);
        }

        sequence_state
    }

    fn set_step_output_pins_for_sequence(&mut self, sequence: Sequence) {
        for (i, step_output_pin) in self.step_output_pins.iter_mut().enumerate() {
            let current_bit = 0b00000001 << i;
            if sequence.matches(current_bit) {
                step_output_pin.set_high().void_unwrap();
            } else {
                step_output_pin.set_low().void_unwrap();
            }
        }
    }

    fn initialize_leds(&mut self) {
        let blank = Default::default();
        let initial_colors = color::get_initial_colors();
        // Workaround for the bright green led color on startup
        self.led_controller.write(blank).unwrap();
        self.led_controller.write(initial_colors).unwrap();
        self.led_controller.write(blank).unwrap();
        self.led_controller.write(initial_colors).unwrap();
    }

    #[allow(unused)]
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

    #[allow(unused)]
    fn test_colors(&mut self) {
        loop {
            match color_from_serial(&mut self.serial) {
                Ok(color) => {
                    self.led_controller.write([color; 8]).unwrap();
                }
                Err(_) => {}
            }
        }
    }
}
