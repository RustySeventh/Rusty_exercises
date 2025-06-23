use all_asserts::assert_false;
pub fn fits_in_type(value: i64, type_name: &str) -> bool {
    match type_name {
        "i8" => 
        "u8" => 
        "i16" =>
        "u16" => 
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
        assert_false!(fits_in_type(800, "i8"));
    }
}

