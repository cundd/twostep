use ufmt::{derive::uDebug, uDebug, uDisplay, uWrite, Formatter};

#[derive(Copy, Clone, PartialOrd, PartialEq, uDebug)]
pub enum TriggerState {
    Rise,
    Fall,
    Unchanged,
}

impl uDisplay for TriggerState {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <TriggerState as uDebug>::fmt(self, f)
    }
}
