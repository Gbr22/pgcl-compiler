use super::compound::CompoundType;
use super::simple::{AstSimpleType, PtSimpleType};
use crate::common::range::Range;
use crate::parser::program_tree::program_tree::{PtError, TryIntoPt};
use crate::parser::tree::TreeNodeLike;

#[derive(Debug, Clone)]
pub enum AstType {
    Simple(AstSimpleType),
    Compound(CompoundType),
}

impl AstTypeLike for AstType {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike> {
        match self {
            AstType::Simple(e) => e.to_node_like(),
            AstType::Compound(e) => e.to_node_like(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PtType {
    Simple(PtSimpleType)
}

impl TryIntoPt<PtType> for AstType {
    fn try_into_pt(
        self,
        root_context: std::sync::Arc<std::sync::Mutex<crate::parser::program_tree::program_tree::RootContext>>,
        context: &crate::parser::program_tree::program_tree::CurrentContext,
    ) -> Result<PtType, crate::parser::program_tree::program_tree::PtError> {
        match self {
            AstType::Simple(st) => {
                let simple_type = st.try_into_pt(root_context, context)?;

                Ok(PtType::Simple(simple_type))
            }
            _ => {
                Err(PtError {
                    range: Some(self.get_range()),
                    message: format!("Unknown type"),
                })
            }
        }
    }
}

pub trait PtTypeLike {
    fn get_range(&self) -> Range;
}

impl PtTypeLike for PtType {
    fn get_range(&self) -> Range {
        match self {
            PtType::Simple(e) => e.get_range(),
        }
    }
}

pub trait AstTypeLike {
    fn to_node_like(&self) -> Box<&dyn TreeNodeLike>;
}

impl TreeNodeLike for AstType {
    fn get_range(&self) -> Range {
        self.to_node_like().get_range()
    }
    fn children(&self) -> Vec<&crate::parser::tree::TreeNode> {
        self.to_node_like().children()
    }
}
