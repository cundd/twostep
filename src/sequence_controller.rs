use crate::sequence::Sequence;
use crate::{SequencesType, SEQUENCES};
use arduino_uno::hal::port::mode::{Input, PullUp};
use arduino_uno::hal::port::portc::PC5;
use arduino_uno::prelude::*;

type SequenceChangeInput = PC5<Input<PullUp>>;

pub struct SequenceController {
    sequences: &'static SequencesType,
    sequence_change_input: SequenceChangeInput,
    sequence_pointer: usize,
    last_sequence_change_state: bool,
}

pub struct SequenceState {
    pub sequence: Sequence,
    pub sequence_pointer: usize,
    pub did_change: bool,
}

impl SequenceController {
    pub fn new(sequence_change_input: SequenceChangeInput) -> Self {
        Self {
            sequences: &SEQUENCES,
            sequence_change_input,
            sequence_pointer: 0,
            last_sequence_change_state: false,
        }
    }

    pub fn check_sequence_change(&mut self) -> SequenceState {
        let last_sequence_change_trigger_state = self.last_sequence_change_state;
        let sequences = self.sequences;
        let mut new_sequence_pointer = self.sequence_pointer;

        let sequence_change_input: bool = self.sequence_change_input.is_low().void_unwrap();
        let did_change = sequence_change_input && false == last_sequence_change_trigger_state;

        if did_change {
            new_sequence_pointer += 1;
            if sequences.len() <= new_sequence_pointer {
                new_sequence_pointer = 0;
            }

            self.sequence_pointer = new_sequence_pointer;
        }

        self.last_sequence_change_state = sequence_change_input;

        SequenceState {
            sequence: self.sequences[self.sequence_pointer],
            sequence_pointer: self.sequence_pointer,
            did_change,
        }
    }

    #[allow(unused)]
    pub fn get_sequence(&self) -> Sequence {
        self.sequences[self.sequence_pointer]
    }
}
