use std::char;
use std::io::BufReader;
use std::io;
use std::mem;
use std::fs;
use std::io::Write;
use std::io::prelude::*;
use std::fs::File;
use std::mem::transmute;
use rand::prelude::*;
use std::num;


#[derive(Debug, Clone)]
pub struct Matrix {
    pub rows: usize,
    pub columns: usize,
    pub entries: Vec<Vec<f64>>
}


pub fn matrix_create(row: usize, col: usize)-> Matrix {
    let mut entrie = Vec::new();
    for i in 0..col {
        entrie.push(Vec::with_capacity(row));
    }

    Matrix {
        
        rows : row,
        columns : col,
        entries : entrie,

            
    }
}
pub fn matrix_fill(matrix: &mut Matrix, n: f64) {
    for x in 0..matrix.rows {
        dbg!(&x);
        for y in 0..matrix.columns {
            dbg!(&x,&y);
            matrix.entries[y].push(n);
        }// for now...don't call this twice. Should we just make a matrix 
        // initialized to 0s in rowxcol in create because why...make memory when 
        // we just fill it with a random integer after the fact like why?
    }
}


//fn matrix_free(matrix: Matrix); //Does Rust even need to free?
//fn matrix_print(matrix: Matrix); //Debug works for now.
pub fn matrix_copy(matrix: Matrix) -> Matrix {
    let new_matrix = matrix.clone();
    new_matrix
}
pub fn matrix_save(matrix: &mut Matrix, filename: &str) {
        //let byte0: [u8; 8] = unsafe { transmute(matrix.rows.to_be()) };
        //let byte1: [u8; 8] = unsafe { transmute(matrix.columns.to_be()) };
        let mut file = File::create(filename).unwrap();
        write!(file, "{}\n",&matrix.rows).unwrap();  //(file, matrix.rows); \n here to make the text file look nicer to me...idk how worth it is but still, ok now they 
        write!(file, "{}\n",&matrix.columns).unwrap(); //  both have a newline...because read_line gives me a usize and I think usize of string 55 is 3 and usize fo 6 is 9???
        for i in 0..matrix.rows {
            for j in 0..matrix.columns {
                write!(file,"{0:.6}\n", matrix.entries[i][j]).unwrap();
            
        }
    }
    println!("Successfully saved matrix to {}\n", filename);
}

pub fn matrix_load(filename: &str) -> Matrix {
    //let file = File::open(filename).unwrap();
	//let mut reader = BufReader::new(file);
    let buffer= fs::read_to_string(filename).unwrap();
    let mut lines = buffer.lines(); // read to string gives you...a lines() method to get the actual lines instead of the bytes I guess.
    /* 
    let mut input = String::new();
    &reader.read_to_string(&mut input).unwrap();
    // Remove mutability
    let input = format!("{:}", input);
    dbg!(&input); this kinda worked, below doesn't, gave 2,2, something
    
    let rows = format!("{:.8}",buffer(&mut buffer).unwrap()).parse::<usize>().unwrap(); the {:.12} format syntax is...neat...still cryptic.
    dbg!(&rows);
    let columns = format!("{:.8}",reader.read_line(&mut buffer).unwrap()).parse::<usize>().unwrap();// previously reader.read_line(&mut String::new()).unwrap();
    dbg!(&columns); //evidently, readline produces garbage if you don't do the formating.
	let mut matrix = matrix_create(rows, columns);
    for i in 0..matrix.rows {
        for j in 0..matrix.columns {
            matrix.entries[j].push(format!("{:}",reader.read_line(&mut buffer).unwrap()).parse::<f64>().unwrap());//reader.read_line(&mut String::new()).unwrap() as f64); gave 9.0
            dbg!(i,j); // I guess maybe investigate why it was giving you 3,6, 9.0,9.0, 9.0, 9.0, 9.0, 9.0, 9.0 what is 6.0 into bytes into f64?
        }
    }
    */
    //let fish = buffer.next().unwrap();
    //dbg!(fish); ... debug variable...look into atoli? is c really that hard?
    let rows = lines.next().unwrap().parse::<usize>().unwrap();
    let columns = lines.next().unwrap().parse::<usize>().unwrap();
    
    let mut matrix = matrix_create(rows, columns);
    for i in 0..matrix.rows {
        for j in 0..matrix.columns {
            matrix.entries[j].push(lines.next().unwrap().parse::<f64>().unwrap());//reader.read_line(&mut String::new()).unwrap() as f64); gave 9.0
            //dbg!(i,j); // I guess maybe investigate why it was giving you 3,6, 9.0,9.0, 9.0, 9.0, 9.0, 9.0, 9.0 what is 6.0 into bytes into f64?
        }
    }
	println!("Sucessfully loaded matrix from {}\n", filename);
	matrix
}


pub fn uniform_distribution(low: f64, high: f64) -> f64{
	let difference = high - low; // The difference between the two
	let scale = 10000.0;
	let scaled_difference = (difference * scale);
    //let random
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen(); 
	low + (1.0 * (y % scaled_difference) / scale)
}

pub fn matrix_randomize(mut matrix: Matrix, n:u64) {
    let min = -1.0 / (n as f64).sqrt();
    let max = 1.0 / (n as f64).sqrt();
    for i in 0..matrix.rows {
        for j in 0..matrix.columns {
            matrix.entries[i][j] = uniform_distribution(min, max);
        }
    }
}
pub fn matrix_argmax(matrix: Matrix) -> f64 {
    let mut max_score = 0.0;
    let mut max_idx = 0.0;
    for i in 0..matrix.rows {
        if matrix.entries[i][0] > max_score {
            max_score = matrix.entries[i][0];
            max_idx = i as f64;
        }
    }
    max_idx
}

pub fn matrix_flatten(matrix: Matrix, axis: usize) {
    let vector = &matrix.rows*&matrix.columns;
    let mut flat_matrix = match axis {
        0 => matrix_create(vector, 1),
        1 => matrix_create(1, vector),
        _ => panic!("Axis has to be 0 or 1")
        
    };
    for i in 0..matrix.rows {
        for j in 0..matrix.columns {
            if axis == 0 {flat_matrix.entries[i * matrix.columns + j][0] = matrix.entries[i][j]};
            if axis == 1 {flat_matrix.entries[0][i*matrix.columns+j] = matrix.entries[i][j]};
        }
    }
}

fn matrix_print(){ // the pretty colors tell you to fix this.
    todo!("Matrix_Print to visualize in terminal, 
update the imageProcess after you make it, if you ever make it.");
}