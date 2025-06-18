pub fn fits_in_type(value: i64, type_name: &str) -> bool {
     match type_name {
        "i8" => {
            // Your code here
        },
        "u8" => {
            // Your code here
        },
        "i16" => {
            // Your code here
        },
        "u16" => {
            // Your code here
        },
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