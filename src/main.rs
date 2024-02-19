#[allow(unused_imports)]
use bnf::Grammar;
#[allow(unused_imports)]
use parser_lab::command::Command;
#[allow(unused_imports)]
use parser_lab::get_terminals::get_all_terminals;

fn main() {
    let command: Command = "FIND COOL_RECORD RECORD BY DATABASE KEY COOL_KEY;".parse().unwrap();
    println!("{:?}", command);
}
