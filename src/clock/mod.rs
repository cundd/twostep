use crate::sequence::Sequence;
use crate::trigger_state::TriggerState;

mod external_clock;

use crate::serial_wrapper::SerialWrapper;
use arduino_uno::hal::port::mode::InputMode;
pub use external_clock::ExternalClock;

pub type StepCounterType = usize;

pub struct ClockResult {
    pub trigger_state: TriggerState,
    pub step_counter: StepCounterType,
}

pub trait Clock {
    fn check<IMODE: InputMode>(
        &mut self,
        serial: &mut SerialWrapper<IMODE>,
        sequence: Sequence,
    ) -> ClockResult;

    fn reset(&mut self);
}
