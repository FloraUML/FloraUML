pub use crate::class::ClassDeclaration;

pub type Declarations<'a> = Vec<Declaration<'a>>;

#[derive(Debug, PartialEq)]
pub enum Declaration<'a> {
    Class(ClassDeclaration<'a>),
}
