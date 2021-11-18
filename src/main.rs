#![allow(non_snake_case)]

use std::{io::{self,Write}};


fn bisection_method(f: fn(f64) -> f64, mut a: f64, mut b: f64, Ex: f64, Ef: f64) -> f64 {
    let horizontal_line = "-".repeat(79);
    println!("{}",horizontal_line);
    println!("|  n  |     a     |     b     |     m     |    f(a)   |    f(b)   |    f(m)   |");
    println!("{}",horizontal_line);

    // Closures used for printing of table
    let calc_padding_left = |x| if x < 0.0 {2} else {3};
    let print_nabm_values = |n,a,b,m| print!("|  {}{}|{}{:.4}  |{}{:.4}  |{}{:.4}  ",
        n+1, " ".repeat(if n+1 >= 10 {1} else {2}), " ".repeat(calc_padding_left(a)),a," ".repeat(calc_padding_left(b)),b," ".repeat(calc_padding_left(m)),m);
    let print_f_values = |fa,fb,fm|  println!("|{}{:.4}  |{}{:.4}  |{}{:.4}  |",
        " ".repeat(calc_padding_left(fa)),fa," ".repeat(calc_padding_left(fb)),fb," ".repeat(calc_padding_left(fm)),fm);
    
    let mut n = 0;
    let mut m = (a+b)/2.0;
    print_nabm_values(n,a,b,m);
    loop {
        if (b-a)/2.0 <= Ex {
            print_f_values(f(a),f(b),f(m));
            break;
        }
        
        let fm = f(m);
        if fm.abs() <= Ef {
            print_f_values(f(a),f(b),fm);
            break;
        }
        
        let (fa, fb) = (f(a), f(b));
        print_f_values(fa,fb,fm);

        if fm * fa < 0.0 {
            b = m;
        } else if fm * fb < 0.0 {
            a = m;
        }
        
        n += 1;
        m = (a+b)/2.0;
        print_nabm_values(n,a,b,m);
    }
    println!("{}",horizontal_line);

    m
}


#[derive(Debug)]
struct BisectionResultInfo {
    m_values:  Vec<f64>,            
    a_values:  Vec<f64>,   
    b_values:  Vec<f64>,  
    fa_values: Vec<f64>,
    fb_values: Vec<f64>,
    fm_values: Vec<f64>,
}

#[derive(Debug)]
struct Inputs {
    a:  f64,
    b:  f64,
    Ex: f64,
    Ef: f64
}

fn get_inputs() -> Inputs {
    fn read_input_from_stdin() -> String {
        print!("\nGive me the:  a b Ex Ef (example: 0 3.1416 0.0031 0): ");
        io::stdout().flush().expect("Could not flash stdout");
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).expect("Could not read from stdin.");
        buf
    }

    let args = {
        let args = std::env::args().skip(1).collect::<Vec<String>>();
        if args.is_empty() {
            read_input_from_stdin().split_whitespace().map(|x|x.to_owned()).collect::<Vec<String>>()
        } else {
            args
        }
    };

    if args.len() != 4 {
        panic!("\n\n==> Must provide 4 arguments.\n\n");
    }

    let mut inputs_iter = args
        .iter()
        .map(|x| 
            x.parse::<f64>()
            .unwrap_or_else(|_| panic!("\n\n==> Invalid argument format. Total arguments must be 4 (a,b,Ex,Ef) and all should be convertible to numbers\n\n")
        )
    );

    Inputs {
        a: inputs_iter.next().unwrap(),
        b: inputs_iter.next().unwrap(),
        Ex: inputs_iter.next().unwrap(),
        Ef: inputs_iter.next().unwrap(),
    }
}


fn run_book_examples() {
    // Example 1 
    println!("\nExample 1 (0  3.1416  0.0031  0)");
    let root_estimate = bisection_method(|x| x - x.cos(), 0.0, 3.1416, 0.0031,  0.0);
    println!("Root estimate: {:.4}\n", root_estimate);
    
    // Example 2
    println!("\nExample 2 (-5  5  0.0195  0)");
    let root_estimate = bisection_method(|x| x/8.0 - (x-3.0).sin(), -5.0, 5.0, 0.0195, 0.0);
    println!("Root estimate: {:.4}\n", root_estimate);
    
    // Example 3 (let's put 0.0175 instead of 0.0176 as Ex, to avoid floating point inaccuracies here)
    println!("\nExample 3 (-5  4  0.0175  0)");
    let root_estimate = bisection_method(|x| x/8.0 - (x-3.0).sin(), -5.0, 4.0, 0.0175, 0.0); 
    println!("Root estimate: {:.4}\n", root_estimate);
}

fn run_for_exercise_27() {
    println!("\nExercise 27 (1  3  0.0001  0)");
    let root_estimate = bisection_method(|x| 2.0 * x.cos()-x, 1.0, 3.0, 0.00005, 0.0);
    println!("Root estimate: {:.4}\n", root_estimate);
}

// Don't forget to specify the function you want (first parameter)
fn run_custom() {
    let inputs = get_inputs();
    let root_estimate = bisection_method(|x| 2.0 * x.cos()-x, inputs.a, inputs.b, inputs.Ex, inputs.Ef);
    println!("Root estimate: {:.4}\n", root_estimate);
}


fn main() {
    run_book_examples();
    run_for_exercise_27();
    println!("\n************************************************************************\nCustom\n");
    run_custom();
}
