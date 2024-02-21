use crate::parser::program_tree::scope::Referable;

#[derive(Debug, Clone)]
pub enum TypeDeclarationReferable {
    Primitive(PrimitiveTypeDeclaration),
}

pub trait TypeDeclarationReferableLike: Referable {
    fn to_string(&self) -> String;
    fn get_description(&self) -> Option<String>;
}

impl Referable for TypeDeclarationReferable {
    fn get_name(&self) -> &str {
        use TypeDeclarationReferable as T;
        match self {
            T::Primitive(e) => e.get_name(),
        }
    }
}
impl TypeDeclarationReferableLike for TypeDeclarationReferable {
    fn to_string(&self) -> String {
        use TypeDeclarationReferable as T;
        match self {
            T::Primitive(e) => e.to_string(),
        }
    }

    fn get_description(&self) -> Option<String> {
        use TypeDeclarationReferable as T;
        match self {
            T::Primitive(e) => e.get_description(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrimitiveTypeDeclaration {
    pub name: String,
    pub description: Option<String>,
}

impl Referable for PrimitiveTypeDeclaration {
    fn get_name(&self) -> &str {
        &self.name
    }
}
impl TypeDeclarationReferableLike for PrimitiveTypeDeclaration {
    fn to_string(&self) -> String {
        self.name.to_string()
    }
    fn get_description(&self) -> Option<String> {
        self.description.clone()
    }
}
