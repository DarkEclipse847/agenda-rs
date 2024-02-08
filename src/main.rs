extern crate colored;
extern crate term_table;
use colored::Colorize;
use chrono::{NaiveDate, Datelike};
use std::process::Command;
use term_table::{row, row::Row,  table_cell::*};
use term_table::{Table, TableStyle, TableBuilder};

fn main() {
    println!("{}","Hello, world!".blue());
    let mut py_bind = Command::new("python");
    let py_com = py_bind.arg("ascii.py");
    py_com.current_dir("..");
    py_com.status().expect("failed to run script");

    let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
    let cur_date = chrono::Local::now().naive_local();
    println!("{:?}", cur_date.month());
    let mut table = Table::new();
    table.max_column_width = 20;
    table.style = TableStyle::extended();
    table.add_row(Row::new(vec![TableCell::new_with_alignment("February", 7, Alignment::Center)]));
    table.add_row(Row::new(vec![TableCell::new("Mon"), TableCell::new("Tue"), TableCell::new("Wed"), TableCell::new("Thu"), TableCell::new("Fri"), TableCell::new("Sat"), TableCell::new("Sun")]));
    //let mut table = TableBuilder::new().rows(vec![Row::new(vec![TableCell::new_with_alignment("February", 7, Alignment::Center)])]).style(TableStyle::elegant()).build();
    for _ in 0..5{
        let mut row = Row::new(vec![TableCell::new("1"), TableCell::new("1"), TableCell::new("1"), ]);
        table.add_row(row)
    }
    println!("{}", table.render())
}
