use std::io::Write;
use std::{collections::HashSet, fs::File};

use instances::InstanceReadError;
use petgraph::adj::NodeIndex;
use petgraph::dot::Config;
use petgraph::visit::{NodeRef, NodeIndexable};
use petgraph::{dot::Dot, visit::EdgeRef, Graph};

use crate::algos::{tsiligirides_s_algo::SAlgorithm, OrienteeringAlgo};

mod algos;
mod instances;

fn main() -> Result<(), InstanceReadError> {
    let (graph, node_positions) = instances::read_instance("res/tsiligirides2.txt")?;

    let path =
        match SAlgorithm::new(0.5f64, 10).generate_path(&graph, 0, graph.node_count() - 1, 20f64) {
            Some(path) => path,
            None => {
                println!("No path found");
                return Ok(());
            }
        };
    println!("Found path: {path:?}");
    println!(
        "Path score: {}",
        path.iter().map(|&n| graph[graph.from_index(n)]).sum::<f64>()
    );

    let path_edges = path
        .windows(2)
        .map(|edge| {
            graph
                .edges_connecting(edge[0].into(), edge[1].into())
                .next()
                .unwrap()
                .id()
        })
        .collect::<HashSet<_>>();
    let path = path.into_iter().collect::<HashSet<_>>();

    println!(
        "Path weight: {}",
        path_edges.iter().map(|&e| graph[e]).sum::<f64>()
    );

    let mut out_file = File::create("out.dot")?;

    writeln!(
        &mut out_file,
        "{}",
        Dot::with_attr_getters(
            &graph,
            &[Config::EdgeNoLabel],
            &|_, edge| if path_edges.contains(&edge.id()) {
                "color=red".to_owned()
            } else {
                "style=invis".to_owned()
            },
            &|_, node| format!(
                "pos=\"{},{}!\" ",
                node_positions[node.0.index()].0,
                node_positions[node.0.index()].1
            ) + if node.0.index() == 0 || node.0.index() == graph.node_count() - 1 {
                "color=pink"
            } else if path.contains(&node.0.index()) {
                "color=red"
            } else {
                ""
            }
        )
    )?;
    Ok(())
}

#[allow(unused)]
fn rnd_graph() -> Graph<f64, f64, petgraph::Undirected> {
    let mut graph = Graph::new_undirected();

    let s = graph.add_node(0.0);
    let v1 = graph.add_node(20.0);
    let v2 = graph.add_node(30.0);
    let v3 = graph.add_node(50.0);
    let v4 = graph.add_node(40.0);
    let e = graph.add_node(0.0);

    for i in graph.node_indices() {
        for j in graph.node_indices() {
            if i <= j {
                continue;
            }
            graph.update_edge(i, j, f64::INFINITY);
        }
    }

    graph.update_edge(s, v1, 50.0);
    graph.update_edge(s, v2, 60.0);
    graph.update_edge(v1, v3, 60.0);
    graph.update_edge(v1, v2, 20.0);
    graph.update_edge(v2, v3, 80.0);
    graph.update_edge(v1, v4, 60.0);
    graph.update_edge(v4, e, 20.0);
    graph.update_edge(s, v4, 10.0);
    graph.update_edge(v2, e, 40.0);
    graph.update_edge(v3, e, 40.0);

    graph
}

#[allow(unused)]
fn problem_graph() -> Graph<f64, f64, petgraph::Undirected> {
    let mut graph = Graph::new_undirected();

    let s = graph.add_node(0.0);
    let v1 = graph.add_node(100.0);
    let v2 = graph.add_node(100.0);
    let e = graph.add_node(0.0);

    for i in graph.node_indices() {
        for j in graph.node_indices() {
            if i <= j {
                continue;
            }
            graph.update_edge(i, j, f64::INFINITY);
        }
    }

    graph.update_edge(s, v1, 10.0);
    graph.update_edge(v1, v2, 10.0);
    graph.update_edge(v2, e, 10.0);

    graph
}
