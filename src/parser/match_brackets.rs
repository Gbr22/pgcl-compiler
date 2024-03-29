use super::{brackets::BracketType, tree::TreeNode};

impl From<usize> for SignedIndex {
    fn from(value: usize) -> Self {
        Self(value as isize)
    }
}

impl From<i32> for SignedIndex {
    fn from(value: i32) -> Self {
        Self(value as isize)
    }
}

impl From<isize> for SignedIndex {
    fn from(value: isize) -> Self {
        Self(value)
    }
}

pub struct SignedIndex(isize);

pub fn find_bracket_end<'a>(
    bracket_type: BracketType,
    opening_index: impl Into<SignedIndex>,
    nodes: impl Iterator<Item = &'a TreeNode>,
) -> Option<usize> {
    let opening_index: SignedIndex = opening_index.into();
    let opening_index: isize = opening_index.0;

    let mut opening_count = 1;
    let mut closing_count = 0;
    for (index, node) in nodes.enumerate() {
        if (index as isize) <= opening_index {
            continue;
        }
        if (bracket_type.is_opening)(node) {
            opening_count += 1;
        } else if (bracket_type.is_closing)(node) {
            closing_count += 1;
        }

        if opening_count == closing_count {
            return Some(index);
        }
    }

    None
}

pub struct BracketTracker {
    pub depth: isize,
    pub typ: BracketType,
}
impl From<BracketType> for BracketTracker {
    fn from(val: BracketType) -> Self {
        BracketTracker { depth: 0, typ: val }
    }
}
pub fn track_bracket_depth(tracker: &mut BracketTracker, node: &TreeNode) {
    if (tracker.typ.is_opening)(node) {
        tracker.depth += 1;
    }
    if (tracker.typ.is_closing)(node) {
        tracker.depth -= 1;
    }
}

impl BracketTracker {
    pub fn track_depth(&mut self, node: &TreeNode) {
        track_bracket_depth(self, node)
    }
}

pub fn find_next_match_outside_brackets<'a, IsMatch: Fn(&TreeNode) -> bool>(
    bracket_types: Vec<BracketType>,
    is_match: IsMatch,
    start_index: usize,
    nodes: impl Iterator<Item = &'a TreeNode>,
) -> Option<usize> {
    let mut trackers: Vec<BracketTracker> =
        bracket_types.into_iter().map(|typ| typ.into()).collect();

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

        return Some(index);
    }

    None
}
