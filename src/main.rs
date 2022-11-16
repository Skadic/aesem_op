use std::collections::HashSet;

use petgraph::{dot::Dot, visit::EdgeRef, Graph};

use crate::algos::{tsiligiridi_s_algo::SAlgorithm, OrienteeringAlgo};

mod algos;

fn main() {
    let graph = problem_graph();

    let path =
        match SAlgorithm::new(0.5f64, 10).generate_path(&graph, 0, graph.node_count() - 1, 150.0) {
            Some(path) => path,
            None => return,
        }
        .collect::<Vec<_>>();
    let path_edges = path
        .windows(2)
        .map(|edge| {
            graph
                .edges_connecting(edge[0], edge[1])
                .next()
                .unwrap()
                .id()
        })
        .collect::<HashSet<_>>();
    let path = path.into_iter().collect::<HashSet<_>>();

    //println!("Path: {:?}", path.map(|iter| iter.collect::<Vec<_>>()));

    println!(
        "{}",
        Dot::with_attr_getters(
            &graph,
            &[],
            &|_, edge| if path_edges.contains(&edge.id()) {
                "color=red".to_owned()
            } else {
                "".to_owned()
            },
            &|_, node| if path.contains(&node.0) {
                "color=red".to_owned()
            } else {
                "".to_owned()
            }
        )
    );
}

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
