mod es0301;

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use crate::es0301::find_sub;

    #[test]
    fn check_find_sub() {
        let string: &str = "A1-2";
        let dna: &str = "ATT";

        let result = match find_sub(dna, string) {
            Some(t) => t,
            None => (0, "")
        };

        assert_eq!(result, (0usize, "A"));

    }
}
