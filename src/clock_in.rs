use crate::sequence::Sequence;
use crate::trigger_state::TriggerState;
use arduino_uno::hal::port::mode::{Floating, Input};
use arduino_uno::hal::port::portb::PB4;
use arduino_uno::prelude::*;
use void::ResultVoidExt;

type ClockInput = PB4<Input<Floating>>;
pub type StepCounterType = usize;

pub struct ClockIn {
    input: ClockInput,
    step_counter: StepCounterType,
    last_important_trigger_state: TriggerState,
}

impl ClockIn {
    pub fn new(input: ClockInput) -> Self {
        Self {
            input,
            step_counter: 0,
            last_important_trigger_state: TriggerState::Unchanged,
        }
    }

    pub fn check(&mut self, sequence: Sequence) -> (TriggerState, StepCounterType) {
        let state = self.get_new_trigger_state();
        match state {
            TriggerState::Rise => {
                self.advance_step_counter(sequence);
                self.last_important_trigger_state = state
            }
            TriggerState::Fall => self.last_important_trigger_state = state,
            TriggerState::Unchanged => {}
        }
        (state, self.step_counter)
    }

    pub fn reset(&mut self) {
        self.step_counter = 0
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
