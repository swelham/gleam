use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct EEP48Doc {
    pub name: String,
    pub arity: usize,
    pub signature: Vec<String>,
    pub doc: Option<HashMap<String, String>>,
    // pub location: DocMeta,
    pub typ: DocType,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DocType {
    Fn,
    TypeAlias,
    CustomType,
    ExternalFn,
    ExternalType,
}

impl std::hash::Hash for DocType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            DocType::Fn => 1.hash(state),
            DocType::TypeAlias => 2.hash(state),
            DocType::CustomType => 3.hash(state),
            DocType::ExternalFn => 4.hash(state),
            DocType::ExternalType => 5.hash(state),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ErlAnno {
    pub line: usize,
    pub column: usize,
    pub file: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CrossReference {
    #[allow(dead_code)]
    ModuleReference { module: Vec<String> },
    #[allow(dead_code)]
    ModuleItemReference {
        module: Vec<String>,
        name: String,
        arity: usize,
        typ: DocType,
    },
}

/// A data structure to hold data to generate a BEAM chunk
/// in compliance with http://erlang.org/eeps/eep-0048.html
#[derive(Debug, PartialEq)]
pub struct EEP48DocChunk {
    pub anno: ErlAnno,
    pub module_doc: HashMap<String, String>,
    // pub location: DocMeta,
    pub docs: Vec<EEP48Doc>,
}
