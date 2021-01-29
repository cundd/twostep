use crate::dac_byte::DacByte;
use crate::serial_wrapper::SerialWrapper;
use crate::RGB_LED_COUNT;
use arduino_uno::hal::port::mode::InputMode;
use arduino_uno::prelude::*;
use smart_leds::hsv::Hsv;
use smart_leds::RGB8;
use void::ResultVoidExt;

pub type Color = RGB8;

pub const COLOR_UNMAPPED: Color = Color { r: 0, g: 0, b: 0 };
pub const COLOR_NO_MATCH: Color = Color { r: 2, g: 0, b: 0 };
pub const COLOR_MATCH: Color = Color { r: 10, g: 0, b: 2 };
pub const COLOR_CURRENT_NO_MATCH: Color = Color { r: 2, g: 0, b: 10 };
pub const COLOR_CURRENT_MATCH: Color = Color { r: 4, g: 0, b: 40 };

pub fn get_initial_colors() -> [RGB8; RGB_LED_COUNT] {
    let mut data = [COLOR_UNMAPPED; RGB_LED_COUNT];
    data[0] = COLOR_MATCH;
    data[1] = COLOR_MATCH;
    data[2] = COLOR_NO_MATCH;
    data[3] = COLOR_NO_MATCH;
    data[4] = COLOR_CURRENT_MATCH;
    data[5] = COLOR_CURRENT_MATCH;
    data[6] = COLOR_CURRENT_NO_MATCH;
    data[7] = COLOR_CURRENT_NO_MATCH;

    data
}

pub fn color_from_serial<IMODE: InputMode>(
    mut serial: &mut SerialWrapper<IMODE>,
) -> Result<Color, ()> {
    ufmt::uwriteln!(&mut serial, "Start reading color\r",).void_unwrap();

    let mut red_buffer: [u8; 2] = [Default::default(); 2];
    let mut green_buffer: [u8; 2] = [Default::default(); 2];
    let mut blue_buffer: [u8; 2] = [Default::default(); 2];

    collect_to_buffer_from_serial(serial, &mut red_buffer)?;
    collect_to_buffer_from_serial(serial, &mut green_buffer)?;
    collect_to_buffer_from_serial(serial, &mut blue_buffer)?;

    let _newline = nb::block!(serial.get_serial().read()).void_unwrap();

    ufmt::uwriteln!(
        &mut serial,
        "Buffers: {{ r:{:?} g:{:?} b:{:?} }}!\r",
        red_buffer,
        green_buffer,
        blue_buffer,
    )
    .void_unwrap();
    ufmt::uwriteln!(
        &mut serial,
        "Color: {{ r:{} g:{} b:{} }}!\r",
        u8_from_color_buffer(&red_buffer),
        u8_from_color_buffer(&green_buffer),
        u8_from_color_buffer(&blue_buffer),
    )
    .void_unwrap();

    Ok(Color {
        r: u8_from_color_buffer(&red_buffer),
        g: u8_from_color_buffer(&green_buffer),
        b: u8_from_color_buffer(&blue_buffer),
    })
}

fn u8_from_color_buffer(buffer: &[u8; 2]) -> u8 {
    match u8::from_str_radix(unsafe { core::str::from_utf8_unchecked(buffer) }, 16) {
        Ok(u) => u,
        Err(_) => 0,
    }
}

fn collect_to_buffer_from_serial<IMODE: InputMode>(
    mut serial: &mut SerialWrapper<IMODE>,
    buffer: &mut [u8; 2],
) -> Result<(), ()> {
    for i in 0..2 {
        let mut character: u8 = nb::block!(serial.get_serial().read()).void_unwrap() as u8;
        if character == 10 {
            ufmt::uwriteln!(&mut serial, "Unexpected newline\r").void_unwrap();
            character = 0
        }
        if character > 102 {
            ufmt::uwriteln!(&mut serial, "Hex value out of bounds\r").void_unwrap();
            return Err(());
        }
        buffer[i] = character;
    }
    Ok(())
}
