use crate::clock::{Clock, ClockResult, StepCounterType};
use crate::sequence::Sequence;
use crate::serial_wrapper::SerialWrapper;
use crate::trigger_state::TriggerState;
use arduino_uno::hal::port::mode::{Floating, Input, InputMode};
use arduino_uno::hal::port::portd::PD2;
use arduino_uno::prelude::*;
use void::ResultVoidExt;

type ClockInput = PD2<Input<Floating>>;

pub struct ExternalClock {
    input: ClockInput,
    step_counter: StepCounterType,
    last_important_trigger_state: TriggerState,
}

impl ExternalClock {
    pub fn new(input: ClockInput) -> Self {
        Self {
            input,
            step_counter: 0,
            last_important_trigger_state: TriggerState::Unchanged,
        }
    }

    fn get_new_trigger_state(&self) -> TriggerState {
        let trigger_pin_state = self.input.is_high().void_unwrap();

        match trigger_pin_state {
            true if self.last_important_trigger_state != TriggerState::Rise => TriggerState::Rise,
            false if self.last_important_trigger_state != TriggerState::Fall => TriggerState::Fall,
            _ => TriggerState::Unchanged,
        }
    }

    fn advance_step_counter(&mut self, sequence: Sequence) {
        if self.step_counter < (sequence.len() as StepCounterType) - 1 {
            self.step_counter += 1
        } else {
            self.step_counter = 0
        }
    }
}

impl Clock for ExternalClock {
    fn check<IMODE: InputMode>(
        &mut self,
        serial: &mut SerialWrapper<IMODE>,
        sequence: Sequence,
    ) -> ClockResult {
        let trigger_state = self.get_new_trigger_state();
        match trigger_state {
            TriggerState::Rise => {
                self.advance_step_counter(sequence);

                ufmt::uwriteln!(serial, "CLK!\r").void_unwrap();

                self.last_important_trigger_state = trigger_state
            }
            TriggerState::Fall => self.last_important_trigger_state = trigger_state,
            TriggerState::Unchanged => {}
        }

        ClockResult {
            trigger_state,
            step_counter: self.step_counter,
        }
    }

    fn reset(&mut self) {
        self.step_counter = 0
    }
}
