use std::fmt::format;
use std::sync::{Arc, Mutex};
use std::thread;
use itertools::{Itertools, repeat_n};


fn check_equation(operators: &Vec<Vec<char>>, values: &Vec<i32>) -> Vec<String> {

    let mut results_vec: Vec<String> = Vec::new();

    for i in 0..operators.len() {

        let mut flag = 0;
        let mut result = values[0];
        let mut result_string = String::new();
        result_string.push_str(format!("{val}", val=values[0]).as_str());


        for j in 0..4 {
            match operators[i][j] {
                '+' => { result = result + values[j + 1]; result_string.push_str(format!("+{val}", val=values[j+1]).as_str()) },
                '-' => { result = result - values[j + 1]; result_string.push_str(format!("-{val}", val=values[j+1]).as_str()) },
                '*' => { result = result * values[j + 1]; result_string.push_str(format!("*{val}", val=values[j+1]).as_str()) },
                '/' => {
                    if values[j+1] == 0 {
                        flag = 1;
                        break;
                    }
                    if (result/values[j+1])%2 != 0 {
                        flag = 1;
                        break;
                    }
                    result += result / values[j + 1];
                    result_string.push_str(format!("/{val}", val=values[j+1]).as_str());
                },
                _ => ()
            }
        }

        if result == 10 && flag == 0{
            results_vec.push(result_string);
        }

    }


    results_vec

}


fn main() {

    let mut input = vec![2, 7, 2, 2, 1];
    let mut input_operators = vec!['+', '-', '*', '/'];
    let mut results: Vec<String> = Vec::new();
    let results_shared = Arc::new(Mutex::new(results));   // rendo disponibili a più thread il vettore risultato

    let mut input_permutations: Vec<Vec<i32>> = Vec::new();
    input_permutations = input.into_iter().permutations(5).collect();

    //println!("{:?}", input_permutations);

    let operators: Vec<_> = repeat_n(input_operators, 4).multi_cartesian_product().collect();
    //println!("{:?}", operators);

    if input_permutations.clone().len() % 2 == 0 {  // numero di permutazioni pari

        let num_threads = 8;
        let block_dim = input_permutations.clone().len()/num_threads;
        let block_total_size = input_permutations.clone().len();
        let shared_operators = Arc::new(operators); // rendo disponibili a più thread gli oepratori
        let shared_values = Arc::new(input_permutations); // rendo disponibili a più thread i valori permutati
        let mut handlers = vec![];


        for i in 0..num_threads { // ciclo per creare i thread

            let private_operators = Arc::clone(&shared_operators);  // clono gli operatori per condividerli
            let private_values = Arc::clone(&shared_values);     // clono i valori permutati per condividerli
            let private_results = Arc::clone(&results_shared);

            let handler = thread::spawn(move || {


                for j in i*block_dim..block_total_size-((num_threads-1-i)*block_dim)
                {
                    // chiamo la funzione per verificare l'equazione
                    let result = check_equation(&(*private_operators) ,&((*private_values)[j]));

                    let mut results_input = private_results.lock().unwrap(); // lock

                    for string in result {  // modifico results
                        results_input.push(string);
                    }
                }

            });

            handlers.push(handler); // mi salvo l'handler del thread

        }

        for handler in handlers { // aspetto la terminazione di tutti i thread
            handler.join().unwrap();
        }

        let result_print = results_shared.lock().unwrap();
        for i in 0..result_print.len() {
            println!("{}", result_print[i])
        }






    }

}
