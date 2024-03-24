extern crate colored;
extern crate term_table;
use colored::Colorize;
use rusqlite::{Connection, Result}; 
use chrono::{NaiveDate, Datelike, Days};
use std::process::Command;
use std::collections::VecDeque;
use term_table::{row, row::Row, table_cell::*};
use term_table::{Table, TableStyle, TableBuilder};

#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct CalendarDay {
    date: u64,
    
    note: String,
    color: Color,
}

impl CalendarDay{
    fn add_day (
        date: u64,
        note: String,
    ) -> CalendarDay {
        let mut color = Color(255,255,255);
        CalendarDay{
            date: date, 
            note: note,
            color: color,
        }  
    }
}

fn main() -> Result<()>{
    //Establishing connection with python to print ascii pictures 
    let mut py_bind = Command::new("python"); 
    let py_com = py_bind.arg("ascii.py"); 
    py_com.current_dir(".."); 
    py_com.status().expect("failed to run script");

    //Database implementation
    let conn = Connection::open("calendar.db")?;
    conn.execute(
        "create table if not exists calendar(
            date integer primary key,
            note text
        )",
        (),
    )?;
    //conn.execute("drop table calendar",(),)?;
    
    let mut stmt = conn.prepare("select date, note from calendar")?;
    let calendar_iter = stmt.query_map([], |row|{
        Ok(CalendarDay{
            date: row.get(0)?,
            note: row.get(1)?,
            color: Color(255, 255, 255),
        })
    })?;
    
    //Getting current month day and year using chrono
    //to write them into sqlite db later
    let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap(); let cur_date = 
    chrono::Local::now().naive_local();
    let mut table = Table::new(); table.max_column_width = 20; 
    table.style = TableStyle::extended(); 
    table.add_row(Row::new(vec![TableCell::new_with_alignment(cur_date.month(), 7, Alignment::Center)])); 
    table.add_row(Row::new(vec![TableCell::new("Mon"), TableCell::new("Tue"), TableCell::new("Wed"), TableCell::new("Thu"), TableCell::new("Fri"), TableCell::new("Sat"), TableCell::new("Sun")]));
    //let first_cal_day = from_ymd(cur_date.year(), cur_date.month(), 1).checked_sub_days(Days::new(from_ymd(cur_date.year(), cur_date.month(), 1).weekday().num_days_from_monday() as u64)).unwrap();
    let first_cal_day = from_ymd(cur_date.year(), cur_date.month(), 1);

    let mut rows_vec: VecDeque<CalendarDay> = VecDeque::with_capacity(35);
    for i in calendar_iter{
        //println!("{:?}", i.unwrap());
        rows_vec.push_back(i.unwrap());
    }
    for week in 0..5{
        for day_iter in 1..8{
            let day = CalendarDay::add_day(first_cal_day.checked_add_days(Days::new(day_iter+7*week)).unwrap().day() as u64, "".to_string());
            conn.execute(
                "insert or ignore into calendar(date, note) values(?1, ?2)",
                (&day.date, &day.note),
            )?;         
        }
        
        if week == 0 {
            for _ in 0..first_cal_day.weekday().num_days_from_monday(){
                rows_vec.push_front(CalendarDay::add_day(0 as u64, "".to_string()));
            }
        }
        let row = Row::new(
            vec![
                TableCell::new(rows_vec[7*week as usize].date),
                TableCell::new(rows_vec[1+7*week as usize].date),
                TableCell::new(rows_vec[2+7*week as usize].date),
                TableCell::new(rows_vec[3+7*week as usize].date),
                TableCell::new(rows_vec[4+7*week as usize].date),
                TableCell::new(rows_vec[5+7*week as usize].date),
                TableCell::new(rows_vec[6+7*week as usize].date),
            ]
        ); table.add_row(row);
    }
    println!("{}", table.render());
    //for j in 0..35{
    //    println!("Day {:?}", rows_vec[j]);
    //}
    Ok(())
}
