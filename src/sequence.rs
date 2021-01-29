use ufmt::{derive::uDebug, uDebug, uDisplay, uWrite, Formatter};

use crate::dac_byte::DacByte;

#[derive(Copy, Clone, uDebug)]
pub struct Sequence {
    length: usize,
    s1: Option<DacByte>,
    s2: Option<DacByte>,
    s3: Option<DacByte>,
    s4: Option<DacByte>,
    s5: Option<DacByte>,
    s6: Option<DacByte>,
    s7: Option<DacByte>,
    s8: Option<DacByte>,
}

impl Sequence {
    pub const fn new(
        length: usize,
        s1: Option<DacByte>,
        s2: Option<DacByte>,
        s3: Option<DacByte>,
        s4: Option<DacByte>,
        s5: Option<DacByte>,
        s6: Option<DacByte>,
        s7: Option<DacByte>,
        s8: Option<DacByte>,
    ) -> Self {
        Self {
            length,
            s1,
            s2,
            s3,
            s4,
            s5,
            s6,
            s7,
            s8,
        }
    }

    pub fn get_step(&self, step: u8) -> Option<DacByte> {
        match step {
            0b00000001 => self.s1,
            0b00000010 => self.s2,
            0b00000100 => self.s3,
            0b00001000 => self.s4,
            0b00010000 => self.s5,
            0b00100000 => self.s6,
            0b01000000 => self.s7,
            0b10000000 => self.s8,
            _ => None,
        }
    }

    pub fn matches(&self, step: u8) -> bool {
        match step {
            0b00000001 => self.s1.is_some() && self.s1.unwrap().value() > 0,
            0b00000010 => self.s2.is_some() && self.s2.unwrap().value() > 0,
            0b00000100 => self.s3.is_some() && self.s3.unwrap().value() > 0,
            0b00001000 => self.s4.is_some() && self.s4.unwrap().value() > 0,
            0b00010000 => self.s5.is_some() && self.s5.unwrap().value() > 0,
            0b00100000 => self.s6.is_some() && self.s6.unwrap().value() > 0,
            0b01000000 => self.s7.is_some() && self.s7.unwrap().value() > 0,
            0b10000000 => self.s8.is_some() && self.s8.unwrap().value() > 0,
            _ => {
                use void::ResultVoidExt;

                let mut serial: arduino_uno::Serial<arduino_uno::hal::port::mode::Floating> =
                    unsafe { core::mem::MaybeUninit::uninit().assume_init() };

                ufmt::uwriteln!(&mut serial, "Step {} out of bounds!\r", step).void_unwrap();

                panic!()
            }
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

impl uDisplay for Sequence {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <Sequence as uDebug>::fmt(self, f)
    }
}

#[macro_export]
macro_rules! seq {
    ($b1:expr $(,)?) => {
        Sequence::new(
            1,
            Some(DacByte::new($b1)),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
    };
    ($b1:expr, $b2:expr $(,)?) => {
        Sequence::new(
            2,
            Some(DacByte::new($b1)),
            Some(DacByte::new($b2)),
            None,
            None,
            None,
            None,
            None,
            None,
        )
    };
    ($b1:expr, $b2:expr, $b3:expr $(,)?) => {
        Sequence::new(
            3,
            Some(DacByte::new($b1)),
            Some(DacByte::new($b2)),
            Some(DacByte::new($b3)),
            None,
            None,
            None,
            None,
            None,
        )
    };
    ($b1:expr, $b2:expr, $b3:expr, $b4:expr $(,)?) => {
        Sequence::new(
            4,
            Some(DacByte::new($b1)),
            Some(DacByte::new($b2)),
            Some(DacByte::new($b3)),
            Some(DacByte::new($b4)),
            None,
            None,
            None,
            None,
        )
    };
    ($b1:expr, $b2:expr, $b3:expr, $b4:expr, $b5:expr $(,)?) => {
        Sequence::new(
            5,
            Some(DacByte::new($b1)),
            Some(DacByte::new($b2)),
            Some(DacByte::new($b3)),
            Some(DacByte::new($b4)),
            Some(DacByte::new($b5)),
            None,
            None,
            None,
        )
    };
    ($b1:expr, $b2:expr, $b3:expr, $b4:expr, $b5:expr, $b6:expr $(,)?) => {
        Sequence::new(
            6,
            Some(DacByte::new($b1)),
            Some(DacByte::new($b2)),
            Some(DacByte::new($b3)),
            Some(DacByte::new($b4)),
            Some(DacByte::new($b5)),
            Some(DacByte::new($b6)),
            None,
            None,
        )
    };
    ($b1:expr, $b2:expr, $b3:expr, $b4:expr, $b5:expr, $b6:expr, $b7:expr $(,)?) => {
        Sequence::new(
            7,
            Some(DacByte::new($b1)),
            Some(DacByte::new($b2)),
            Some(DacByte::new($b3)),
            Some(DacByte::new($b4)),
            Some(DacByte::new($b5)),
            Some(DacByte::new($b6)),
            Some(DacByte::new($b7)),
            None,
        )
    };
    ($b1:expr, $b2:expr, $b3:expr, $b4:expr, $b5:expr, $b6:expr, $b7:expr, $b8:expr $(,)?) => {
        Sequence::new(
            8,
            Some(DacByte::new($b1)),
            Some(DacByte::new($b2)),
            Some(DacByte::new($b3)),
            Some(DacByte::new($b4)),
            Some(DacByte::new($b5)),
            Some(DacByte::new($b6)),
            Some(DacByte::new($b7)),
            Some(DacByte::new($b8)),
        )
    };
}

mod test {
    use super::*;

    #[allow(unused)]
    const SEQUENCES: [Sequence; 8] = [
        seq!(1),
        seq!(15, 13),
        seq!(15, 13, 1),
        seq!(15, 13, 1, 0),
        seq!(15, 0, 1, 0, 15),
        seq!(15, 0, 1, 0, 15, 2),
        seq!(15, 0, 1, 0, 15, 2, 7),
        seq!(15, 0, 1, 0, 15, 2, 7, 15),
    ];
}
