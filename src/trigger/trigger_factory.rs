use crate::trigger::{Trigger, TriggerMode};
use arduino_uno::hal::port::mode::Output;
use arduino_uno::hal::port::portd::PD3;

pub struct TriggerFactory {}

impl TriggerFactory {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(&self, trigger_out: PD3<Output>) -> Trigger {
        // let trigger_mode: TriggerMode = TriggerMode::Hold;
        // let trigger_mode: TriggerMode = TriggerMode::Follow;
        let trigger_mode: TriggerMode = TriggerMode::Pulse;
        Trigger::new(trigger_out, trigger_mode)
    }
}
