//--- Эта функция отработает при вызове из main.rs
//--- В файле main.rs должна быть объявлена функция с подобной сигнатурой,
//--- т.е. fn triple_input(input: libc::c_int) -> libc::c_int;
 
extern "C"
int triple_input(int input) {
    return input * 3;
}
