use std::{collections::VecDeque, fmt::format};

use rayon::prelude::*; 
use rayon::iter::IntoParallelIterator;

use crate::parser::tree::ParseError;

use super::{tree::TreeNode, tree_nodes::TreeNodes};

pub struct Grammar<'a> {
    inner: Box<dyn GrammarLike + 'a>,
}

struct Match {
    nodes_before: TreeNodes,
    nodes_inner: TreeNodes
}
struct ProcessedMatch {
    nodes_before: TreeNodes,
    result: TreeNode
}
pub trait GrammarLike: Sync {
    fn next_match_start(&self, _nodes: &TreeNodes) -> Option<usize>;

    // range inclusive
    fn next_match_end(&self, nodes: &TreeNodes, start_index: usize) -> Option<usize>;
    fn next_match(&self, nodes: &TreeNodes) -> Option<(usize,usize)> {
        let Some(start_index) = self.next_match_start(&nodes) else {
            return None;
        };
        let Some(end_index) = self.next_match_end(&nodes,start_index) else {
            return None;
        };

        Some((start_index,end_index))
    }
    fn has_match(&self, nodes: &TreeNodes) -> bool {
        self.next_match(nodes).is_some()
    }
    fn construct(&self, nodes: TreeNodes) -> TreeNode;
    
    fn process_sequential_iteration(&self, mut nodes: TreeNodes) -> TreeNodes {
        let Some((
            start_index,
            end_index
        )) = self.next_match(&nodes) else {
            return nodes;
        };

        let nodes_inner = nodes.slice(start_index, end_index + 1);
        let nodes_before = nodes.slice(0, start_index);
        let rest = nodes;
        let result_node = self.construct(nodes_inner);
        let mut result = nodes_before;
        result = result.push_right(result_node);
        result = result.append(rest);

        result
    }
    fn process_parallel_iteration(&self, mut nodes: TreeNodes) -> TreeNodes {
        let mut matched: Vec<Match> = vec![];

        while let Some((
            start_index,
            end_index
        )) = self.next_match(&nodes) {
            let nodes_inner = nodes.slice(start_index, end_index + 1);
            let nodes_before = nodes.slice(0, start_index);

            matched.push(Match { nodes_before, nodes_inner });
        }

        let rest = nodes;

        let mut processed_matches: VecDeque<ProcessedMatch> = matched
            .into_par_iter()
            .map(|m| {
                ProcessedMatch {
                    nodes_before: m.nodes_before,
                    result: self.construct(m.nodes_inner)
                }
            })
            .collect();

        let maybe_left = processed_matches.pop_front();
        let Some(left_match) = maybe_left else {
            return rest;
        };
        let mut left_nodes = left_match.nodes_before;
        left_nodes = left_nodes.push_right(left_match.result);

        for m in processed_matches {
            left_nodes = left_nodes.append(m.nodes_before);
            left_nodes = left_nodes.push_right(m.result);
        }

        left_nodes = left_nodes.append(rest);

        left_nodes
    }
    fn process_all(&self, mut nodes: TreeNodes) -> TreeNodes {
        let range = nodes.range;
        let max_iteration_count = nodes.len() * nodes.len();

        let mut iteration_count = 0;

        let allow_parallel_processing = self.allow_parallel_processing();

        while self.has_match(&nodes) {
            if iteration_count > max_iteration_count {
                let err: TreeNode = ParseError::at(range, format!("Internal error: Max iteration count ({}) exceeded!",max_iteration_count)).into();
                nodes = nodes.push_right(err);
                break;
            }
            iteration_count += 1;
            if allow_parallel_processing {
                nodes = self.process_parallel_iteration(nodes);
            } else {
                nodes = self.process_sequential_iteration(nodes);
            }
        }

        nodes
    }
    fn allow_parallel_processing(&self) -> bool {
        false
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
            $crate::parser::grammar::process_grammars(grammar_nodes, $existing_nodes)
        }
    };
}
