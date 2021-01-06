use crate::dac_byte::DacByte;
use arduino_uno::hal::port::mode::Output;
use arduino_uno::hal::port::portc::{PC0, PC1, PC2, PC3};
use arduino_uno::prelude::*;

pub struct Dac {
    a0: PC0<Output>,
    a1: PC1<Output>,
    a2: PC2<Output>,
    a3: PC3<Output>,
}

impl Dac {
    pub fn new(a0: PC0<Output>, a1: PC1<Output>, a2: PC2<Output>, a3: PC3<Output>) -> Self {
        Self { a0, a1, a2, a3 }
    }

    pub(crate) fn set(&mut self, input: DacByte) {
        if input.matches(0b00000001) {
            self.a0.set_high()
        } else {
            self.a0.set_low()
        }
        .void_unwrap();

        if input.matches(0b00000010) {
            self.a1.set_high()
        } else {
            self.a1.set_low()
        }
        .void_unwrap();

        if input.matches(0b00000100) {
            self.a2.set_high()
        } else {
            self.a2.set_low()
        }
        .void_unwrap();

        if input.matches(0b00001000) {
            self.a3.set_high()
        } else {
            self.a3.set_low()
        }
        .void_unwrap();
    }
}
