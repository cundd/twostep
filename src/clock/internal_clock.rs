use crate::clock::{ClockTrait, ClockResult, StepCounterType};
use crate::millis::millis;
use crate::sequence::Sequence;
use crate::serial_wrapper::SerialWrapper;
use crate::trigger_state::TriggerState;
use arduino_uno::hal::port::mode::InputMode;
use void::ResultVoidExt;

pub struct InternalClock {
    /// Interval between clock-triggers in milliseconds
    interval: u32,
    /// Timestamp in milliseconds when the last clock-trigger happened
    last_tick_timestamp: u32,
    /// Duration in milliseconds how long the trigger will be held high
    hold_time: u32,
    step_counter: StepCounterType,
}

impl InternalClock {
    /// Create a new internal clock which triggers every `interval` milliseconds
    pub fn new(interval: u32, hold_time: u32) -> Self {
        Self {
            step_counter: 0,
            last_tick_timestamp: 0,
            interval,
            hold_time,
        }
    }

    fn get_new_trigger_state(&mut self) -> TriggerState {
        let current_timestamp = millis();
        if self.last_tick_timestamp + self.interval <= current_timestamp {
            self.last_tick_timestamp = current_timestamp;
            TriggerState::Rise
        } else if self.last_tick_timestamp + self.hold_time <= current_timestamp {
            TriggerState::Fall
        } else {
            TriggerState::Unchanged
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

impl ClockTrait for InternalClock {
    fn check<IMODE: InputMode>(
        &mut self,
        serial: &mut SerialWrapper<IMODE>,
        sequence: Sequence,
    ) -> ClockResult {
        let trigger_state = self.get_new_trigger_state();
        if let TriggerState::Rise = trigger_state {
            self.advance_step_counter(sequence);

            ufmt::uwriteln!(serial, "CLK!\r").void_unwrap();
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
