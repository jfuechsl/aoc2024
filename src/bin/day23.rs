use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    fmt::Display,
};

use aoc2024::utils::file::load_file_lines;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Computer {
    code: [char; 2],
}

impl Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.code[0], self.code[1])
    }
}

impl Computer {
    fn new(code: &str) -> Self {
        Self {
            code: [code.chars().nth(0).unwrap(), code.chars().nth(1).unwrap()],
        }
    }

    fn starts_with_t(&self) -> bool {
        self.code[0] == 't'
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct InterconnectedComputers {
    computers: BTreeSet<Computer>,
}

impl Display for InterconnectedComputers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.computers.iter().sorted().join(","))?;
        Ok(())
    }
}

impl Default for InterconnectedComputers {
    fn default() -> Self {
        Self {
            computers: BTreeSet::new(),
        }
    }
}

impl InterconnectedComputers {
    fn len(&self) -> usize {
        self.computers.len()
    }

    fn add_computer(&mut self, computer: Computer) -> bool {
        !self.computers.insert(computer)
    }

    fn has_starts_with_t(&self) -> bool {
        self.computers.iter().any(|c| c.starts_with_t())
    }
}

struct ComputerGraph {
    graph: BTreeMap<Computer, BTreeSet<Computer>>,
}

impl Default for ComputerGraph {
    fn default() -> Self {
        Self {
            graph: BTreeMap::new(),
        }
    }
}

impl ComputerGraph {
    fn add_edge(&mut self, a: Computer, b: Computer) {
        self.graph.entry(a).or_default().insert(b);
        self.graph.entry(b).or_default().insert(a);
    }

    fn find_k_cliques_rec(&self, k: usize) -> BTreeSet<InterconnectedComputers> {
        let mut cliques = BTreeSet::new();
        for (vertex, neighbours) in self.graph.iter() {
            let mut current = InterconnectedComputers::default();
            current.add_computer(*vertex);
            self.extend_clique_rec(current, neighbours, k, &mut cliques);
        }
        cliques
    }

    fn extend_clique_rec(
        &self,
        current: InterconnectedComputers,
        candidates: &BTreeSet<Computer>,
        k: usize,
        cliques: &mut BTreeSet<InterconnectedComputers>,
    ) {
        if current.len() == k {
            cliques.insert(current);
            return;
        }

        for candidate in candidates.iter() {
            // is candidate connected to all computers in current?
            let cneighbours = self.graph.get(candidate).unwrap();
            let is_fully_connected = current.computers.is_subset(cneighbours);
            if is_fully_connected {
                let mut new_current = current.clone();
                if !new_current.add_computer(*candidate) {
                    self.extend_clique_rec(new_current, cneighbours, k, cliques);
                }
            }
        }
    }

    fn bron_kerbosch_find_cliques_rec(
        &self,
        current: InterconnectedComputers,
        candidates: &BTreeSet<Computer>,
        excluded: &BTreeSet<Computer>,
        max_clique: &mut InterconnectedComputers,
    ) {
        if candidates.is_empty() && excluded.is_empty() {
            if current.len() > max_clique.len() {
                *max_clique = current;
            }
            return;
        }

        let pivot =
            self.bron_kerbosch_pivot(&candidates.union(excluded).copied().collect(), candidates);

        let mut new_candidates = candidates.clone();
        let mut new_excluded = excluded.clone();
        for c in candidates.difference(self.graph.get(&pivot).unwrap()) {
            let neighbours = self.graph.get(c).unwrap();
            let mut new_current = current.clone();
            new_current.add_computer(*c);
            self.bron_kerbosch_find_cliques_rec(
                new_current,
                &new_candidates.intersection(neighbours).copied().collect(),
                &new_excluded.intersection(neighbours).copied().collect(),
                max_clique,
            );
            new_candidates.remove(c);
            new_excluded.insert(*c);
        }
    }

    fn bron_kerbosch_pivot(
        &self,
        computers: &BTreeSet<Computer>,
        candidates: &BTreeSet<Computer>,
    ) -> Computer {
        let mut max_connections = -1;
        let mut pivot = None;
        for computer in computers.iter() {
            let num_connections = candidates
                .intersection(self.graph.get(&computer).unwrap())
                .count() as isize;
            if num_connections > max_connections {
                max_connections = num_connections;
                pivot = Some(*computer);
            }
        }
        pivot.unwrap()
    }
}

fn main() {
    let filename = "inputs/day23.txt";
    let lines = load_file_lines(filename).expect("Invalid filename");
    let mut graph = ComputerGraph::default();
    lines.iter().for_each(|line| {
        let parts: Vec<&str> = line.split('-').collect();
        let (ca, cb) = (Computer::new(parts[0]), Computer::new(parts[1]));
        graph.add_edge(ca, cb);
    });

    // Part 1
    let found_3_cliques = graph.find_k_cliques_rec(3);
    let cliques_with_t: HashSet<_> = found_3_cliques
        .iter()
        .filter(|g| g.has_starts_with_t())
        .collect();
    assert_eq!(cliques_with_t.len(), 1253);
    println!(
        "Found {} 3-cliques with one computer starting with t",
        cliques_with_t.len()
    );

    // Part 2
    let mut max_clique = InterconnectedComputers::default();
    graph.bron_kerbosch_find_cliques_rec(
        InterconnectedComputers::default(),
        &graph.graph.keys().copied().collect(),
        &BTreeSet::new(),
        &mut max_clique,
    );
    let lan_party_password = max_clique.to_string();
    assert_eq!(lan_party_password, "ag,bt,cq,da,hp,hs,mi,pa,qd,qe,qi,ri,uq");
    println!("LAN Party Password: {}", lan_party_password);
}
