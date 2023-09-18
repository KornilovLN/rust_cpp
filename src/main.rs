//-----------------------------------------------------------------------------
//--- Отработка методики совместного использования функций
//--- написанных в Cpp из программы на rust
//-----------------------------------------------------------------------------
//--- Author: Kornilov LN (Starmark)
//--- github: "https://github.com/KornilovLN/Life.git"
//--- e-mail: ln.KornilovStar@gmail.com
//--- e-mail: ln.starmark@ekatra.io
//--- e-mail: ln.starmark@gmail.com
//--- date:   18.09.2023 14:44:00
//-----------------------------------------------------------------------------

/*
use std::env;
use std::fmt;
extern crate ansi_term;
use ansi_term::Colour;
*/

use rust_cpp::*;
mod md_about;

fn main() {

    //--------------------------------------------------------------------------
    let about = md_about::StAbout::new (
		"Leonid",
		"Nikolaevich",
		"Kornilov",	
		"Kornilov LN (Starmark)",
		"https://github.com/KornilovLN/Life.git",
		"ln.KornilovStar@gmail.com",
		"11.08.2023 02:40:00",
	);
	about.out();
	about.waiter(1);

    //--------------------------------------------------------------------------

    calculator();

    //--------------------------------------------------------------------------
}



