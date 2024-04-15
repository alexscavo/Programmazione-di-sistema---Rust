use std::fs;

const bsize: usize = 20;
pub struct Board {
    boats: [u8; 4],
    data: [[u8; bsize]; bsize],
}
pub enum Error {
    Overlap,
    OutOfBounds,
    BoatCount,
}

pub enum Boat {
    Vertical(usize),
    Horizontal(usize)
}

impl Board {
    /* crea una board vuota con una disponibilità di navi */
    pub fn new(boats: &[u8]) -> Board {

        //creo la board vuota
        let d = [[0u8; 20]; 20];
        let b = boats.try_into().expect("Slice with incorrect length :(");

        Board {boats: b, data: d}
    }

    /* crea una board a partire da una stringa che rappresenta tutto
    il contenuto del file board.txt */
    pub fn from(s: String) -> Board {

        let mut boats = [0u8; 4];
        //let mut board: Board;
        let mut i = 0;
        let mut data = [[0u8; 20]; 20];

        let splitted_file = s.split("\r\n");    //splitto la board per righe

        for line in splitted_file {

            if i == 0 {   //se e' la prima linea, inserisco in boat il numero di ciascuna nave
                let splitted_line = line.split(' ');  //splitto la stringa per leggerne i campi
                let mut j = 0;

                for val in splitted_line {

                     let converted_val: i32 = match val.parse::<i32>() {    //inserisco in boats il valore letto
                        Ok(val) => val,
                        Err(err) => -1,
                    };

                    if(converted_val != -1){
                        boats[j] = converted_val as u8;
                    }
                    else{
                        println!("Errore di parsing!");
                    }

                    j = j + 1;
                }

            }
            else {  //riempo la matrice con i valori del file
                let mut j = 0;
                for cell in line.chars() {

                    match cell {
                        '.' => data[i-1][j] = 0,    //cella vuota
                        'B' => data[i-1][j] = 1,    //cella piena
                        _ => {}
                    }
                }
            }
            i = i + 1;
        }

        let mut board = Board::new(&[1,2,3,4]);
        board.data = data;

        board
    }

    /*
    /* aggiunge la nave alla board, restituendo la nuova board se
    possibile */
    /* bonus: provare a *non copiare* data quando si crea e restituisce
    una nuova board con la barca, come si può fare? */
    pub fn add_boat(self, boat: Boat, pos: (usize, usize)) -> Result<Board, Error> {

    }

    /* converte la board in una stringa salvabile su file */
    pub fn to_string(&self) -> String {

    }*/
}



fn main() {

    let path = "src/board.txt";


    let prova = Board::from(fs::read_to_string(path).unwrap());

    println!("fine");
}
