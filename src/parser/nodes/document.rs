use std::sync::{Arc, Mutex};

use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{
    common::range::Range,
    parser::{
        program_tree::{
            function_declaration::FunctionDeclarationReferable, program_tree::{try_map_into_pt, CurrentContext, PtError, RootContext, TryIntoPt}, scope::{DocumentScopeId, Scope, ScopeId}, value_declaration::ValueDeclarationReferable
        },
        tree::{TreeNode, TreeNodeLike},
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

impl TryIntoPt<PtDocument> for AstDocument {
    fn try_into_pt(
        self,
        root_context: Arc<Mutex<RootContext>>,
        context: &CurrentContext,
    ) -> Result<PtDocument, PtError> {
        let range = self.range;
        let mut functions: Vec<AstFunctionDeclaration> = vec![];
        let mut uniforms: Vec<AstUniformDeclaration> = vec![];

        for child in self.children.into_iter() {
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

        let functions: Result<Vec<PtFunctionDeclaration>, PtError> =
            try_map_into_pt(functions, root_context.clone(), context);
        let functions = functions?;

        let uniforms: Result<Vec<PtUniformDeclaration>, PtError> =
            try_map_into_pt(uniforms, root_context.clone(), context);
        let uniforms = uniforms?;

        let mut root = root_context.lock().unwrap();

        let scope_id = ScopeId::Document(DocumentScopeId {
            uri: context.uri.to_owned(),
        });

        let scope = Scope {
            types: vec![],
            values: vec![],
            functions: functions
                .clone()
                .into_par_iter()
                .map(|fun| FunctionDeclarationReferable::UserFunction(fun))
                .collect(),
        };

        root.scopes.insert(scope_id, scope);

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
