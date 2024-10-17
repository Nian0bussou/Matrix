#![allow(dead_code)]
#![allow(unused_variables)]
use std::fmt::{self, Debug};

#[derive(Debug, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>, pub usize, pub usize);

impl<T> Matrix<T> {
    pub fn rows(&self) -> usize {
        self.1
    }
    pub fn cols(&self) -> usize {
        self.2
    }
}

pub type Alpha = f64;

impl<T> fmt::Display for Matrix<T>
where
    T: fmt::Display + std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.0 {
            // Join elements in each row with a space between them
            let row_str = row
                .iter()
                .map(|elem| {
                    let left = format!("{:>15.5?}", elem);
                    format!("{:<4}", left)
                }) // Convert each element to a string
                .collect::<Vec<String>>() // Collect into a Vec<String>
                .join(" "); // Join them with spaces
            writeln!(f, "[ {} ]", row_str)?; // Write each row to the formatter
        }
        writeln!(f, "")?; // Write each row to the formatter
        Ok(())
    }
}

impl<T> Matrix<T>
where
    T: std::fmt::Debug,
{
    pub fn print(&self) {
        for row in &self.0 {
            let row_str = row
                .iter()
                .map(|elem| {
                    let left = format!("{:>15.5?}", elem);
                    format!("{:<4}", left)
                }) // Convert each element to a string
                .collect::<Vec<String>>() // Collect into a Vec<String>
                .join(" "); // Join them with spaces
            println!("[ {} ]", row_str); // Write each row to the formatter
        }
        println!("");
    }
}
