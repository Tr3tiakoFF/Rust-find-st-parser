use std::str::FromStr;

use bnf::Error;
use bnf::Grammar;

use crate::get_terminals::get_all_terminals;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Command {
    record_name: String,
    key_name: String,
}

const GRAMMAR_STR: &str = "
<find-st2> ::= <find> <whitespace> <rec-name> <whitespace> <record> <whitespace> <by> <whitespace> <database> <whitespace> <key> <whitespace> <key-name> <EOL>;
<whitespace> ::= ' ';
<find> ::= 'FIND';
<record> ::= 'RECORD';
<by> ::= 'BY';
<database> ::= 'DATABASE';
<key> ::= 'KEY';
<rec-name> ::= <ident>;
<key-name> ::= <ident>;
<ident> ::= <ident-head> <ident-tail>;
<ident-head> ::= <ident-head-symbol>;
<ident-head-symbol> ::= <letter> | '_';
<ident-tail> ::= <ident-tail-symbol> | <ident-tail-symbol> <ident-tail>;
<ident-tail-symbol> ::= <letter> | <number> | '_';
<small-letter> ::= 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n' | 'o' | 'p' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z';
<big-letter> ::= 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N' | 'O' | 'P' | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z';
<letter> ::= <small-letter> | <big-letter>;
<number> ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9';
<EOL> ::= ';';
";

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut record_name = "".to_string();
        let mut key_name = "".to_string();
        
        let grammar: Grammar = GRAMMAR_STR.parse().unwrap();

        let parse_trees = grammar.parse_input(s);
    
        for parse_tree in parse_trees.into_iter() {
            let parse_tree_nodes: Vec<&bnf::ParseTreeNode<'_>> = parse_tree.rhs_iter().collect();
            for parse_tree_node in parse_tree_nodes {
                match parse_tree_node {
                    bnf::ParseTreeNode::Terminal(_) => {
                        return Err(Error::ParseError("UNKNOWN ERROR".to_string()));
                    },
                    bnf::ParseTreeNode::Nonterminal(term) => {
                        if term.lhs.to_string() == "<rec-name>" {
                            record_name = get_all_terminals(&term.rhs_iter().collect::<Vec<_>>());
                        }
                        if term.lhs.to_string() == "<key-name>" {
                            key_name = get_all_terminals(&term.rhs_iter().collect::<Vec<_>>());
                        }
                    },
                }
            }
        }

        if record_name == "" || key_name == "" {
            return Err(Error::ParseError("CAN NOT PARSE SUCH STRING".to_string()));
        }

        Ok(Self {
            record_name,
            key_name
        })
    }
}