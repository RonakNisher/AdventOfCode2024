
use std::collections::{BTreeSet, HashMap, HashSet};
use itertools::Itertools;

pub fn bron_kerbosch(
    r: &mut BTreeSet<String>,
    p: &mut HashSet<String>,
    x: &mut HashSet<String>,
    graph: &HashMap<String, HashSet<String>>,
    cliques: &mut Vec<BTreeSet<String>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
        return;
    }

    let mut p_clone = p.clone();
    let pivot = p.union(x).next().unwrap().clone();
    let neighbors = graph.get(&pivot).unwrap();

    for v in p.difference(neighbors) {
        let v = v.clone();
        r.insert(v.clone());
        let mut p_new = p.intersection(graph.get(&v).unwrap()).cloned().collect();
        let mut x_new = x.intersection(graph.get(&v).unwrap()).cloned().collect();
        bron_kerbosch(r, &mut p_new, &mut x_new, graph, cliques);
        r.remove(&v);
        p_clone.remove(&v);
        x.insert(v);
    }
}

pub fn solve(input: String) {
    let mut result = 0;

    let mut computer_sets: HashSet<Vec<String>> = HashSet::new();
    let mut computer_map: HashMap<String, HashSet<String>> = HashMap::new();

    for line in input.lines() {
        let (computer_a, computer_b) = line.split("-").collect_tuple().unwrap();

        computer_map.entry(computer_a.to_string()).or_insert(HashSet::new()).insert(computer_b.to_string());
        computer_map.entry(computer_b.to_string()).or_insert(HashSet::new()).insert(computer_a.to_string());
    }

    computer_map.iter().for_each(|(key, connections)| {
        for connection in connections {
            
            let connection_computers = computer_map.get(connection).unwrap();
            let possible_values = connection_computers.intersection(connections);

            for possible_value in possible_values {
                let mut connected_set: Vec<String> = Vec::new();
                connected_set.push(key.to_string());
                connected_set.push(connection.to_string());
                connected_set.push(possible_value.to_string());

                connected_set.sort(); // so we avoid adding duplicate values in the computer_sets
                
                computer_sets.insert(connected_set);
            }
        }
    });

    result = computer_sets
        .iter()
        .filter(|&x| x.iter().any(|c| c.starts_with("t")))
        .count();
    
    // part 2

    let mut cliques = Vec::new();
    let mut r = BTreeSet::new();
    let mut p: HashSet<String> = computer_map.keys().cloned().collect();
    let mut x = HashSet::new();

    bron_kerbosch(&mut r, &mut p, &mut x, &computer_map, &mut cliques);
    let max_length_set = cliques.iter().max_by_key(|x| x.len()).unwrap();

    println!("*******************");
    println!("Solved Day 1 Part 1: {}", result);
    println!("Solved Day 1 Part2: {}", max_length_set.iter().join(","));
    println!("*******************");
} 