use crate::es0301::{demo1, demo2, demo_SimpleDNAIter, demo_dna_iter};

mod es0301;

fn main() {
    /*
    println!("---------------DEMO 1---------------/");
    demo1();

    println!("---------------DEMO 2---------------/");
    demo2();


    println!("----------------ITER 1--------------/");
    demo_SimpleDNAIter();
     */

    println!("----------------ITER 2--------------");
    demo_dna_iter();
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
