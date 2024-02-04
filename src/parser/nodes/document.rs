use crate::{
    common::range::Range,
    parser::{
        program_tree::{try_map_into, PtError},
        tree::{ParseError, TreeNode, TreeNodeLike},
    },
};

use super::{
    function_declaration::{AstFunctionDeclaration, PtFunctionDeclaration},
    uniform_declaration::{AstUniformDeclaration, PtUniformDeclaration},
};

#[derive(Debug, Clone)]
pub struct AstDocument {
    pub range: Range,
    pub children: Vec<TreeNode>,
}

#[derive(Debug)]
pub struct PtDocument {
    pub range: Range,
    pub uniforms: Vec<PtUniformDeclaration>,
    pub functions: Vec<PtFunctionDeclaration>,
}

impl TryFrom<AstDocument> for PtDocument {
    type Error = PtError;
    fn try_from(value: AstDocument) -> Result<Self, Self::Error> {
        let range = value.range;
        let mut functions: Vec<AstFunctionDeclaration> = vec![];
        let mut uniforms: Vec<AstUniformDeclaration> = vec![];

        for child in value.children.into_iter() {
            match child {
                TreeNode::FunctionDeclaration(fun) => {
                    functions.push(fun);
                }
                TreeNode::UniformDeclaration(uni) => {
                    uniforms.push(uni);
                }
                TreeNode::ParseError(err) => {
                    return Err(err.into());
                }
                _ => {}
            }
        }

        let functions: Result<Vec<PtFunctionDeclaration>, PtError> = try_map_into(functions);
        let functions = functions?;

        let uniforms: Result<Vec<PtUniformDeclaration>, PtError> = try_map_into(uniforms);
        let uniforms = uniforms?;

        Ok(PtDocument {
            range,
            functions,
            uniforms,
        })
    }
}

impl TreeNodeLike for AstDocument {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&TreeNode> {
        return self.children.iter().collect();
    }
}
