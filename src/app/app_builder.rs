use crate::{millis, STEP_LED_COUNT};

use crate::app::{App, State};
use crate::clock::{Clock, ClockFactory, ClockTrait};
use crate::dac::Dac;
use crate::led_controller::LedController;
use crate::sequence_controller::SequenceController;
use crate::serial_wrapper::SerialWrapper;
use crate::trigger::TriggerFactory;
use arduino::prelude::*;
use arduino_uno as arduino;
use arduino_uno::hal::port::{mode, Pin};
use arduino_uno::{adc, spi};

pub trait AppBuilderTrait {
    type Clock: ClockTrait;

    fn build(
        clock_factory: ClockFactory<Self::Clock>,
        trigger_factory: TriggerFactory,
    ) -> App<Self::Clock>;
}

pub struct AppBuilder {}

impl AppBuilderTrait for AppBuilder {
    type Clock = Clock;

    fn build(
        clock_factory: ClockFactory<Self::Clock>,
        trigger_factory: TriggerFactory,
    ) -> App<Self::Clock> {
        let dp = arduino::Peripherals::take().unwrap();

        let mut pins = arduino::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

        let sequence_change_output = pins.d4.into_output(&mut pins.ddr);
        let step_output_pins: [Pin<mode::Output>; STEP_LED_COUNT] = [
            // pins.d2.into_output(&mut pins.ddr).downgrade(),
            // pins.d3.into_output(&mut pins.ddr).downgrade(),
            // pins.d4.into_output(&mut pins.ddr).downgrade(),
            pins.d5.into_output(&mut pins.ddr).downgrade(),
            pins.d6.into_output(&mut pins.ddr).downgrade(),
            pins.d7.into_output(&mut pins.ddr).downgrade(),
            pins.d8.into_output(&mut pins.ddr).downgrade(),
            pins.d9.into_output(&mut pins.ddr).downgrade(),
        ];

        let serial = SerialWrapper::new(
            if cfg!(feature = "debug") { true } else { false },
            arduino::Serial::new(
                dp.USART0,
                pins.d0,
                pins.d1.into_output(&mut pins.ddr),
                57600.into_baudrate(),
            ),
        );

        let mut adc = adc::Adc::new(dp.ADC, Default::default());
        let analog_input = Some(pins.a4.into_analog_input(&mut adc));

        let a0 = pins.a0.into_output(&mut pins.ddr);
        let a1 = pins.a1.into_output(&mut pins.ddr);
        let a2 = pins.a2.into_output(&mut pins.ddr);
        let a3 = pins.a3.into_output(&mut pins.ddr);
        let dac = Dac::new(a0, a1, a2, a3);

        let trigger_input = pins.d2.into_floating_input(&mut pins.ddr);
        let clock_in = clock_factory.build(trigger_input);

        let trigger_out = pins.d3.into_output(&mut pins.ddr);
        let trigger = trigger_factory.build(trigger_out);

        let sequence_change_input = pins.a5.into_pull_up_input(&mut pins.ddr);
        let sequence_controller = SequenceController::new(sequence_change_input);

        let (spi, _) = spi::Spi::new(
            dp.SPI,
            pins.d13.into_output(&mut pins.ddr),        // SCK
            pins.d11.into_output(&mut pins.ddr),        // MOSI
            pins.d12.into_pull_up_input(&mut pins.ddr), // MISO
            pins.d10.into_output(&mut pins.ddr),        // led_rx
            spi::Settings {
                clock: spi::SerialClockRate::OscfOver4,
                ..Default::default()
            },
        );
        let led_controller = LedController::new(spi);

        millis::millis_init(dp.TC0);

        // Enable interrupts globally
        unsafe { avr_device::interrupt::enable() };

        App {
            step_output_pins,
            sequence_change_output,
            adc,
            analog_input,
            serial,
            state: State {
                trigger_interval: None,
                auto_trigger_interval_countdown: 0,
                last_trigger_time: None,
            },
            dac,
            trigger,
            clock_in,
            led_controller,
            sequence_controller,
        }
    }
}
