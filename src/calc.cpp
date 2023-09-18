//--- Эта функция отработает при вызове из main.rs
//--- В файле main.rs должна быть объявлена функция с подобной сигнатурой,
//--- т.е. fn mult_input(input: libc::c_int) -> libc::c_int;
 
extern "C"
int calc_input(int input, int mult) {
    return input * mult;
}
