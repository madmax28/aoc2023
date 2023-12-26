use std::{
    collections::{HashMap, HashSet},
    fs,
    io::Read,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Edge<'a> {
    id: (&'a str, &'a str),
    ns: (&'a str, &'a str),
}

fn parse(input: &str) -> (HashMap<&str, HashSet<usize>>, HashMap<usize, Edge>) {
    let mut edges = HashSet::new();
    for line in input.lines() {
        let (n1, neighbors) = line.split_once(": ").unwrap();
        for n2 in neighbors.split_whitespace() {
            let edge = if n1 < n2 { (n1, n2) } else { (n2, n1) };
            edges.insert(Edge { id: edge, ns: edge });
        }
    }
    let edges: HashMap<usize, Edge> = edges.into_iter().enumerate().collect();

    let mut nodes: HashMap<&str, HashSet<usize>> = HashMap::new();
    for (idx, edge) in &edges {
        nodes.entry(edge.id.0).or_default().insert(*idx);
        nodes.entry(edge.id.1).or_default().insert(*idx);
    }

    (nodes, edges)
}

pub fn part1(input: &str) -> crate::Result<usize> {
    let (nodes, edges) = parse(input);

    let mut rand = fs::File::open("/dev/urandom").unwrap();
    let mut score: HashMap<(&str, &str), usize> = HashMap::new();
    let mut cut = HashSet::new();
    while cut.len() < 3 {
        let mut nodes = nodes.clone();
        let mut edges = edges.clone();

        while nodes.len() > 2 {
            let idx = {
                let mut bytes = [0u8; 8];
                rand.read_exact(&mut bytes).unwrap();
                *edges
                    .keys()
                    .nth(usize::from_le_bytes(bytes) % edges.len())
                    .unwrap()
            };

            let (kept, removed) = edges[&idx].ns;
            for edge_idx in nodes.remove(removed).unwrap() {
                let remove_edge = {
                    let edge = edges.get_mut(&edge_idx).unwrap();
                    if edge.ns.0 == removed {
                        edge.ns.0 = kept;
                    }

                    if edge.ns.1 == removed {
                        edge.ns.1 = kept;
                    }

                    edge.ns.0 == edge.ns.1
                };

                if remove_edge {
                    edges.remove(&edge_idx);
                    nodes.get_mut(kept).unwrap().retain(|i| i != &edge_idx);
                } else {
                    nodes.get_mut(kept).unwrap().insert(edge_idx);
                }
            }
        }

        for edge in edges.values() {
            let score = score.entry(edge.id).or_default();
            *score += 1;
            if *score > 20 {
                cut.insert(edge.id);
            }
        }
    }

    let graph: HashMap<&str, HashSet<&str>> = nodes
        .into_iter()
        .map(|(id, edge_indexes)| {
            let mut node_edges: HashSet<&str> = HashSet::new();
            node_edges.extend(
                edge_indexes
                    .into_iter()
                    .filter(|idx| !cut.contains(&edges[&idx].id))
                    .flat_map(|idx| [edges[&idx].id.0, edges[&idx].id.1])
                    .filter(|n| *n != id),
            );
            (id, node_edges)
        })
        .collect();

    let mut frontier = Vec::new();
    frontier.push(*graph.keys().next().unwrap());
    let mut seen = HashSet::new();
    while let Some(id) = frontier.pop() {
        for n in &graph[id] {
            if seen.insert(n) {
                frontier.push(n);
            }
        }
    }

    Ok(seen.len() * (graph.len() - seen.len()))
}
