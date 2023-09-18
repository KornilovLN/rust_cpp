//--- В cargo.toml внесена зависимость
extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/calc.cpp")
        .cpp(true)
        .compile("libcalc.a");
}
