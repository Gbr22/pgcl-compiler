use crate::parser::brackets::{is_closing_bracket_curly, is_opening_bracket_curly};
use crate::parser::grammars::block::BlockGrammar;
use crate::parser::grammars::statements::ret::ReturnStatementGrammar;
use crate::parser::grammars::statements::simple::SimpleStatementGrammar;
use crate::parser::grammars::var_declaration::VarDeclarationGrammar;
use crate::parser::parsers::block_content::BlockContentParser;
use crate::parser::{nodes::block::Block, parse::Parser, tree::TreeNode, tree_nodes::TreeNodes};
use crate::{pop_back_node, pop_front_node, process_grammars};

pub struct BlockParser {}

impl Parser for BlockParser {
    fn parse(mut nodes: TreeNodes) -> TreeNode {
        let range = nodes.range;

        pop_front_node!(
            nodes,
            "Missing opening `{`.",
            Some(node),
            is_opening_bracket_curly(&node)
        );

        pop_back_node!(
            nodes,
            "Missing closing `}`.",
            Some(node),
            is_closing_bracket_curly(&node)
        );

        let block = BlockContentParser::parse(nodes);

        block
    }
}
