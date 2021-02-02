use super::ClockTrait;
#[allow(unused_imports)]
use crate::clock::{Clock, ExternalClock, InternalClock};
use crate::{DELAY_TIME, USE_INTERNAL_CLOCK};
use arduino_uno::hal::port::mode::{Floating, Input};
use arduino_uno::hal::port::portd::PD2;
use core::marker::PhantomData;

pub struct ClockFactory<CLOCK: ClockTrait> {
    _phantom: PhantomData<CLOCK>,
}

impl<CLOCK: ClockTrait> ClockFactory<CLOCK> {
    pub fn build(&self, trigger_input: PD2<Input<Floating>>) -> Clock {
        if USE_INTERNAL_CLOCK {
            Clock::Internal(InternalClock::new(250, DELAY_TIME))
        } else {
            Clock::External(ExternalClock::new(trigger_input))
        }
    }

    pub fn new() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}

impl<CLOCK: ClockTrait> Default for ClockFactory<CLOCK> {
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}
