use serde::Serialize;

/// Everything found in a Terraform configuration.
#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct Model {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Node {
    /// Terraform address, e.g. `aws_vpc.main`, `data.aws_ami.ubuntu`, `module.network`, `var.region`.
    pub id: String,
    /// File the block was declared in.
    pub file: String,
    #[serde(flatten)]
    pub kind: NodeKind,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum NodeKind {
    Resource {
        r#type: String,
        name: String,
    },
    Data {
        r#type: String,
        name: String,
    },
    Module {
        name: String,
        source: Option<String>,
    },
    Variable {
        name: String,
    },
    Output {
        name: String,
    },
}

/// `from` references `to` (by expression or `depends_on`). Values are node ids.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Edge {
    pub from: String,
    pub to: String,
}
