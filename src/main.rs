// Color datatype using RGB representation

// TODO: you will need to derive some traits for this struct
struct Color {
    // TODO: add fields: red, green, blue of type u8
}

impl Color {

    // Base Colors
    const BLACK: Color = Color {
        red: 0,
        green: 0,
        blue: 0,
    };
    const WHITE: Color = Color {
        red: 255,
        green: 255,
        blue: 255,
    };
    const RED: Color = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    const GREEN: Color = Color {
        red: 0,
        green: 255,
        blue: 0,
    };
    const BLUE: Color = Color {
        red: 0,
        green: 0,
        blue: 255,
    };
    const YELLOW: Color = Color {
        red: 255,
        green: 255,
        blue: 0,
    };
    const CYAN: Color = Color {
        red: 0,
        green: 255,
        blue: 255,
    };
    const MAGENTA: Color = Color {
        red: 255,
        green: 0,
        blue: 255,
    };

    // TODO: define (parameters/return type) and implement a constructor function
    //       new() that takes three u8 values as parameters and returns a Color
    fn new() {

    }

    // TODO: define (parameters/return type) and implement an alternative 
    //      constructor function from_name() that takes a &str as parameter and
    //      returns an Option<Color>. The parameter is a case-insensitive string
    //      that can be one of the base colors (e.g., "black", "white", "red")
    fn from_name() {
    }

    // TODO: define (parameters/return type) and implement an alternative
    //      constructor function from_hex() that takes a &str as parameter and
    //      returns a Color. The parameter is a string of the form "#RRGGBB".
    //      You will need to learn some basic string processing on your own here.
    fn from_hex() {
    }

    // TODO: define (parameters/return type) and implement a method to_hex()
    //       The method should return a String of the form "#RRGGBB".
    fn to_hex() {
    }

    // TODO: define (parameters/return type) and implement a method distance()
    //       that takes a reference to another Color and returns a f64. The
    //       method should return the Euclidean distance between the two colors.
    fn distance() {
    }

    // TODO: define (parameters/return type) and implement a method closest_base_color()
    //       that returns the closest base color (closest to to this instance).
    fn closest_base_color() {
    }

    // TODO: define (parameters/return type) and implement a method to_grayscale()
    //       that returns a new Color that is the grayscale version of this color.
    //       The grayscale version is computed as 0.3*red + 0.59*green + 0.11*blue.
    fn to_grayscale() {
    }

    // TODO: define (parameters/return type) and implement a method darken(). The
    //       method should modify the current color by darkening it by the given
    //       factor (0.0 - 1.0) by multiplying each of the red, green, and blue.
    //       Higher than 1.0 factors should result in a black color.
    fn darken() {
    }

    // TODO: define (parameters/return type) and implement a method mix() that
    //       takes a reference to another Color and returns a new Color that is
    //       the average of the two (current instance and the other) colors. 
    fn mix() {
    }
}

// You can use the main function as your playground
// Feel free to edit/use it for experimenting with your code
// The main function will not be tested/graded in this file
fn main() {}

