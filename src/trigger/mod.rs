mod trigger_factory;
use crate::clock::StepCounterType;
use crate::millis::millis;
use crate::scheduler::{Task, TaskId};
use crate::sequence::Sequence;
use crate::trigger_state::TriggerState;
use crate::DELAY_TIME;
use arduino_uno::hal::port::mode::Output;
use arduino_uno::hal::port::portd::PD3;
use arduino_uno::prelude::*;
pub use trigger_factory::TriggerFactory;
use void::{ResultVoidExt, Void};

type TriggerOutput = PD3<Output>;

const HIGH: bool = true;
const LOW: bool = false;

#[derive(Copy, Clone, PartialOrd, PartialEq)]
#[allow(unused)]
pub enum TriggerMode {
    /// Set the trigger output LOW when the external trigger goes low
    Follow,
    /// Hold the trigger outputs state until the next external trigger
    Hold,
    /// Pulse the trigger output for 2ms (`DELAY_TIME`)
    Pulse,
}

pub struct Trigger {
    output: TriggerOutput,
    trigger_mode: TriggerMode,
    last_trigger_state: TriggerState,
    scheduled_task: Option<Task<TriggerTask>>,
}

impl Trigger {
    pub fn new(output: TriggerOutput, trigger_mode: TriggerMode) -> Self {
        Self {
            output,
            trigger_mode,
            last_trigger_state: TriggerState::Unchanged,
            scheduled_task: None,
        }
    }

    pub fn check(
        &mut self,
        state: TriggerState,
        step_counter: StepCounterType,
        sequence: Sequence,
    ) {
        match state {
            TriggerState::Rise => {
                let step_pointer: u8 = 0b00000001 << step_counter;
                self.set_output(if sequence.matches(step_pointer) {
                    HIGH
                } else {
                    LOW
                })
                .void_unwrap();

                if self.trigger_mode == TriggerMode::Pulse {
                    self.scheduled_task =
                        Some(Task::new(TriggerTask::SetOff, millis() + DELAY_TIME as u32));
                }
            }
            TriggerState::Fall => {
                match self.trigger_mode {
                    TriggerMode::Follow => self.set_output(LOW).void_unwrap(),
                    TriggerMode::Hold => { /* Do nothing; wait for the next external trigger */ }
                    TriggerMode::Pulse => { /* Do nothing; delay was set using Arduino library */ }
                }
            }
            TriggerState::Unchanged => {}
        }
        self.last_trigger_state = state;
    }

    pub fn check_scheduled(
        &mut self,
        millis: u32,
        _state: TriggerState,
        _step_counter: StepCounterType,
        _sequence: Sequence,
    ) {
        if let Some(task) = &self.scheduled_task {
            if task.id == TriggerTask::SetOff && millis >= task.timestamp {
                self.set_output(LOW).void_unwrap();
                self.scheduled_task = None
            }
        }
    }

    fn set_output(&mut self, value: bool) -> Result<(), Void> {
        if value == HIGH {
            self.output.set_high()
        } else {
            self.output.set_low()
        }
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum TriggerTask {
    SetOff,
}

impl TaskId for TriggerTask {}
