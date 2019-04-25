extern crate lalrpop;
extern crate peg;

fn main() {
    lalrpop::process_root().unwrap();

    peg::cargo_build("src/computor_v1/parser.rustpeg");
}
