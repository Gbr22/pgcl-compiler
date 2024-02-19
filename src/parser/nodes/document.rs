use std::sync::{Arc, Mutex};

use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{
    common::range::Range,
    parser::{
        program_tree::{
            function_declaration::FunctionDeclarationReferable, program_tree::{try_map_into_pt, CurrentContext, PtError, RootContext, RootContextMutRef, RootContextMutRefType, TryIntoPt}, scope::{DocumentScopeId, Scope, ScopeId}, value_declaration::ValueDeclarationReferable
        },
        tree::{TreeNode, TreeNodeLike},
    },
};

use super::{
    function_declaration::{AstFunctionDeclaration, PtFunctionDeclaration},
    var_declaration::{AstVarDeclaration, PtVarDeclaration},
};

#[derive(Debug, Clone)]
pub struct AstDocument {
    pub range: Range,
    pub children: Vec<TreeNode>,
}

#[derive(Debug, Clone)]
pub struct PtDocument {
    pub range: Range,
    pub vars: Vec<PtVarDeclaration>,
    pub functions: Vec<PtFunctionDeclaration>,
}

impl TryIntoPt<PtDocument> for AstDocument {
    fn try_into_pt(
        self,
        mut root_context: RootContextMutRef,
        context: &CurrentContext,
    ) -> Result<PtDocument, PtError> {
        let range = self.range;
        let mut functions: Vec<AstFunctionDeclaration> = vec![];
        let mut vars: Vec<AstVarDeclaration> = vec![];

        
        let scope_id = ScopeId::Document(DocumentScopeId {
            uri: context.uri.clone(),
        });
        
        let scope = Scope::new();
        let context = context.to_owned().extend(scope_id.clone());

        root_context.insert_scope(scope_id, scope)?;

        for child in self.children.into_iter() {
            match child {
                TreeNode::FunctionDeclaration(fun) => {
                    functions.push(fun);
                }
                TreeNode::VarDeclaration(uni) => {
                    vars.push(uni);
                }
                TreeNode::ParseError(err) => {
                    return Err(err.into());
                }
                _ => {}
            }
        }

        let functions: Result<Vec<PtFunctionDeclaration>, PtError> =
            try_map_into_pt(functions, root_context.clone(), &context);
        let functions = functions?;

        let vars: Result<Vec<PtVarDeclaration>, PtError> =
            try_map_into_pt(vars, root_context.clone(), &context);
        let vars = vars?;

        Ok(PtDocument {
            range,
            functions,
            vars,
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
