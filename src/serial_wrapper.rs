use arduino_uno::hal::port::mode::InputMode;
use arduino_uno::Serial;
use ufmt::uWrite;

pub struct SerialWrapper<IMODE: InputMode> {
    debug: bool,
    serial: Serial<IMODE>,
}

impl<IMODE: InputMode> SerialWrapper<IMODE> {
    pub fn new(debug: bool, serial: Serial<IMODE>) -> Self {
        SerialWrapper { debug, serial }
    }

    pub fn get_serial(&mut self) -> &mut Serial<IMODE> {
        &mut self.serial
    }
}

impl<IMODE: InputMode> uWrite for SerialWrapper<IMODE> {
    type Error = void::Void;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        if cfg!(feature = "debug") && self.debug {
            self.serial.write_str(s)
        } else {
            Ok(())
        }
    }
}
