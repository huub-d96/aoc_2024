use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::fs;
use std::time::Instant;

fn part_1(data: &str) -> usize {

    let mut nodes = HashMap::new();

    for line in data.lines(){
        let (n1, n2) = line.trim().split_once("-").unwrap();

        if let Entry::Vacant(e) = nodes.entry(n1) {
            e.insert(Vec::from([n2]));
        } else {
            nodes.get_mut(&n1).unwrap().push(n2)
        }

        if let Entry::Vacant(e) = nodes.entry(n2) {
            e.insert(Vec::from([n1]));
        } else {
            nodes.get_mut(&n2).unwrap().push(n1)
        }
    }

    let mut trifectas = HashSet::new();
    let mut sum = 0;
    for (node, neighbours) in &nodes {

        if node.as_bytes()[0]==b't'{
            for (i, n1) in neighbours.iter().enumerate(){
                for n2 in &neighbours[i+1..]{
                    if nodes[n1].contains(n2){

                        let mut trifecta = [*node, *n1, *n2];
                        trifecta.sort();
                        let trf_str = trifecta.join("-");
                        if !trifectas.contains(&trf_str){
                            sum += 1;
                            trifectas.insert(trf_str);
                        }                       
                    }
                }
            }
        }
    }

    sum
}

fn bron_kerbosch<'a>(r: Vec<&'a str>, mut p: Vec<&'a str>, mut x: Vec<&'a str>, neighbours: &'a HashMap<&'a str, Vec<&'a str>>) -> Vec<&'a str>{

    if p.is_empty() && x.is_empty(){
        return r
    }   

    let mut output = Vec::new();
    for node in &p.clone(){
        let mut new_r = r.clone();
        new_r.push(node);

        let mut new_p = Vec::new();
        let mut new_x = Vec::new();

        for n in &neighbours[node]{
            if p.contains(n){
                new_p.push(*n)
            }
            if x.contains(n){
                new_x.push(*n)
            }
        }

        let result = bron_kerbosch(new_r, new_p, new_x, neighbours);

        if result.len() > output.len(){
            output = result;
        }
        p.remove(p.iter().position(|v| v == node).unwrap());
        x.push(node);
    }

    output
}

fn part_2(data: &str) -> String {

    let mut nodes = HashMap::new();

    for line in data.lines(){
        let (n1, n2) = line.trim().split_once("-").unwrap();

        if let Entry::Vacant(e) = nodes.entry(n1) {
            e.insert(Vec::from([n2]));
        } else {
            nodes.get_mut(&n1).unwrap().push(n2)
        }

        if let Entry::Vacant(e) = nodes.entry(n2) {
            e.insert(Vec::from([n1]));
        } else {
            nodes.get_mut(&n2).unwrap().push(n1)
        }
    }

    let p = nodes.keys().cloned().collect();
    let r = Vec::new();
    let x = Vec::new();

    let mut maxclique = bron_kerbosch(r, p, x, &nodes);
    maxclique.sort();

    maxclique.join(",")    
}


fn main() {
    let data= fs::read_to_string("src/data.txt").expect("Failed to read");

    println!("***Day 23***");

    //Part 1
    let timer = Instant::now();
    let result_1 = part_1(&data);
    let time_1 = timer.elapsed();

    println!("Part 1: {:?} - Duration: {:?}", result_1, time_1);

    //Part 2
    let timer = Instant::now();

    let result_2 = part_2(&data);
    let time_2 = timer.elapsed();
    println!("Part 2: {:?} - Duration: {:?}", result_2, time_2);
}