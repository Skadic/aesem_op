use std::path::Path;

use petgraph::{Graph, Undirected, adj::NodeIndex};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InstanceReadError {
    #[error("cannot open file")]
    File(#[from] std::io::Error),
    #[error("missing token \"{0}\"")]
    MissingToken(String),
    #[error("invalid token \"{token}\" for token {token_name}, expected {expected_type}")]
    InvalidToken {
        token: String,
        token_name: &'static str,
        expected_type: &'static str,
    },
}

impl InstanceReadError {
    #[allow(unused)]
    pub fn expected_int(token: impl AsRef<str>, token_name: &'static str) -> Self {
        Self::InvalidToken {
            token: token.as_ref().into(),
            token_name,
            expected_type: "integer",
        }
    }

    #[allow(unused)]
    pub fn expected_float(token: impl AsRef<str>, token_name: &'static str) -> Self {
        Self::InvalidToken {
            token: token.as_ref().into(),
            token_name,
            expected_type: "floating point number",
        }
    }
}

pub fn read_instance(path: impl AsRef<Path>) -> Result<(Graph<f64, f64, Undirected, usize>, Vec<(f64, f64)>), InstanceReadError> {
    use InstanceReadError::*;
    let mut graph = Graph::default();

    let file = std::fs::read_to_string(path)?;
    let lines = file
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim());

    let mut node_positions = Vec::with_capacity(file.lines().count());

    macro_rules! parse_single {
        ($it:ident, $name:literal, $t:ty) => {{
            let token = $it.next().ok_or_else(|| MissingToken($name.into()))?;
            token
                .parse::<$t>()
                .map_err(|_| InstanceReadError::InvalidToken {
                    token: token.into(),
                    token_name: $name,
                    expected_type: stringify!($t),
                })?
        }};
    }

    for line in lines {
        let mut split = line.split_whitespace();
        let x = parse_single!(split, "x coordinate", f64);
        let y = parse_single!(split, "y coordinate", f64);
        let score = parse_single!(split, "score", f64);
        let new_node = graph.add_node(score);
        node_positions.push((x, y));

        for node in graph.node_indices() {
            if node == new_node {
                continue;
            }
            let (other_x, other_y) = node_positions[node.index()];
            graph.add_edge(new_node, node, ((x - other_x).powi(2) + (y - other_y).powi(2)).sqrt());
        }
    }

    Ok((graph, node_positions))
}
