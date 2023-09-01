#[cfg(test)]
mod tests {
    use std::num::*;

    #[test]
    fn non_zero_i32() {
        let num = NonZeroI32::new(-128).unwrap();
        assert_eq!(-128, num.get());
        assert_eq!(128, num.abs().get());
    }

    #[test]
    fn non_zero_i32_parse_zero() {
        let num = NonZeroI32::new(0);
        assert!(num.is_none());
    }

    #[test]
    fn non_zero_u32() {
        let num = NonZeroU32::new(256).unwrap();
        assert_eq!(256, num.get());
        assert_eq!(2, num.ilog10());
        assert!(num.is_power_of_two())
    }

    #[test]
    fn non_zero_u32_parse_zero() {
        let num = NonZeroU32::new(0);
        assert!(num.is_none());
    }
}
