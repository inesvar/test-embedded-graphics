#[allow(unused_imports)]
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{
        Circle, CornerRadii, Ellipse, Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle,
        RoundedRectangle, StrokeAlignment, StrokeStyle, Triangle,
    },
};
use embedded_graphics_simulator::{OutputImage, OutputSettingsBuilder, SimulatorDisplay};

const PADDING: i32 = 16;
const RECTANGLE_SIZES: [Size; 3] = [Size::new(9, 64), Size::new(42, 43), Size::new(47, 24)];
const COLORS: [Rgb888; 3] = [
    Rgb888::new(0xF8, 0xF4, 0xE3),
    Rgb888::new(0x58, 0xA4, 0xB0),
    Rgb888::new(0x0C, 0x7C, 0x59),
];

fn draw_column_of_3_rectangles<D>(
    target: &mut D,
    width: u32,
    stroke_alignment: StrokeAlignment,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb888>,
{
    for i in 0..3 {
        let rectangle_top_left = Point::new(64, 64) - RECTANGLE_SIZES[i];
        let style =
            PrimitiveStyleBuilder::from(&PrimitiveStyle::with_stroke(COLORS[(i + 1) % 3], width))
                .stroke_alignment(stroke_alignment)
                .stroke_style(StrokeStyle::Dotted)
                .fill_color(COLORS[i])
                .build();

        Rectangle::new(rectangle_top_left, RECTANGLE_SIZES[i])
            .translate(Point::new(0, (64 + PADDING) * i as i32))
            .into_styled(style)
            .draw(target)?;
    }

    Ok(())
}

fn draw_column_of_2_thin_squares<D>(target: &mut D, rectangle_size: Size) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb888>,
{
    for i in 0..2 {
        let rectangle_top_left = Point::new(16, 16) - rectangle_size;
        let fill_style =
            PrimitiveStyleBuilder::from(&PrimitiveStyle::with_stroke(COLORS[i % 3], 1 + i as u32))
                .build();
        let dot_style = PrimitiveStyleBuilder::from(&fill_style)
            .stroke_color(COLORS[(i + 1) % 3])
            .stroke_style(StrokeStyle::Dotted)
            .build();

        let rectangle = Rectangle::new(rectangle_top_left, rectangle_size)
            .translate(Point::new(0, (16 + PADDING) * i as i32));
        rectangle.into_styled(fill_style).draw(target)?;
        rectangle.into_styled(dot_style).draw(target)?;
    }

    Ok(())
}

fn draw_rectangles_with_varying_border_width(
    stroke_alignment: StrokeAlignment,
) -> Option<OutputImage<Rgb888>> {
    let mut display = SimulatorDisplay::<Rgb888>::new(Size::new(512, 256));

    let mut position = Point::new(10, 16);
    draw_column_of_3_rectangles(&mut display.translated(position), 1, stroke_alignment).ok()?;

    position.x += 64 + PADDING;
    draw_column_of_3_rectangles(&mut display.translated(position), 2, stroke_alignment).ok()?;

    position.x += 64 + PADDING;
    draw_column_of_3_rectangles(&mut display.translated(position), 3, stroke_alignment).ok()?;

    position.x += 64 + PADDING;
    draw_column_of_3_rectangles(&mut display.translated(position), 4, stroke_alignment).ok()?;

    position.x += 64 + PADDING;
    draw_column_of_3_rectangles(&mut display.translated(position), 8, stroke_alignment).ok()?;

    position.x += 64 + PADDING;
    draw_column_of_3_rectangles(&mut display.translated(position), 16, stroke_alignment).ok()?;

    let output_settings = OutputSettingsBuilder::new().build();
    Some(display.to_rgb_output_image(&output_settings))
}

fn draw_squares_with_varying_size() -> Option<OutputImage<Rgb888>> {
    let mut display = SimulatorDisplay::<Rgb888>::new(Size::new(256, 128));

    let mut position = Point::new(8, 8);

    for size in 0..=12 {
        draw_column_of_2_thin_squares(&mut display.translated(position), Size::new_equal(size))
            .ok()?;

        position.x += 16 + PADDING;
    }

    position = Point::new(8, 64);

    for size in 12..=24 {
        draw_column_of_2_thin_squares(&mut display.translated(position), Size::new_equal(size))
            .ok()?;

        position.x += 16 + PADDING;
    }

    let output_settings = OutputSettingsBuilder::new().build();
    Some(display.to_rgb_output_image(&output_settings))
}

fn main() -> Result<(), ()> {
    draw_rectangles_with_varying_border_width(StrokeAlignment::Center)
        .expect("Oops")
        .save_png("./screenshots_v9/alignment_center.png")
        .unwrap();

    draw_rectangles_with_varying_border_width(StrokeAlignment::Outside)
        .expect("Oops")
        .save_png("./screenshots_v9/alignment_outside.png")
        .unwrap();

    draw_rectangles_with_varying_border_width(StrokeAlignment::Inside)
        .expect("Oops")
        .save_png("./screenshots_v9/alignment_inside.png")
        .unwrap();

    draw_squares_with_varying_size()
        .expect("Oops")
        .save_png("./screenshots_v9/small_border_width.png")
        .unwrap();

    Ok(())
}
