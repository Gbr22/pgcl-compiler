use super::tree::TreeNode;
use super::grammars::document::DocumentGrammar;
use super::grammars::uniform_declaration::UniformDeclarationGrammar;
use super::grammars::function_declaration::FunctionDeclarationGrammar;

trait_enum!{
    pub enum Grammar: GrammarLike {
        DocumentGrammar,
        UniformDeclarationGrammar,
        FunctionDeclarationGrammar,
    }
}

pub trait GrammarLike {
    fn next_match_at(&self, _nodes: &[TreeNode]) -> Option<usize>;
    
    // range inclusive
    fn find_match_end(&self, nodes: &[TreeNode], start_index: usize) -> Option<usize>;
    fn has_match(&self, nodes: &[TreeNode]) -> bool {
        self.next_match_at(nodes).is_some()
    }
    fn construct(&self, nodes: Vec<TreeNode>) -> TreeNode;
    fn process_next(&self, mut nodes: Vec<TreeNode>) -> (Option<TreeNode>, Vec<TreeNode>) {
        let start_index = self.next_match_at(&nodes);
        let Some(start_index) = start_index else {
            return (None, nodes);
        };
        let end_index = self.find_match_end(&nodes, start_index);
        let Some(end_index) = end_index else {
            return (None, nodes);
        };

        let inner_nodes = nodes
            .splice(start_index..(end_index+1),vec![])
            .collect();

        let new_node = self.construct(inner_nodes);

        nodes.insert(start_index,new_node.clone());

        (Some(new_node), nodes)
    }
    fn process_all(&self, nodes: Vec<TreeNode>) -> Vec<TreeNode> {
        let mut processed_nodes = nodes;
        
        while self.has_match(&processed_nodes) {
            let (new_node, new_nodes) = self.process_next(processed_nodes);
            processed_nodes = new_nodes;
            if let None = new_node {
                break;
            }
        };

        processed_nodes
    }
}