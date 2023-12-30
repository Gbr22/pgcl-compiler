
use crate::common::range::Range;
use core::slice::Iter;

use super::tree::{TreeNode, get_range};

#[derive(Clone)]
pub struct TreeNodes {
    pub range: Range,
    vec: Vec<TreeNode>
}

fn calculate_new_ranges(original_range: Range,left_last: Option<&TreeNode>, right_first: Option<&TreeNode>) -> (Range, Range) {
    let mut left_range = Range::null();
    let mut right_range = Range::null();
    
    left_range.start_index = original_range.start_index;
    left_range.end_index = left_range.start_index;

    right_range.end_index = original_range.end_index;
    right_range.start_index = right_range.end_index;

    if let Some(left_last) = left_last {
        left_range.end_index = left_last.get_range().end_index;
    }
    if let Some(right_first) = right_first {
        right_range.start_index = right_first.get_range().start_index;
    }
    
    return (left_range, right_range);
}

impl TreeNodes {
    pub fn new(range: Range, nodes: Vec<TreeNode>) -> TreeNodes {
        TreeNodes {
            range,
            vec: nodes.into()
        }
    }
    pub fn iter(&self) -> Iter<'_, TreeNode> {
        let iter = self.vec.iter();

        iter
    }
    pub fn slice_left(&mut self, mut count: usize) -> TreeNodes {
        if count > self.vec.len() {
            count = self.vec.len();
        }
        let removed_nodes: Vec<TreeNode> = self.vec
            .splice(0..count, vec![])
            .collect();

        let (removed_range, self_range) = calculate_new_ranges(
            self.range,
            removed_nodes.last(),
            self.vec.first()
        );

        self.range = self_range;

        TreeNodes {
            range: removed_range,
            vec: removed_nodes
        }
    }
    pub fn slice_right(&mut self, mut count: usize) -> TreeNodes {
        if count > self.vec.len() {
            count = 0;
        }
        let remove_start_index = self.vec.len() - count;
        let removed_nodes: Vec<TreeNode> = self.vec
            .splice(
                remove_start_index..self.vec.len(),
                 vec![]
            )
            .collect();
        
        let (self_range, removed_range) = calculate_new_ranges(
            self.range,
            self.vec.last(),
            removed_nodes.first()
        );

        self.range = self_range;

        TreeNodes {
            range: removed_range,
            vec: removed_nodes
        }
    }
    pub fn append(mut self, mut right: TreeNodes) -> TreeNodes {
        let range = Range::new(
            self.range.start_index,
            right.range.end_index
        );
        self.range = range;
        self.vec.append(&mut right.vec);
        self
    }
    pub fn slice(&mut self, start_inclusive: usize, end_exclusive: usize) -> TreeNodes {
        if self.vec.len() == 0 {
            // There are no nodes.
            // It doesn't matter where we take the slice from.
            // The clone will keep the range data and have an empty vector.
            return self.clone()
        }
        if start_inclusive >= self.vec.len() {
            // The start index is out of bounds
            // Return an empty range with the position of the current end_index
            return TreeNodes {
                range: Range::new(
                    self.range.end_index,
                    self.range.end_index,    
                ),
                vec: vec![]
            }
        }

        // Clamp indicies to the bounds of the vector 
        let start_inclusive = start_inclusive.clamp(
            0,
             self.vec.len()-1 // start_inclusive is already asseted to be less than `self.nodes.len()` otherwise the function would've returned already
        );
        let end_exclusive = end_exclusive.clamp(
            0, 
            self.vec.len()
        );
        
        // Clamps indicies so that start_inclusive <= end_exclusive
        let start_inclusive = start_inclusive.clamp(start_inclusive, end_exclusive);
        let end_exclusive = end_exclusive.clamp(start_inclusive, end_exclusive);

        let range_length = end_exclusive - start_inclusive;
        /* let start_distance_from_left = start_inclusive;
        let end_distance_from_left = end_exclusive;
        let start_distance_from_right = 
        let end_distance_from_right = self.nodes.len() - end_exclusive; */
        // There are a few assuptions we make from this point
        // start_inclusive <= end_exclusive

        match (start_inclusive == 0, end_exclusive == self.vec.len()){
            (true, true)=>self.slice_left(range_length),
            (true, false)=>self.slice_left(range_length),
            (false, true)=>self.slice_right(range_length),
            (false, false)=>{
                /*
                The vector looks like this:

          start_inclusive-|         |-end_exclusive
              0-|         |         |         |-self.vec.len()
                [        ][        ][        ]|
                   left     middle     right
                
                */
                
                let right = self.slice_right(self.vec.len() - end_exclusive);
                let left = self.slice_left(0 + start_inclusive);
                let middle = self.clone();
                let union = left.append(right);

                let result = middle;
                self.range = union.range;
                self.vec = union.vec;

                result
            }
        }
    }
    pub fn insert(&mut self, index: usize, node: TreeNode) {
        self.vec.insert(index, node.clone());
        self.range = self.range + node.get_range();
    }
    pub fn len(&self) -> usize {
        self.vec.len()
    }
    pub fn from_vec(nodes: Vec<TreeNode>) -> Option<TreeNodes> {
        get_range(&nodes).map(|range|{
            TreeNodes::new(range, nodes)
        })
    }
    pub fn null() -> TreeNodes {
        TreeNodes { range: Range::null(), vec: vec![] }
    }
    pub fn first(&self) -> Option<&TreeNode> {
        self.vec.first()
    }
    pub fn into_first(self) -> Option<TreeNode> {
        self.vec.into_iter().next()
    }
    pub fn last(&self) -> Option<&TreeNode> {
        self.vec.last()
    }
    pub fn into_last(self) -> Option<TreeNode> {
        self.vec.into_iter().last()
    }
    pub fn into_vec(self) -> Vec<TreeNode> {
        self.vec
    }
    pub fn pop_front(&mut self) -> Option<TreeNode> {
        let slice = self.slice_left(1);
        slice.into_first()
    }
    pub fn pop_front_internal(&mut self) -> (Option<TreeNode>, Range) {
        let old_range = self.range;
        let slice = self.slice_left(1);
        match slice.into_first() {
            Some(node)=>{
                let node_range = node.get_range();
                (Some(node),node_range)
            },
            None=>{
                (None, old_range)
            }
        }
    }
    pub fn pop_back_internal(&mut self) -> (Option<TreeNode>, Range) {
        let old_range = self.range;
        let slice = self.slice_right(1);
        match slice.into_last() {
            Some(node)=>{
                let node_range = node.get_range();
                (Some(node),node_range)
            },
            None=>{
                (None, old_range)
            }
        }
    }
    pub fn pop_back(&mut self) -> Option<TreeNode> {
        let slice = self.slice_right(1);
        slice.into_last()
    }
}
