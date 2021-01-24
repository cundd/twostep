use crate::RGB_LED_COUNT;
use smart_leds::RGB8;

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
// pub const fn darken(color: Color, percent: u8) -> Color {
//     scale(color, -1 * percent as i32)
// }
// pub const fn scale(color: Color, percent: i32) -> Color {
//     // let factor =
//     Color {
//         r: color.r.checked_add(color.r as i32 * percent),
//         g: color.g.checked_add(color.g as i32 * percent),
//         b: color.b.checked_add(color.b as i32 * percent),
//     }
// }
