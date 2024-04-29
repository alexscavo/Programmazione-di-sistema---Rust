use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    // input string
    slug_in: String,
}


trait MySlug {
    fn is_slug(&self) -> bool;
    fn to_slug(&self) -> String;
}

//Voglio essere sicuro che T (che può essere qualsiasi tipo) abbia un'implementazione all'interno del tratto
//che gli permetta di essere trasformato in un qualcosa. Mettere AsRef<str> dice che io voglio che T
//implementi as_ref per str. Il compilatore va a vedere che to_slug vuole un &ref quindi capisce tra i vari
//as_ref che io voglio ottenere un &str, quindi andrà da solo a recuperarsi l'implementazione di
//as_ref che converte ad esempio String -> &str
impl<T> MySlug for T where T:AsRef<str> {   //T è obbligato ad avere un'implementazione per as_ref che gli permette di essere trasformato in un "&str"

    fn is_slug(&self) -> bool {

        let conv_str: String = self.as_ref().to_slug();

        conv_str == self.as_ref()
    }

    fn to_slug(&self) -> String{
        slugify(self.as_ref())
    }
}



// FUNZIONE CHE CONVERTE L'INTERA STRINGA
fn slugify(s: &str) -> String{

    let mut ret_val = String::new();
    let mut new_car: char;


    for car in s.chars() {

        new_car = conv(car);    //converto il carattere

        if !ret_val.ends_with('-') || new_car != '-' {
            ret_val.push(new_car);
        }

    }

    if ret_val.ends_with('-') && ret_val.len() != 1 {
        ret_val.pop();
    }

    ret_val
}

// FUNZIONE CHE CONVERTE UN SINGOLO CARATTERE
fn conv(c: char) -> char {

    const SUBS_I : &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
    const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";

    let mut count_index = 0;
    let mut count_index_conv = 0;

    //verifico se c è un carattere ascii e lo converto in lowercase
    if c.is_ascii_alphanumeric() {
        return c.to_ascii_lowercase();
    }

    //cerco la posizione nel vettore dei caratteri accentati
    for car in SUBS_I.chars() {

        if c == car {   //la lettera deve essere convertita
            break;
        }

        count_index = count_index + 1;
    }

    //cerco la posizione corrispondente nel vettore dei caratteri non accentiati
    for conv_car in SUBS_O.chars() {
        if count_index == count_index_conv {
            return conv_car;
        }

        count_index_conv = count_index_conv + 1;
    }

    '-'
}




fn main() {


    let args = Args::parse();  //faccio il parsing della command line

    let final_string = slugify(args.slug_in.as_str());

    println!("{}", final_string);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_is_slug() {

        let s1 = String::from("Hello String");
        let s2 = "hello-slice";

        assert_eq!(false, s1.is_slug());
        assert_eq!(true, s2.is_slug());
    }

    #[test]
    fn check_to_slug() {

        let s1 = String::from("Hello String");
        let s2 = "hello-slice";
        let s3: String = s1.to_slug();
        let s4: String = s2.to_slug();

        assert_eq!(s3, "hello-string");
        assert_eq!(s4, "hello-slice");
    }



}
