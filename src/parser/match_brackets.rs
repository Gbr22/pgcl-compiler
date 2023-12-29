use super::tree::TreeNode;

pub fn find_bracket_end<
    IsOpening: Fn(&TreeNode)->bool,
    IsClosing: Fn(&TreeNode)->bool,
>(
    is_opening_bracket: IsOpening,
    is_closing_bracket: IsClosing,
    opening_index: usize,
    nodes: &[TreeNode]
) -> Option<usize> {
    let mut opening_count = 1;
    for (index, node) in nodes.iter().enumerate() {
        if index <= opening_index {
            continue;
        }
        if is_opening_bracket(node) {
            opening_count = opening_count + 1;
        } else if is_closing_bracket(node) {
            opening_count = opening_count - 1;
        }

        if opening_count == 0 {
            return Some(index);
        }
    }

    None
}