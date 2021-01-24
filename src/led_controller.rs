use crate::color::{
    COLOR_CURRENT_MATCH, COLOR_CURRENT_NO_MATCH, COLOR_MATCH, COLOR_NO_MATCH, COLOR_UNMAPPED,
};
use crate::sequence::Sequence;
use crate::ws2812::prerendered::Ws2812;
use crate::RGB_LED_COUNT;
use arduino_uno::hal::port::mode::PullUp;
use arduino_uno::spi::Spi;
use smart_leds::{SmartLedsWrite, RGB8};

static mut OUTPUT_BUFFER: [u8; 136] = [0; 40 + (RGB_LED_COUNT * 12)];

pub struct LedController<'a> {
    outlet: Ws2812<'a, Spi<PullUp>>,
    last_data: [RGB8; RGB_LED_COUNT],
    _buffer: [u8; 40 + (RGB_LED_COUNT * 12)],
}

impl<'a> LedController<'a> {
    pub fn new(spi: Spi<PullUp>) -> Self {
        let data: [RGB8; RGB_LED_COUNT] = [RGB8::default(); RGB_LED_COUNT];
        let outlet = unsafe { Ws2812::new(spi, &mut OUTPUT_BUFFER) };
        Self {
            outlet,
            last_data: data,
            _buffer: [0; 40 + (RGB_LED_COUNT * 12)],
        }
    }

    // fn init_outlet(&mut self, spi: Spi<PullUp>) {
    //     let ws2812 = unsafe { Ws2812::new(spi, &mut OUTPUT_BUFFER_ST) };
    //     self.outlet = Some(ws2812);
    // }

    pub fn show_sequence(&mut self, sequence: Sequence) {
        self.write(self.data_for_sequence(sequence)).unwrap();
        //
        // let mut data: [RGB8; RGB_LED_COUNT] = [RGB8 { r: 0, g: 0, b: 0 }; RGB_LED_COUNT];
        //
        // for step_counter in 0..RGB_LED_COUNT {
        //     let step_pointer: u8 = 0b00000001 << step_counter;
        //     if sequence.matches(step_pointer) {
        //         data[step_counter] = RGB8 {
        //             r: 0x10,
        //             g: 0,
        //             b: 0x10,
        //         }
        //     }
        // }
        //
        // self.write(data).unwrap();
    }

    pub fn show_step(
        &mut self,
        sequence: Sequence,
        step_counter: usize,
        sequence_matches: bool,
    ) -> Result<(), ()> {
        let mut data = self.data_for_sequence(sequence);

        if sequence_matches {
            data[step_counter] = COLOR_CURRENT_MATCH;
        } else {
            data[step_counter] = COLOR_CURRENT_NO_MATCH;
        };
        self.write(data)
    }

    pub fn write(&mut self, data: [RGB8; RGB_LED_COUNT]) -> Result<(), ()> {
        if data != self.last_data {
            use arduino_uno::prelude::*;
            let mut serial: crate::arduino::Serial<crate::arduino::hal::port::mode::Floating> =
                unsafe { core::mem::MaybeUninit::uninit().assume_init() };

            ufmt::uwriteln!(&mut serial, "w").void_unwrap();

            match self.outlet.write(data.iter().cloned()) {
                Ok(_) => {
                    self.last_data = data;
                    Ok(())
                }
                Err(_) => Err(()),
            }
        } else {
            Ok(())
        }
    }

    pub fn data_for_sequence(&self, sequence: Sequence) -> [RGB8; RGB_LED_COUNT] {
        let mut data: [RGB8; RGB_LED_COUNT] = [COLOR_UNMAPPED; RGB_LED_COUNT];

        for step_counter in 0..RGB_LED_COUNT {
            let step_pointer: u8 = 0b00000001 << step_counter;
            if let Some(dac_byte) = sequence.get_step(step_pointer) {
                if dac_byte.value() > 0 {
                    data[step_counter] = COLOR_MATCH;
                } else {
                    data[step_counter] = COLOR_NO_MATCH;
                }
            }
        }
        data
    }
}
