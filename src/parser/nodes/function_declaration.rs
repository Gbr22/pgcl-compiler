use std::sync::{Arc, Mutex};

use crate::{
    common::range::Range,
    parser::{
        program_tree::{
            function_declaration::FunctionDeclarationReferableLike, program_tree::{CurrentContext, PtError, RootContext, RootContextMutRef, RootContextMutRefType, TryIntoPt}, scope::{FunctionScopeId, Referable, Scope, ScopeId}, value_declaration::ValueDeclarationReferableLike
        },
        tree::{TreeNode, TreeNodeLike},
    },
};

use super::{block::PtBlock, function_arg::PtFunctionArg, tagged_string::TaggedString, types::typ::PtType};

#[derive(Debug, Clone)]
pub struct AstFunctionDeclaration {
    pub range: Range,
    pub name: TaggedString,
    pub args: Box<TreeNode>,
    pub return_type: Box<TreeNode>,
    pub body: Box<TreeNode>,
}


impl TryIntoPt<PtFunctionDeclaration> for AstFunctionDeclaration {
    fn try_into_pt(
        self,
        mut root_context: RootContextMutRef,
        context: &CurrentContext,
    ) -> Result<PtFunctionDeclaration, PtError> {
        let range = self.range;
        let name = self.name;
        
        let scope_id = ScopeId::Function(FunctionScopeId::new(&context.uri, &name.value));
        let scope = Scope::new();
        let context = context.to_owned().extend(scope_id.clone());
        
        root_context.insert_scope(scope_id, scope)?;

        let args: Vec<PtFunctionArg> = match *self.args {
            TreeNode::FunctionArgs(args) => {
                args.try_into_pt(root_context.clone(),&context)?
            },
            TreeNode::ParseError(err) => {
                return Err(PtError::from(err))
            },
            _ => {
                return Err(PtError::in_at(&context.uri, self.args.get_range(), "Expected function args."));
            }
        };

        let return_type = match *self.return_type {
            TreeNode::AstType(typ) => {
                typ.try_into_pt(root_context.clone(), &context)?
            },
            node => {
                return Err(PtError::in_at(&context.uri, node.get_range(), "Expected type."));
            }
        };

        let body = match *self.body {
            TreeNode::Block(body) => {
                body.try_into_pt(root_context.clone(), &context)?
            },
            node => {
                return Err(PtError::in_at(&context.uri, node.get_range(), "Expected block."));
            }
        };

        Ok(PtFunctionDeclaration { range, name, args, return_type, body })
    }
}

#[derive(Debug, Clone)]
pub struct PtFunctionDeclaration {
    pub range: Range,
    pub name: TaggedString,
    pub args: Vec<PtFunctionArg>,
    pub return_type: PtType,
    pub body: PtBlock
}

impl Referable for PtFunctionDeclaration {
    fn get_name(&self) -> &str {
        &self.name.value
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
