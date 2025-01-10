// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

pub struct WrappingU32 {
    value: u32,
}

impl From<u32> for WrappingU32 {
    fn from(wrappee: u32) -> WrappingU32 {
        WrappingU32 {
            value: wrappee
        }
    }
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into() {
        let wrapping: WrappingU32 = 42.into();
        assert_eq!(42, wrapping.value);
    }

    #[test]
    fn test_from() {
        let wrapping = WrappingU32::from(42);
        assert_eq!(42, wrapping.value);
    }
}