//////////////////////////////////////////////////////////////////////////////
// DO NOT EDIT BELOW THIS LINE
#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    const REL_TOL: f64 = 1e-9;

    #[test]
    fn test_new() {
        let c = Color::new(0, 0, 0);
        assert_eq!(c.red, 0);
        assert_eq!(c.green, 0);
        assert_eq!(c.blue, 0);

        let c = Color::new(255, 255, 255);
        assert_eq!(c.red, 255);
        assert_eq!(c.green, 255);
        assert_eq!(c.blue, 255);

        let c = Color::new(7, 13, 74);
        assert_eq!(c.red, 7);
        assert_eq!(c.green, 13);
        assert_eq!(c.blue, 74);
    }

    #[test]
    fn test_debug() {
        let c = Color::new(7, 13, 74);
        assert_eq!(format!("{:?}", c), "Color { red: 7, green: 13, blue: 74 }");
    }

    #[test]
    fn test_eq() {
        let c = Color::new(7, 13, 74);
        let d = Color::new(7, 13, 74);
        let e = Color::new(7, 13, 75);
        assert_eq!(c, d);
        assert_ne!(c, e);
    }

    #[test]
    fn test_clone() {
        let c = Color::new(7, 13, 74);
        let d = c.clone();
        assert_eq!(c, d);
    }

    #[test]
    fn test_copy() {
        let c = Color::new(7, 13, 74);
        let d = c;
        assert_eq!(c, d);
    }

    #[test]
    fn test_from_name() {
        assert_eq!(Color::from_name("black"), Some(Color::BLACK));
        assert_eq!(Color::from_name("white"), Some(Color::WHITE));
        assert_eq!(Color::from_name("red"), Some(Color::RED));
        assert_eq!(Color::from_name("green"), Some(Color::GREEN));
        assert_eq!(Color::from_name("blue"), Some(Color::BLUE));
        assert_eq!(Color::from_name("yellow"), Some(Color::YELLOW));
        assert_eq!(Color::from_name("cyan"), Some(Color::CYAN));
        assert_eq!(Color::from_name("magenta"), Some(Color::MAGENTA));
        assert_eq!(Color::from_name("Black"), Some(Color::BLACK));
        assert_eq!(Color::from_name("White"), Some(Color::WHITE));
        assert_eq!(Color::from_name("Red"), Some(Color::RED));
        assert_eq!(Color::from_name("Green"), Some(Color::GREEN));
        assert_eq!(Color::from_name("Blue"), Some(Color::BLUE));
        assert_eq!(Color::from_name("Yellow"), Some(Color::YELLOW));
        assert_eq!(Color::from_name("Cyan"), Some(Color::CYAN));
        assert_eq!(Color::from_name("Magenta"), Some(Color::MAGENTA));
        assert_eq!(Color::from_name("BLACK"), Some(Color::BLACK));
        assert_eq!(Color::from_name("WHITE"), Some(Color::WHITE));
        assert_eq!(Color::from_name("RED"), Some(Color::RED));
        assert_eq!(Color::from_name("GREEN"), Some(Color::GREEN));
        assert_eq!(Color::from_name("BLUE"), Some(Color::BLUE));
        assert_eq!(Color::from_name("YELLOW"), Some(Color::YELLOW));
        assert_eq!(Color::from_name("CYAN"), Some(Color::CYAN));
        assert_eq!(Color::from_name("MAGENTA"), Some(Color::MAGENTA));
        assert_eq!(Color::from_name("black "), None);
        assert_eq!(Color::from_name(" white"), None);
        assert_eq!(Color::from_name("red "), None);
        assert_eq!(Color::from_name(" green"), None);
        assert_eq!(Color::from_name("blue "), None);
        assert_eq!(Color::from_name(" yellow"), None);
        assert_eq!(Color::from_name("cyan "), None);
        assert_eq!(Color::from_name(""), None);
    }

    #[test]
    fn test_from_hex() {
        assert_eq!(Color::from_hex("#000000"), Color::BLACK);
        assert_eq!(Color::from_hex("#FFFFFF"), Color::WHITE);
        assert_eq!(Color::from_hex("#ff0000"), Color::RED);
        assert_eq!(Color::from_hex("#123456"), Color::new(18, 52, 86));
    }

    #[test]
    fn test_to_hex() {
        assert_eq!(Color::BLACK.to_hex(), "#000000");
        assert_eq!(Color::WHITE.to_hex(), "#FFFFFF");
        assert_eq!(Color::RED.to_hex(), "#FF0000");
        assert_eq!(Color::new(18, 52, 86).to_hex(), "#123456");
    }

    #[test]
    fn test_distance() {
        let c = Color::new(0, 0, 0);
        let d = Color::new(0, 0, 0);
        assert_relative_eq!(c.distance(&d), 0.0, max_relative = REL_TOL);

        let c = Color::new(0, 0, 0);
        let d = Color::new(255, 255, 255);
        assert_relative_eq!(c.distance(&d), 441.6729559300637, max_relative = REL_TOL);

        let c = Color::new(255, 255, 255);
        let d = Color::new(0, 0, 0);
        assert_relative_eq!(c.distance(&d), 441.6729559300637, max_relative = REL_TOL);

        let c = Color::new(0, 0, 0);
        let d = Color::new(255, 0, 0);
        assert_relative_eq!(c.distance(&d), 255.0, max_relative = REL_TOL);

        let c = Color::new(7, 13, 74);
        let d = Color::new(76, 12, 11);
        assert_relative_eq!(c.distance(&d), 93.43982020530648, max_relative = REL_TOL);
    }

    #[test]
    fn test_closest_base_color() {
        let c = Color::new(0, 0, 0);
        assert_eq!(c.closest_base_color(), Color::BLACK);

        let c = Color::new(255, 255, 255);
        assert_eq!(c.closest_base_color(), Color::WHITE);

        let c = Color::new(255, 0, 0);
        assert_eq!(c.closest_base_color(), Color::RED);

        let c = Color::new(0, 174, 0);
        assert_eq!(c.closest_base_color(), Color::GREEN);

        let c = Color::new(7, 13, 74);
        assert_eq!(c.closest_base_color(), Color::BLACK);

        let c = Color::new(176, 167, 11);
        assert_eq!(c.closest_base_color(), Color::YELLOW);
    }

    #[test]
    fn test_to_grayscale() {
        let c = Color::new(0, 0, 0);
        assert_eq!(c.to_grayscale(), Color::BLACK);

        let c = Color::new(255, 255, 255);
        assert_eq!(c.to_grayscale(), Color::WHITE);

        let c = Color::new(7, 13, 74);
        assert_eq!(c.to_grayscale(), Color::new(17, 17, 17));

        let c = Color::new(176, 167, 11);
        assert_eq!(c.to_grayscale(), Color::new(152, 152, 152));
    }

    #[test]
    fn test_darken() {
        let mut c = Color::WHITE;
        c.darken(0.2);
        assert_eq!(c, Color::new(204, 204, 204));

        let mut c = Color::new(176, 167, 11);
        c.darken(0.5);
        assert_eq!(c, Color::new(88, 83, 5));

        let mut c = Color::WHITE;
        c.darken(1.5);
        assert_eq!(c, Color::BLACK);
    }

    #[test]
    fn test_mix() {
        let c = Color::WHITE;
        let d = Color::BLACK;
        assert_eq!(c.mix(&d), Color::new(127, 127, 127));

        let c = Color::WHITE;
        let d = Color::WHITE;
        assert_eq!(c.mix(&d), Color::WHITE);

        let c = Color::BLACK;
        let d = Color::BLACK;
        assert_eq!(c.mix(&d), Color::BLACK);

        let c = Color::new(176, 167, 11);
        let d = Color::new(7, 13, 74);
        assert_eq!(c.mix(&d), Color::new(91, 90, 42));
    }
}
