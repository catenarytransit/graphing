use petgraph::*;
use gtfs_structures::*;
use std::sync::Arc;

struct Node {
    stop: (String, Arc<gtfs_structures::Stop>),
}
struct Edge {
    name: String, 
    route: gtfs_structures::Route
}

fn scan_gtfs(gtfs_zip: &str) -> Result<Gtfs, Error>  {
    let gtfs = gtfs_structures::GtfsReader::default()
    .read(gtfs_zip)?;
    gtfs.print_stats();
    Ok(gtfs)
}

fn graph(target: Gtfs) -> Graph<Node, Edge> {
    let mut deps = Graph::<Node, Edge>::new();
    let _stop = target.stops.into_iter().map(|s| deps.add_node(Node{stop:s}));
    //target.routes.into_iter().map(|c| deps.add_node(c));
    deps
}

fn main() {
    println!("Hello, world!");
    let _ = scan_gtfs("gtfs_bus.zip");
    println!("done");
}
