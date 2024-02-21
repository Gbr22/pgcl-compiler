use crate::{
    common::range::Range,
    parser::{program_tree::{program_tree::{CurrentContext, PtError, RootContextMutRef, TryIntoPt}, scope::{BlockScopedId, Scope, ScopeId, VarScopeId}, value_declaration::ValueDeclarationReferable}, tree::{TreeNode, TreeNodeLike}},
};

use super::{expressions::expr::PtExpression, statements::{simple::PtExpressionStatement, statement::PtStatement}, var_declaration::PtVarDeclaration};

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

#[derive(Debug, Clone)]
pub enum PtBlockChild {
    Statement(PtStatement),
    VarDeclaration(PtVarDeclaration),
    Block(PtBlock)
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

        let mut statements: Vec<PtBlockChild> = vec![];
        for child in self.children {
            match child {
                TreeNode::ParseError(e) => {
                    return Err(e.into())
                },
                TreeNode::VarDeclaration(var_declaration) => {
                    let pt_var = var_declaration.try_into_pt(root_context.clone(), &context)?;
                    let scope_id = ScopeId::Var(VarScopeId::new(&context.uri, pt_var.name.clone(), pt_var.range));
                    let mut scope = Scope::new();
                    scope.values.push(ValueDeclarationReferable::Var(pt_var.clone()));
                    root_context.insert_scope(scope_id.clone(), scope)?;
                    context = context.extend(scope_id);
                    statements.push(PtBlockChild::VarDeclaration(pt_var));
                },
                TreeNode::Statement(statement) => {
                    let pt = statement.try_into_pt(root_context.clone(), &context)?;
                    statements.push(PtBlockChild::Statement(pt));
                },
                TreeNode::Block(block) => {
                    let pt = block.try_into_pt(root_context.clone(), &context)?;
                    let new_scope = ScopeId::Block(BlockScopedId::new(&context.uri, pt.range));
                    context = context.extend(new_scope);
                    statements.push(PtBlockChild::Block(pt));
                }
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
    pub statements: Vec<PtBlockChild>
}