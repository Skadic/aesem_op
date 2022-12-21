use std::io::Write;
use std::{collections::HashSet, fs::File};

use instances::InstanceReadError;

use log::info;
use petgraph::dot::Config;
use petgraph::{dot::Dot, visit::EdgeRef, Graph};

use crate::algos::szwarc_boryczka::util::f64_cmp;
use crate::algos::szwarc_boryczka::SzwarcBoryczka;
use crate::algos::Solution;
use crate::algos::{tsiligirides_s_algo::SAlgorithm, OrienteeringAlgo};

mod algos;
mod instances;

fn main() -> Result<(), InstanceReadError> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    };
    pretty_env_logger::init();

    let (graph, node_positions) = instances::read_instance("res/tsiligirides1.txt")?;

    let mut algo = SAlgorithm::new(4f64, 3);
    //let mut algo = SzwarcBoryczka::new(50, 0.97, 0.0, 100000)
        //.unwrap();

    let Solution { path, score, cost } = match (0..1)
        .filter_map(|_| algo.generate_path(&graph, 0, graph.node_count() - 1, 85f64))
        .max_by(|a, b| f64_cmp(&a.score, &b.score))
    {
        Some(path) => path,
        None => {
            info!("No path found");
            return Ok(());
        }
    };
    info!("Found path: {path:?}");
    info!("Path score: {score}");
    info!("Path cost: {cost}");

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

    let mut out_file = File::create("out.dot")?;

    writeln!(
        &mut out_file,
        "{}",
        Dot::with_attr_getters(
            &graph,
            &[Config::EdgeNoLabel, Config::NodeIndexLabel],
            &|_, edge| if path_edges.contains(&edge.id()) {
                "color=red".to_owned()
            } else {
                "style=invis".to_owned()
            },
            &|_, node| format!(
                "pos=\"{},{}\" ",
                node_positions[node.0.index()].0 * 50.0,
                node_positions[node.0.index()].1 * 50.0
            ) + if node.0.index() == 0 || node.0.index() == graph.node_count() - 1 {
                "color=green"
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
