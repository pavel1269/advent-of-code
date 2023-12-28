use std::collections::{HashMap, HashSet};

use rand::seq::IteratorRandom;

fn main() {
    let input = get_input();
    let result_part1 = part1(&input);
    println!("Part1: {}", result_part1);
}

fn part1(input: &str) -> usize {
    let graph = Graph::from_str(input);

    let mut _iter_index = 1;
    loop {
        let (cut_size, group1_size, group2_size) = try_contracting_graph(&graph);

        if cut_size == 3 {
            // println!("took {} attempts", _iter_index);
            return group1_size * group2_size;
        }
        _iter_index += 1;
    }
}

fn try_contracting_graph(graph: &Graph) -> (usize, usize, usize) {
    let mut graph = graph.clone();
    let mut rng = rand::thread_rng();

    while graph.count_alive_nodes() > 2 {
        let connection = graph.connections.keys().choose(&mut rng).unwrap().clone();
        graph.shring_connection(connection);
    }

    assert_eq!(1, graph.connections.len());
    let groups = graph
        .nodes
        .iter()
        .map(|nodes| nodes.len())
        .filter(|size| size > &0)
        .collect::<Vec<_>>();
    assert_eq!(2, groups.len());
    let cut_size = graph.connections.values().cloned().next().unwrap();

    return (cut_size, groups[0], groups[1]);
}

type Connection = (usize, usize);

trait ConnectionExt {
    fn from_ids(from: usize, to: usize) -> Self;
}

impl ConnectionExt for Connection {
    fn from_ids(from: usize, to: usize) -> Self {
        return (from.min(to), from.max(to));
    }
}

#[derive(Debug, Clone)]
struct Graph {
    nodes: Vec<HashSet<String>>,
    connections: HashMap<Connection, usize>,
}

impl Graph {
    fn shring_connection(&mut self, connection: Connection) {
        let node_index_to_remove = connection.1;
        // shring node
        let removed_node = self.nodes.get(node_index_to_remove).unwrap().clone();
        self.nodes
            .get_mut(connection.0)
            .unwrap()
            .extend(removed_node.into_iter());
        self.nodes.get_mut(node_index_to_remove).unwrap().clear();

        // remove self-connection
        self.connections.remove(&connection);

        // handle connections to removed node
        for connection_other in self
            .connections
            .keys()
            .filter(|(from, to)| &node_index_to_remove == from || &node_index_to_remove == to)
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
        {
            let foreign_node = if connection_other.0 == node_index_to_remove {
                connection_other.1
            } else {
                connection_other.0
            };
            let size = self.connections.get(&connection_other).unwrap().clone();
            self.connections.remove(&connection_other);

            let new_connection = Connection::from_ids(connection.0, foreign_node);
            *self.connections.entry(new_connection).or_insert(0) += size;
        }
    }

    fn count_alive_nodes(&self) -> usize {
        self.nodes.iter().filter(|node| node.len() > 0).count()
    }

    fn from_str(str: &str) -> Self {
        let mut connections = HashSet::new();
        let mut nodes = HashSet::new();
        for line in str.lines() {
            let split: [&str; 2] = line
                .split(':')
                .map(|str| str.trim())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let node_name = split[0].to_string();
            nodes.insert(node_name.clone());
            for connection in split[1].split(' ').map(|str| str.trim().to_string()) {
                nodes.insert(connection.clone());
                connections.insert((node_name.clone(), connection));
            }
        }
        let nodes = nodes
            .into_iter()
            .map(|node| HashSet::from([node]))
            .collect::<Vec<_>>();
        let connections = connections
            .into_iter()
            .map(|(from, to)| {
                let from_id = Self::get_node_id(&from, &nodes);
                let to_id = Self::get_node_id(&to, &nodes);
                return ((from_id.min(to_id), from_id.max(to_id)), 1);
            })
            .collect::<HashMap<_, _>>();

        let result = Self { nodes, connections };
        return result;
    }

    fn get_node_id(name: &String, nodes: &Vec<HashSet<String>>) -> usize {
        nodes
            .iter()
            .enumerate()
            .find(|(_, node)| node.contains(name))
            .map(|(index, _)| index)
            .unwrap()
    }
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        include_str!("./example.txt")
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let result = part1(&input);
        assert_eq!(result, 54);
    }
}
