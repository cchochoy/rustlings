// TryFrom is a simple and safe type conversion that may fail in a controlled way under some circumstances.
// Basically, this is the same as From. The main difference is that this should return a Result type
// instead of the target type itself.
// You can read more about it at https://doc.rust-lang.org/std/convert/trait.TryFrom.html
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Your task is to complete this implementation
// and return an Ok result of inner type Color.
// You need create implementation for a tuple of three integer,
// an array of three integer and slice of integer.
//
// Note, that implementation for tuple and array will be checked at compile-time,
// but slice implementation need check slice length!
// Also note, that chunk of correct rgb color must be integer in range 0..=255.

// Tuple implementation
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = String;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        let bounds = 0..256;
        match tuple {
            (r, g, b) if bounds.contains(&r) && bounds.contains(&g) && bounds.contains(&b) => {
                let red = u8::try_from(r).unwrap();
                let green = u8::try_from(g).unwrap();
                let blue = u8::try_from(b).unwrap();

                Ok(Color { red, green, blue })
            },
            _ => Err(String::from("Couldn't convert tuples to Color"))
        }
    }
}

// Array implementation
impl TryFrom<[i16; 3]> for Color {
    type Error = String;
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        let bounds = 0..256;
        match arr {
            [r, g, b] if bounds.contains(&r) && bounds.contains(&g) && bounds.contains(&b) => {
                let red = u8::try_from(r).unwrap();
                let green = u8::try_from(g).unwrap();
                let blue = u8::try_from(b).unwrap();

                Ok(Color { red, green, blue })
            },
            _ => Err(String::from("Couldn't convert array to Color"))
        }
    }
}

// Slice implementation
impl TryFrom<&[i16]> for Color {
    type Error = String;
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err(String::from("Need 3 element in slice to create a Color"))
        }
        let bounds = 0..256;
        match slice {
            [r, g, b] if bounds.contains(r) && bounds.contains(g) && bounds.contains(b) => {
                let red = u8::try_from(*r).unwrap();
                let green = u8::try_from(*g).unwrap();
                let blue = u8::try_from(*b).unwrap();

                Ok(Color { red, green, blue })
            },
            _ => Err(String::from("Couldn't convert slice to Color"))
        }
    }
}

fn main() {
    // Use the `from` function
    let c1 = Color::try_from((183, 65, 14)).unwrap();
    println!("{:?}", c1);

    // Since From is implemented for Color, we should be able to use Into
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    let v = vec![183, 65, 14];
    // With slice we should use `from` function
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);
    // or take slice within round brackets and use Into
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_out_of_range_positive() {
        assert!(Color::try_from((256, 1000, 10000)).is_err());
    }
    #[test]
    fn test_tuple_out_of_range_negative() {
        assert!(Color::try_from((-1, -10, -256)).is_err());
    }
    #[test]
    fn test_tuple_sum() {
        assert!(Color::try_from((-1, 255, 255)).is_err());
    }
    #[test]
    fn test_tuple_correct() {
        let c: Result<Color, String> = (183, 65, 14).try_into();
        assert_eq!(
            c,
            Ok(Color {
                red: 183,
                green: 65,
                blue: 14
            })
        );
    }
    #[test]
    fn test_array_out_of_range_positive() {
        let c: Result<Color, String> = [1000, 10000, 256].try_into();
        assert!(c.is_err());
    }
    #[test]
    fn test_array_out_of_range_negative() {
        let c: Result<Color, String> = [-10, -256, -1].try_into();
        assert!(c.is_err());
    }
    #[test]
    fn test_array_sum() {
        let c: Result<Color, String> = [-1, 255, 255].try_into();
        assert!(c.is_err());
    }
    #[test]
    #[test]
    fn test_array_correct() {
        let c: Result<Color, String> = [183, 65, 14].try_into();
        assert_eq!(
            c,
            Ok(Color {
                red: 183,
                green: 65,
                blue: 14
            })
        );
    }
    #[test]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        assert!(Color::try_from(&arr[..]).is_err());
    }
    #[test]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        assert!(Color::try_from(&arr[..]).is_err());
    }
    #[test]
    fn test_slice_sum() {
        let arr = [-1, 255, 255];
        assert!(Color::try_from(&arr[..]).is_err());
    }
    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, String> = Color::try_from(&v[..]);
        assert_eq!(
            c,
            Ok(Color {
                red: 183,
                green: 65,
                blue: 14
            })
        );
    }
    #[test]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert!(Color::try_from(&v[..]).is_err());
    }
    #[test]
    fn test_slice_insufficient_length() {
        let v = vec![0, 0];
        assert!(Color::try_from(&v[..]).is_err());
    }
}
