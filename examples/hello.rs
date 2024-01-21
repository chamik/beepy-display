use embedded_graphics::{
    mono_font::{ascii::FONT_9X18_BOLD, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use beepy_display::BeepyDisplay;
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut display = BeepyDisplay::new("/dev/fb1".into())?;

    let text_style = MonoTextStyle::new(&FONT_9X18_BOLD, BinaryColor::On);
    Text::new("Hello!", Point::new(20, 30), text_style)
        .draw(&mut display)?;

    display.flush();

    std::io::stdin().read_line(&mut String::new()).unwrap();

    Ok(())
}
