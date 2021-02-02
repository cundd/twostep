use super::{ClockResult, ClockTrait, ExternalClock, InternalClock};
use crate::sequence::Sequence;
use crate::serial_wrapper::SerialWrapper;
use arduino_uno::hal::port::mode::InputMode;

pub enum Clock {
    #[allow(unused)]
    External(ExternalClock),
    #[allow(unused)]
    Internal(InternalClock),
}
impl ClockTrait for Clock {
    fn check<IMODE: InputMode>(
        &mut self,
        serial: &mut SerialWrapper<IMODE>,
        sequence: Sequence,
    ) -> ClockResult {
        match self {
            Clock::External(c) => c.check(serial, sequence),
            Clock::Internal(c) => c.check(serial, sequence),
        }
    }

    fn reset(&mut self) {
        match self {
            Clock::External(c) => c.reset(),
            Clock::Internal(c) => c.reset(),
        }
    }
}
