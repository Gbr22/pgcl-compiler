use crate::common::range::Range;
use crate::parser::program_tree::program_tree::{CurrentContext, PtError, RootContextMutRef, TryIntoPt};
use crate::parser::tree::{TreeNode, TreeNodeLike};

use super::ret::{PtReturnStatement, ReturnStatement};
use super::simple::{ExpressionStatement, PtExpressionStatement};

trait_enum! {
    #[derive(Debug, Clone)]
    pub enum Statement: StatementLike {
        ExpressionStatement,
        ReturnStatement
    }
}

pub trait StatementLike {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike>;
}

impl TreeNodeLike for Statement {
    fn get_range(&self) -> Range {
        self.to_node_like().get_range()
    }
    fn children(&self) -> Vec<&TreeNode> {
        self.to_node_like().children()
    }
}

#[derive(Debug, Clone)]
pub enum PtStatement {
    Expression(PtExpressionStatement),
    Return(PtReturnStatement)
}

impl TryIntoPt<PtStatement> for Statement {
    fn try_into_pt(
        self,
        root_context: RootContextMutRef,
        context: &CurrentContext,
    ) -> Result<PtStatement, PtError> {
        match self {
            Statement::ExpressionStatement(e) => Ok(PtStatement::Expression(e.try_into_pt(root_context, context)?)),
            Statement::ReturnStatement(r) => Ok(PtStatement::Return(r.try_into_pt(root_context, context)?)),
        }
    }
}