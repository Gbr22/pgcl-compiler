use super::{tree::TreeNode, brackets::BracketType};

pub fn find_bracket_end<'a>(
    bracket_type: BracketType,
    opening_index: usize,
    nodes: impl Iterator<Item = &'a TreeNode>
) -> Option<usize> {
    let mut opening_count = 1;
    let mut closing_count = 0;
    for (index, node) in nodes.enumerate() {
        if index <= opening_index {
            continue;
        }
        if (bracket_type.is_opening)(node) {
            opening_count = opening_count + 1;
        } else if (bracket_type.is_closing)(node) {
            closing_count = closing_count + 1;
        }

        if opening_count == closing_count {
            return Some(index);
        }
    }

    None
}

pub struct BracketTracker {
    pub depth: isize,
    pub typ: BracketType
}
impl Into<BracketTracker> for BracketType {
    fn into(self) -> BracketTracker {
        BracketTracker { depth: 0, typ: self }
    }
}
pub fn track_bracket_depth(tracker: &mut BracketTracker, node: &TreeNode) {
    if (tracker.typ.is_opening)(node) {
        tracker.depth = tracker.depth + 1;
    }
    if (tracker.typ.is_closing)(node) {
        tracker.depth = tracker.depth - 1;
    }
}

impl BracketTracker {
    pub fn track_depth(&mut self, node: &TreeNode) {
        track_bracket_depth(self, node)
    }
}

pub fn find_next_match_outside_brackets<
    'a,
    IsMatch: Fn(&TreeNode)->bool
>(bracket_types: Vec<BracketType>, is_match: IsMatch, start_index: usize, nodes: impl Iterator<Item = &'a TreeNode>) -> Option<usize> {
    let mut trackers: Vec<BracketTracker> = bracket_types
        .into_iter()
        .map(|typ|typ.into())
        .collect();

    'outer: for (index, node) in nodes.enumerate() {
        if index < start_index {
            continue;
        }

        for tracker in &mut trackers {
            tracker.track_depth(node);
        }

        if !is_match(node) {
            continue;
        }

        for tracker in &trackers {
            if tracker.depth != 0 {
                continue 'outer;
            }
        }

        return Some(index)
    }

    None
}