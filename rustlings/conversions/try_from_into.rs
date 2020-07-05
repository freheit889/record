use std::convert::{TryInto, TryFrom};

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl TryFrom<(i16, i16, i16)> for Color {
    type Error = String;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        let (x,y,z)=tuple;
        if(x<255 && y<255 && z<255){
            return Ok(Color{red:x as u8,green:y as u8,blue:z as u8});
        }else{
            return Err("color is wrong".to_string());
        }
    }
}

// Array implementation
impl TryFrom<[i16; 3]> for Color {
    type Error = String;
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        let x=arr[0];
        let y=arr[1];
        let z=arr[2];
        if(x<255 && y<255 && z<255){
            return Ok(Color{red:x as u8,green:y as u8,blue:z as u8});
        }else{
            return Err("color is wrong".to_string());
        }
    }
}

 //Slice implementation
impl TryFrom<&[i16]> for Color {
    type Error = String;
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        let x=slice[0];
        let y=slice[1];
        let z=slice[2];
        if(x<255 && y<255 && z<255){
            return Ok(Color{red:x as u8,green:y as u8,blue:z as u8});
        }else{
            return Err("color is wrong".to_string());
        }
    }
}

fn main() {
    // Use the `from` function
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    let v = vec![183, 65, 14];
    
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_tuple_out_of_range_positive() {
        let _ = Color::try_from((256, 1000, 10000)).unwrap();
    }
    #[test]
    #[should_panic]
    fn test_tuple_out_of_range_negative() {
        let _ = Color::try_from((-1, -10, -256)).unwrap();
    }
    #[test]
    fn test_tuple_correct() {
        let c: Color = (183, 65, 14).try_into().unwrap();
        assert_eq!(c.red, 183);
        assert_eq!(c.green, 65);
        assert_eq!(c.blue, 14);
    }

    #[test]
    #[should_panic]
    fn test_array_out_of_range_positive() {
        let _: Color = [1000, 10000, 256].try_into().unwrap();
    }
    #[test]
    #[should_panic]
    fn test_array_out_of_range_negative() {
        let _: Color = [-10, -256, -1].try_into().unwrap();
    }
    #[test]
    fn test_array_correct() {
        let c: Color = [183, 65, 14].try_into().unwrap();
        assert_eq!(c.red, 183);
        assert_eq!(c.green, 65);
        assert_eq!(c.blue, 14);
    }

    #[test]
    #[should_panic]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        let _ = Color::try_from(&arr[..]).unwrap();
    }
    #[test]
    #[should_panic]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        let _ = Color::try_from(&arr[..]).unwrap();
    }
    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c = Color::try_from(&v[..]).unwrap();
        assert_eq!(c.red, 183);
        assert_eq!(c.green, 65);
        assert_eq!(c.blue, 14);
    }
    #[test]
    #[should_panic]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        let _ = Color::try_from(&v[..]).unwrap();
    }
}
