use bnf::ParseTree;
use bnf::ParseTreeNode;

pub fn get_lhs_terminal(parse_tree: &ParseTree) -> String {
    let mut parse_tree = parse_tree;
    while let bnf::ParseTreeNode::Nonterminal(non_terminal) = parse_tree.rhs_iter().next().unwrap() {
        parse_tree = non_terminal;
    }

    let node = parse_tree.rhs_iter().next().unwrap();
    match node {
        ParseTreeNode::Terminal(str) => str.to_string(),
        ParseTreeNode::Nonterminal(_) => todo!(),
    }
}

pub fn get_all_terminals(parse_tree_nodes: &[&ParseTreeNode]) -> String {
    let mut word: String = "".to_string();
    for parse_tree_node in parse_tree_nodes {
        match parse_tree_node {
            ParseTreeNode::Terminal(_) => todo!(),
            ParseTreeNode::Nonterminal(term) => {
                if term.lhs.to_string() == "<ident>" {
                    return get_all_terminals(&term.rhs_iter().collect::<Vec<_>>())
                }
                if term.lhs.to_string() == "<ident-head>" {
                    word.push_str(&get_lhs_terminal(term));
                }
                if term.lhs.to_string() == "<ident-tail>" {
                    word.push_str(&get_all_terminals(&term.rhs_iter().collect::<Vec<_>>()));
                }
                if term.lhs.to_string() == "<ident-tail-symbol>" {
                    word.push_str(&get_lhs_terminal(term));
                }
            },
        }
    }
    return word;
}