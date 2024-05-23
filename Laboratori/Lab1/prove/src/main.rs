use std::time::SystemTime;

enum Error{
    Simple {time: SystemTime},
    Complex {time: SystemTime, err: String}
}

enum MulErr {
    Overflow,
    NegativeNumber
}

struct Node {
    name: String,
    size: u32,
    count: u32,
}

impl Node {
    pub fn new(n: String) -> Node {
        Node {name: n, size: 0, count: 0}
    }

    pub fn size(&mut self, size: u32) {
        self.size = size;
    }

    pub fn count_p(&mut self, count: u32) {
        self.count = count;
    }

    pub fn to_string(&self) -> String {

        format!("name: {}, size: {}, count: {}", self.name, self.size, self.count)
    }

}

fn print_error(e: Error){
    
    match e {
        Error::Simple { time } => println!("time = {:?}", time),
        Error::Complex { time , err } => println!("time = {:?}, error = {}", time, err)
    }
}

fn mul(a: i32, b: i32) -> Result<u32, MulErr> {

    if a < 0 || b < 0 {
        return Err(MulErr::NegativeNumber)
    }
    else if a.overflowing_mul(b).1 == true  {
        return Err(MulErr::Overflow)
    }

    let ris = a as u32 * b as u32;

    Ok(ris)
}

fn main() {

    let error = Error::Complex {time: SystemTime::now(), err: "Suca".to_string()};

    print_error(error);

    let risultato = mul(-29, 10);

    match risultato {
        Ok(value) => {println!("{}", value)},
        Err(error) => {
            match error {
                MulErr::Overflow {} => println!("Overflow!"),
                MulErr::NegativeNumber {} => println!("Numero Negativo :(")
            }
        }
    }


    let mut node: Node =  Node::new("node".to_string());

    node.size(5);
    node.count_p(4);


    println!("{}", node.to_string());
}

