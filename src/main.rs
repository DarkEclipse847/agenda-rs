extern crate colored;
extern crate term_table;
use colored::Colorize;
use chrono::{NaiveDate, Datelike, Days};
use std::process::Command;
use term_table::{row, row::Row,  table_cell::*};
use term_table::{Table, TableStyle, TableBuilder};

struct Color(i32, i32, i32);

struct CalendarDay {
    date: i32,
    note: String,
    color: Color,
}

fn build_day(
    date: i32,
    note: String,
) -> CalendarDay{
    let mut color = Color(255,255,255);
    CalendarDay{
        date: date,
        note: note,
        color: color,
    }  
}

fn main() {
    //println!("{}","Hello, world!".blue());
    let mut py_bind = Command::new("python");
    let py_com = py_bind.arg("ascii.py");
    py_com.current_dir("..");
    py_com.status().expect("failed to run script");

    let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
    let cur_date = chrono::Local::now().naive_local();
    let mut table = Table::new();
    table.max_column_width = 20;
    table.style = TableStyle::extended();
    table.add_row(Row::new(vec![TableCell::new_with_alignment(cur_date.month(), 7, Alignment::Center)]));
    table.add_row(Row::new(vec![TableCell::new("Mon"), TableCell::new("Tue"), TableCell::new("Wed"), TableCell::new("Thu"), TableCell::new("Fri"), TableCell::new("Sat"), TableCell::new("Sun")]));
    let first_cal_day = from_ymd(cur_date.year(), cur_date.month(), 1).checked_sub_days(Days::new(from_ymd(cur_date.year(), cur_date.month(), 1).weekday().num_days_from_monday() as u64)).unwrap();
    //let mut table = TableBuilder::new().rows(vec![Row::new(vec![TableCell::new_with_alignment("February", 7, Alignment::Center)])]).style(TableStyle::elegant()).build();
    for week in 0..5{
        let mut row = Row::new(vec![
            TableCell::new(first_cal_day.checked_add_days(Days::new(7*week)).unwrap().day()),
            TableCell::new(first_cal_day.checked_add_days(Days::new(1+7*week)).unwrap().day()),
            TableCell::new(first_cal_day.checked_add_days(Days::new(2+7*week)).unwrap().day()),
            TableCell::new(first_cal_day.checked_add_days(Days::new(3+7*week)).unwrap().day()),
            TableCell::new(first_cal_day.checked_add_days(Days::new(4+7*week)).unwrap().day()),
            TableCell::new(first_cal_day.checked_add_days(Days::new(5+7*week)).unwrap().day()),
            TableCell::new(first_cal_day.checked_add_days(Days::new(6+7*week)).unwrap().day()),
        ]);
        table.add_row(row);
    }
    println!("{}", table.render());
}
