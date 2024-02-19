use std::env::var;

use crate::{
    common::range::Range,
    parser::{program_tree::{program_tree::{CurrentContext, PtError, RootContextMutRef, TryIntoPt}, scope::{BlockScopedId, Scope, ScopeId, VarScopeId}}, tree::{TreeNode, TreeNodeLike}},
};

use super::statements::statement::PtStatement;

#[derive(Debug, Clone)]
pub struct Block {
    pub range: Range,
    pub children: Vec<TreeNode>,
}

impl TreeNodeLike for Block {
    fn get_range(&self) -> Range {
        self.range
    }
    fn children(&self) -> Vec<&TreeNode> {
        return self.children.iter().collect();
    }
}

impl TryIntoPt<PtBlock> for Block {
    fn try_into_pt(
        self,
        mut root_context: RootContextMutRef,
        context: &CurrentContext,
    ) -> Result<PtBlock, PtError> {
        let range = self.range;

        let scope_id = ScopeId::Block(BlockScopedId::new(&context.uri, range));
        let scope = Scope::new();
        let mut context = context.to_owned().extend(scope_id.clone());
        root_context.insert_scope(scope_id, scope)?;

        let mut statements: Vec<PtStatement> = vec![];
        for child in self.children {
            match child {
                TreeNode::ParseError(e) => {
                    return Err(e.into())
                },
                TreeNode::VarDeclaration(var_declaration) => {
                    let pt = var_declaration.try_into_pt(root_context.clone(), &context)?;
                    let new_scope = ScopeId::Var(VarScopeId::new(&context.uri, pt.name.clone(), pt.range));
                    context = context.extend(new_scope);
                    statements.push(PtStatement::VarDeclaration(pt));
                },
                TreeNode::Statement(_) => {
                    // TODO
                },
                _ => {
                    return Err(PtError::in_at(&context.uri, range, "Unexpected item."))
                }
            }
        }

        Ok(PtBlock {
            range,
            statements
        })
    }
}

#[derive(Debug, Clone)]
pub struct PtBlock {
    pub range: Range,
    pub statements: Vec<PtStatement>
}