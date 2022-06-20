use std::fs;
use crate::matrixNotMath::{Matrix, matrix_create};
struct Img {

}
pub fn csv_to_imgs(filename: &str /*, number_of_imgs: f64 */) /*-> Vec<Matrix>*/
-> Vec<Matrix>
{
    /* 
    let file_path = filename;
    let file = File::open(file_path).unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    let result = rdr.records().next();
        let record = result.unwrap();
        
        println!("{:?}", record.unwrap().len());
    */

    let file = fs::read_to_string(filename).unwrap();
    
    let mut info = file.lines();
    //dbg!(&info.clone().count());
    let first_line = info.next();
    dbg!(&first_line);
    dbg!(first_line.unwrap().split(",").count());
    //let huh = info.next().unwrap().split(",").filter_map(|s| s.parse::<f64>().ok()).collect::<Vec<_>>();
    //dbg!(&huh);
    //dbg!(&huh.len());
    let mut csv_images: Vec<Matrix> = vec!();

    for i in info {
        let mut image_matrix = matrix_create(28, 28);
        let mut breakdown: Vec<&str> = i.split(",").collect();
        let mut again =  breakdown.iter();
        //dbg!(&breakdown);
        //i.chars();
        for i in 0..image_matrix.rows {
            for j in 0..image_matrix.columns {
                image_matrix.entries[i].push(again.next().unwrap().parse::<f64>().unwrap());
               


                /* 
                info.next().unwrap()
                .split(",")
                .filter_map(|s| s.parse::<f64>().ok());
                .collect::<Vec<_>>();
*/

                //dbg!(breakdown.next());
                //image_matrix.entries[i][j] = *brstr.parse::<f64>().iter().next().unwrap()
                
            }
        }
        //dbg!(&image_matrix);
        csv_images.push(image_matrix);
    }
    csv_images
}
/* 

pub fn img_print(img: Img) {
    todo!("I really need to make a print matrix so we can see it from the 
    terminal");
    //dbg!(&img.img_data);
    //dbg!("Img Label: {}\n", img.img_data);
}
*/
