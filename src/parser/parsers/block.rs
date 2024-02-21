use crate::parser::brackets::{is_closing_bracket_curly, is_opening_bracket_curly};

use crate::parser::parsers::block_content::BlockContentParser;
use crate::parser::{parse::Parser, tree::TreeNode, tree_nodes::TreeNodes};
use crate::{pop_back_node, pop_front_node};

pub struct BlockParser {}

impl Parser for BlockParser {
    fn parse(mut nodes: TreeNodes) -> TreeNode {
        let _range = nodes.range;

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

        BlockContentParser::parse(nodes)
    }
}
