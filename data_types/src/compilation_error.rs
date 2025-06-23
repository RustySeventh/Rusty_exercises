 fn compilation_error {
    let guess = "42".parse().expect("Not a number!");
    let number = 3.14;
    let is_ready = true;
    let letter = 'R';
    
    println!("Guess: {}, Number: {}, Ready: {}, Letter: {}", 
             guess, number, is_ready, letter);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compilation_error() {
        compilation_error();
    }
}