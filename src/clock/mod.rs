use crate::sequence::Sequence;
use crate::trigger_state::TriggerState;

mod clock;
mod clock_factory;
mod external_clock;
mod internal_clock;

use crate::serial_wrapper::SerialWrapper;
use arduino_uno::hal::port::mode::InputMode;
pub use clock::Clock;
pub use clock_factory::ClockFactory;
pub use external_clock::ExternalClock;
pub use internal_clock::InternalClock;

pub type StepCounterType = usize;

pub struct ClockResult {
    pub trigger_state: TriggerState,
    pub step_counter: StepCounterType,
}

pub trait ClockTrait {
    fn check<IMODE: InputMode>(
        &mut self,
        serial: &mut SerialWrapper<IMODE>,
        sequence: Sequence,
    ) -> ClockResult;

    fn reset(&mut self);
}
