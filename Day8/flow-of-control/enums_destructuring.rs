#[allow(dead_code)]

enum Color {
    // These 3 are specified solely by their name.
    Red,
    Blue,
    Green,
    // These likewise tie `u32` tuples to different names: color models.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    let color = Color::RGB(122, 17, 40);

    match color {
        Color::Red => println!(" The color is red"),
        Color::Blue => println!(" The color is blue"),
        Color::Green => println!(" The color is green"),
        Color::RGB(r, g, b) => println!("Red: {}, Green: {}, Blue: {}", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, Saturation: {}, Value: {}", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, Saturation: {}, Lightness: {}", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, Magneto: {}, Yellow: {}", c, m, y),
        Color::CMYK(c, m, y, k) => println!(
            "Cyan: {}, Magneto: {}, Yellow: {}, Key(Black): {}",
            c, m, y, k
        ),
    }
}
