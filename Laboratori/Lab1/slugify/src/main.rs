use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    // input string
    slug_in: String,
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
    fn accented_letter() {

        let t1 = slugify("ç");
        let t2 = slugify("à");
        let t3 = slugify("ż");

        assert_eq!(t1, "c");
        assert_eq!(t2, "a");
        assert_eq!(t3, "z");
    }

    #[test]
    fn lowercase_letter() {

        let t1 = slugify("a");
        let t2 = slugify("k");

        assert_eq!(t1, "a");
        assert_eq!(t2, "k");
    }

    #[test]
    fn unkown_letter() {

        let t1 = slugify("ק");
        let t2 = slugify("漢");

        assert_eq!(t1, "-");
        assert_eq!(t2, "-");
    }

    #[test]
    fn unkown_accented_letter() {

        let t1 = slugify("ῶ");

        assert_eq!(t1, "-");
    }

    #[test]
    fn spaced_words() {

        let t1 = slugify("Questa mattina fa caldo");
        let t2 = slugify("Delta = Δ");

        assert_eq!(t1, "questa-mattina-fa-caldo");
        assert_eq!(t2, "delta");
    }

    #[test]
    fn accented_words() {

        let t1 = slugify("Questa è una prova");
        let t2 = slugify("Perchè è più bello");

        assert_eq!(t1, "questa-e-una-prova");
        assert_eq!(t2, "perche-e-piu-bello");
    }

    #[test]
    fn empty_string() {

        let t1 = slugify("");

        assert_eq!(t1, "");
    }

    #[test]
    fn multiple_spaces() {

        let t1 = slugify("Ops     troppi    spazi");

        assert_eq!(t1, "ops-troppi-spazi");
    }

    #[test]
    fn multiple_unkown_characters() {

        let t1 = slugify("Βίβλος");

        assert_eq!(t1, "-");
    }

    #[test]
    fn final_space() {
        let t1 = slugify("provaaa ");

        assert_eq!(t1, "provaaa");
    }

    #[test]
    fn final_unkown_characters() {

        let t1 = slugify("Testo greco: Βίβλος");

        assert_eq!(t1, "testo-greco");
    }
}
