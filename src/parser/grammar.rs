use super::{tree::TreeNode, tree_nodes::TreeNodes};

pub struct Grammar<'a> {
    inner: Box<dyn GrammarLike + 'a>,
}

pub trait GrammarLike {
    fn next_match_start(&self, _nodes: &TreeNodes) -> Option<usize>;

    // range inclusive
    fn next_match_end(&self, nodes: &TreeNodes, start_index: usize) -> Option<usize>;
    fn has_match(&self, nodes: &TreeNodes) -> bool {
        self.next_match_start(nodes).is_some()
    }
    fn construct(&self, nodes: TreeNodes) -> TreeNode;
    fn process_next(&self, mut nodes: TreeNodes) -> (Option<TreeNode>, TreeNodes) {
        let start_index = self.next_match_start(&nodes);
        let Some(start_index) = start_index else {
            return (None, nodes);
        };
        let end_index = self.next_match_end(&nodes, start_index);
        let Some(end_index) = end_index else {
            return (None, nodes);
        };

        let inner_nodes = nodes.slice(start_index, end_index + 1);

        let new_node = self.construct(inner_nodes);

        nodes.insert(start_index, new_node.clone());

        (Some(new_node), nodes)
    }
    fn process_all(&self, nodes: TreeNodes) -> TreeNodes {
        let mut processed_nodes = nodes;

        while self.has_match(&processed_nodes) {
            let (new_node, new_nodes) = self.process_next(processed_nodes);
            processed_nodes = new_nodes;
            if let None = new_node {
                break;
            }
        }

        processed_nodes
    }
}

impl<'a, T> From<T> for Grammar<'a>
where
    T: GrammarLike + 'a,
{
    fn from(value: T) -> Self {
        let b = Box::new(value);
        Grammar { inner: b }
    }
}

pub fn process_grammars(grammars: Vec<Grammar>, nodes: TreeNodes) -> TreeNodes {
    let mut nodes = nodes;

    for grammar in &grammars {
        nodes = grammar.inner.process_all(nodes);
    }

    nodes
}

#[macro_export]
macro_rules! process_grammars {
    ($existing_nodes:ident [ $( $grammar:ident ),* ] ) => {
        {
            let grammar_nodes = vec![
                $( ($grammar {}).into() ),*
            ];
            crate::parser::grammar::process_grammars(grammar_nodes, $existing_nodes)
        }
    };
}
