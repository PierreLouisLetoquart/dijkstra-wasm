use std::collections::BTreeMap;

use dijkstra::{dijkstra, Graph, GraphExt};

fn main() {
    // Create a graph representing a road network
    let mut road_network: Graph<&str, u32> = BTreeMap::new();

    // Add road connections between cities with distances
    road_network.add_edge("New York", "Boston", 215);
    road_network.add_edge("New York", "Philadelphia", 97);
    road_network.add_edge("Philadelphia", "Washington DC", 139);
    road_network.add_edge("Boston", "Philadelphia", 308);
    road_network.add_edge("Washington DC", "Richmond", 109);
    road_network.add_edge("Philadelphia", "Richmond", 267);

    // Find shortest paths from New York
    let shortest_paths = dijkstra(&road_network, "New York");

    // Print out the shortest paths
    println!("Shortest Paths from New York:");
    for (city, path_info) in shortest_paths.iter() {
        match path_info {
            Some((prev, distance)) => {
                println!("{} <- {} (Distance: {} miles)", city, prev, distance);
            }
            None => {
                println!("{} (Start City)", city);
            }
        }
    }

    // Optional: Demonstrate finding the path to a specific city
    if let Some(Some((prev, distance))) = shortest_paths.get("Richmond") {
        println!("\nPath to Richmond:");
        println!("Richmond <- {} (Total Distance: {} miles)", prev, distance);
    }
}
