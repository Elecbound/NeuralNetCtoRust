use std::fs;
use crate::matrixNotMath::{Matrix, matrix_create};
struct Img {

}
pub fn csv_to_imgs(filename: &str /*, number_of_imgs: f64 */) -> Vec<Matrix> {
    let file = fs::read_to_string(filename).unwrap();
    dbg!(&file);
    let info = file.lines().next();
    dbg!(info);
    let csv_images = file.lines().next();
    let mut images: Vec<Matrix> = vec![];
    //for tomorrow, file.lines only runs once so.....its gotta be in da loop?
  

    for i in csv_images {
        
        dbg!(&i);
        let mut img_matrix = matrix_create(28,28);
        for (i, element) in i.chars().enumerate() {
            let j = i/27;
            dbg!(&i, & element, &j);
            img_matrix.entries[i][j] = element.to_digit(10).unwrap() as f64;
            
        }
     images.push(img_matrix);
    }
    images
}
/* 

pub fn img_print(img: Img) {
    todo!("I really need to make a print matrix so we can see it from the 
    terminal");
    //dbg!(&img.img_data);
    //dbg!("Img Label: {}\n", img.img_data);
}
*/