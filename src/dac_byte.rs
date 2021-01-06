use ufmt::{derive::uDebug, uDebug, uDisplay, uWrite, Formatter};

#[derive(Copy, Clone, uDebug)]
pub struct DacByte(u8);

const MAX: u8 = 0b00001111; // = 15
impl DacByte {
    pub const fn new(input: u8) -> Self {
        if input > MAX {
            panic!("DAC bit overflow");
        }
        Self(input)
    }

    pub const fn max() -> Self {
        Self(MAX)
    }

    pub const fn half() -> Self {
        Self(MAX / 2)
    }

    pub const fn min() -> Self {
        Self(0b00000000)
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn matches(&self, step: u8) -> bool {
        (self.0 & step) == step
    }
}

impl uDisplay for DacByte {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <DacByte as uDebug>::fmt(self, f)
    }
}

// impl BitAnd for DacByte {
//     type Output = ();
//
//     fn bitand(self, rhs: Self) -> Self::Output {
//         unimplemented!()
//     }
// }
