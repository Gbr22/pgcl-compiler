use std::{string::ParseError, sync::{Arc, Mutex}};

use crate::{
    common::range::Range,
    parser::{
        program_tree::{
            function_declaration::FunctionDeclarationReferableLike, program_tree::{CurrentContext, PtError, RootContext, TryIntoPt}, scope::Referable, value_declaration::ValueDeclarationReferableLike
        },
        tree::{TreeNode, TreeNodeLike},
    },
};

use super::{function_arg::PtFunctionArg, types::typ::PtType};

#[derive(Debug, Clone)]
pub struct AstFunctionDeclaration {
    pub range: Range,
    pub name: String,
    pub args: Box<TreeNode>,
    pub return_type: Box<TreeNode>,
    pub body: Box<TreeNode>,
}

impl TryIntoPt<PtFunctionDeclaration> for AstFunctionDeclaration {
    fn try_into_pt(
        self,
        root_context: Arc<Mutex<RootContext>>,
        context: &CurrentContext,
    ) -> Result<PtFunctionDeclaration, PtError> {
        let range = self.range;
        let name = self.name;

        let args: Vec<PtFunctionArg> = match *self.args {
            TreeNode::FunctionArgs(args) => {
                args.try_into_pt(root_context.clone(),context)?
            },
            TreeNode::ParseError(err) => {
                return Err(PtError::from(err))
            },
            _ => {
                return Err(PtError {
                    range: Some(self.args.get_range()),
                    message: "Expected function args.".to_owned()
                });
            }
        };

        let return_type = match *self.return_type {
            TreeNode::AstType(typ) => {
                typ.try_into_pt(root_context.clone(), context)?
            },
            node => {
                return Err(PtError {
                    range: Some(node.get_range()),
                    message: format!("Expected type.")
                })
            }
        };

        Ok(PtFunctionDeclaration { range, name, args, return_type })
    }
}

#[derive(Debug, Clone)]
pub struct PtFunctionDeclaration {
    pub range: Range,
    pub name: String,
    pub args: Vec<PtFunctionArg>,
    pub return_type: PtType
}

impl Referable for PtFunctionDeclaration {
    fn get_name(&self) -> &str {
        &self.name
    }
}
impl FunctionDeclarationReferableLike for PtFunctionDeclaration {}

impl TreeNodeLike for AstFunctionDeclaration {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&TreeNode> {
        vec![&self.args, &self.return_type, &self.body]
    }
}
