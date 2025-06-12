pub fn fits_in_type(value: i64, type_name: &str) -> bool {
    match type_name {
        "i8" => value >= i8::MIN as i64 && value <= i8::MAX as i64,
        "u8" => value >= u8::MIN as i64 && value <= u8::MAX as i64,
        "i16" => value >= i16::MIN as i64 && value <= i16::MAX as i64,
        "u16" => value >= u16::MIN as i64 && value <= u16::MAX as i64,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fits_in_type() {
        assert!(fits_in_type(100, "i8"));
        assert!(!fits_in_type(300, "u8"));
        assert!(fits_in_type(-100, "i16"));
        assert!(!fits_in_type(-1, "u16"));
    }
}