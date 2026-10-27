use crate::model::Model;

/// A `.tf` file passed in by the caller (the core never reads the filesystem).
#[derive(Debug, Clone)]
pub struct SourceFile {
    pub path: String,
    pub contents: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("failed to parse {path}: {source}")]
    Hcl {
        path: String,
        #[source]
        source: hcl::Error,
    },
}

/// Parses Terraform sources into a [`Model`].
///
/// TODO(you): implement. Hints:
/// - `hcl::parse(&file.contents)` returns an `hcl::Body`; map its error to `ParseError::Hcl`.
/// - Iterate `body.blocks()`; match on `block.identifier()` ("resource", "data", "module",
///   "variable", "output") and read `block.labels()` for type/name.
/// - Edges: walk each block's attribute `Expression`s recursively and collect
///   `Expression::Traversal` (e.g. `aws_vpc.main.id` -> `aws_vpc.main`). References inside
///   strings like `"${aws_vpc.main.id}"` show up as `Expression::TemplateExpr` — parse with
///   `hcl::Template::from_expr`. Don't forget nested blocks and `depends_on`.
/// - Sort nodes and edges so snapshot output is deterministic.
pub fn parse(files: &[SourceFile]) -> Result<Model, ParseError> {
    let _ = files;
    todo!("implement Terraform parsing")
}
