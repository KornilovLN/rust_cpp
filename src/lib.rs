//-----------------------------------------------------------------------------
//--- Библиотека для главного модуля main.rs 
//--- переменные и методы должны быть pub
//-----------------------------------------------------------------------------
//--- Author: Kornilov LN (Starmark)
//--- github: "https://github.com/KornilovLN/Life.git"
//--- e-mail: ln.KornilovStar@gmail.com
//--- e-mail: ln.starmark@ekatra.io
//--- e-mail: ln.starmark@gmail.com
//--- date:   18.09.2023 14:44:00
//-----------------------------------------------------------------------------

/*
use std::error::Error;
use std::fs;

extern crate time; 
use time::precise_time_ns;
*/

//--- В cargo.toml внесена зависимость -----------------------------------
extern crate libc;
//--- Дана сигнатура функции, что будет вызвана из скомпилированного файла 
//--- calc.cpp,  т.е.  int calc_input(int input, int mult)
//--- типы указать следует, как в библиотеке libc
extern {
    fn calc_input(input: libc::c_int, mult: libc::c_int) -> libc::c_int;
}
//------------------------------------------------------------------------

use std::io::Write;
use std::io;

use std::fmt;

extern crate ansi_term;
use ansi_term::Colour;

//-----------------------------------------------------------------------------

pub fn read_string(comment:&str) -> String {
    print!("{}", comment);
    io::stdout().flush();

    let mut string: String = String::new();

    io::stdin().read_line(&mut string)
        .ok()
        .expect("Error read line!");

    return string;
}

//-----------------------------------------------------------------------------

//--- Собственно - пример применения функции на Cpp
pub fn calculator() {

    let prompt0 = format!("{}",Colour::Green.paint("Вычислить произведение 2-х чисел:"));
    println!("{}",prompt0);

    let prompt1 = format!("{}",Colour::Yellow.paint("Введите 1-е число -> "));
    let prompt2 = format!("{}",Colour::Yellow.paint("Введите 2-е число -> "));

	let str1 = read_string(&prompt1);
    let str2 = read_string(&prompt2);

    let promptErr = format!("{}",Colour::Red.paint("Не правильный ввод числа"));
    let input = str1.trim().parse().expect(&promptErr);
    let mult =  str2.trim().parse().expect(&promptErr);

    //--- Вызов Cpp функции следует делать в unsafe режиме ---
    let output = unsafe { calc_input(input, mult) };
    //--------------------------------------------------------

    println!("{}\n",format!("{} -> {} * {} = {}", Colour::Green.paint("Результат: ".to_string()),
                                                Colour::Yellow.paint(input.to_string()),
                                                Colour::Yellow.paint(mult.to_string()),
                                                Colour::Yellow.paint(output.to_string())) );
}

