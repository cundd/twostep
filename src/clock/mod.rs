use crate::sequence::Sequence;
use crate::trigger_state::TriggerState;

mod external_clock;

pub use external_clock::ExternalClock;

pub type StepCounterType = usize;

pub struct ClockResult {
    pub trigger_state: TriggerState,
    pub step_counter: StepCounterType,
}

pub trait Clock {
    fn check(&mut self, sequence: Sequence) -> ClockResult;

    fn reset(&mut self);
}
