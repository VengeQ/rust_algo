use std::collections::{HashMap, BTreeMap, VecDeque};
use std::fmt::Display;
use std::fmt;
use std::time::Duration;

type Matrix = Vec<Vec<bool>>;


struct MGraph {
    vertex: BTreeMap<String, usize>,
    edges: Matrix,
}

impl MGraph {
    pub fn new_empty(n: usize) -> Self {
        let x = vec![false; n];
        let edges = vec![x.clone(); n];
        let v: Vec<(String, usize)> = (0..n).collect::<Vec<usize>>().into_iter().enumerate()
            .map(|(x, y)| (x.to_string(), y)).collect();
        let vertex: BTreeMap<_, _> = v.into_iter().collect();

        MGraph {
            vertex,
            edges,
        }
    }

    pub fn edges(&self) -> &Matrix {
        &self.edges
    }
    pub fn vertex(&self) -> &BTreeMap<String, usize> {
        &self.vertex
    }

    pub fn connect(&mut self, v1: String, v2: String) {
        if v1 == v2 { return; }
        let first_vertex = self.vertex.get(&v1[..]);
        let second_vertex = self.vertex.get(&v2[..]);
        if let Some(first) = first_vertex {
            if let Some(second) = second_vertex {
                self.edges[*first][*second] = true;
                self.edges[*second][*first] = true;
                println!("connect {} with {}", v1, v2);
            }
        }
    }

    pub fn disconnect(&mut self, v1: String, v2: String) {
        if v1 == v2 { return; }
        let first_vertex = self.vertex.get(&v1[..]);
        let second_vertex = self.vertex.get(&v2[..]);
        if let Some(first) = first_vertex {
            if let Some(second) = second_vertex {
                self.edges[*first][*second] = false;
                self.edges[*second][*first] = false;
                println!("disconnect {} with {}", v1, v2);
            }
        }
    }

    pub fn bfs(&mut self, process_vertex_early:fn(String)->()) {
        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        enum VertexType {
            Undiscovered,
            Discovered,
            Processed,
        }
        use VertexType::*;
        let mut statuses: Vec<VertexType> = vec![VertexType::Undiscovered; self.vertex.len()];
        let vertex_keys: &Vec<&String> = &self.vertex.keys().collect();
        let mut queue = VecDeque::new();
        queue.push_front(vertex_keys[0].clone());
        statuses[*self.vertex.get(vertex_keys[0]).unwrap()] = Discovered;
        while !queue.is_empty() {
            let current = queue.pop_back().unwrap();
            let key = *self.vertex.get(&current).unwrap();
            statuses[key] = Processed;
            process_vertex_early(format!("{} was processed!",key));
            let childs = &self.edges[key];
            for idx in 0..childs.len() {
                if childs[idx] == true {
                    if !(statuses[idx] == Processed) {
                        if !(statuses[idx] == Discovered) {
                            println!("discovered_pairs: {} <-> {}", idx, current);
                            ///TODO Будет работать только если ключ равен номеру вершины
                            queue.push_front(idx.to_string());
                            statuses[idx] = Discovered;
                        }
                    }
                }
            }
        }
    }
}


impl fmt::Display for MGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        result.push_str(&format!("\n"));
        for (k, v) in &self.vertex {
            result.push_str(&format!("   {}   ", v));
        }
        result.push_str(&format!("\n"));
        for x in &self.edges {
            for y in x {
                result.push_str(&format!(" {:6}", y.to_string()));
            }
            result.push_str(&format!("\n"));
        }

        write!(f, "{}", result)
    }
}


#[cfg(test)]
mod tests {
    use std::iter::Map;
    use std::collections::{HashMap, BTreeMap};

    #[test]
    fn new_empty_test() {
        use super::MGraph;
        use super::Matrix;
        let n = 5;
        let graph = MGraph::new_empty(n);
        let mut expected_vertex = BTreeMap::new();
        for c in 0..n {
            expected_vertex.insert(c.to_string(), c);
        }
        assert_eq!(graph.vertex(), &expected_vertex);
        let xx = (0..n).collect::<Vec<usize>>().iter().map(|v| false).collect::<Vec<bool>>();
        let xy = (0..n).collect::<Vec<usize>>().iter().map(|v| xx.clone()).collect::<Matrix>();
        assert_eq!(graph.edges(), &xy);
    }

    #[test]
    fn connect_test() {
        use super::MGraph;
        use super::Matrix;
        let n = 5;
        let mut graph = MGraph::new_empty(n);
        graph.connect("2".to_string(), "3".to_string());
        assert_eq!(graph.edges.get(2).unwrap().get(3).unwrap(), &true);
        assert_eq!(graph.edges.get(3).unwrap().get(2).unwrap(), &true);
        assert_eq!(graph.edges.get(2).unwrap().get(4).unwrap(), &false);
    }

    #[test]
    fn disconnect_test() {
        use super::MGraph;
        use super::Matrix;
        let n = 10;
        let mut graph = MGraph::new_empty(n);
        graph.connect("2".to_string(), "3".to_string());
        assert_eq!(graph.edges.get(2).unwrap().get(3).unwrap(), &true);
        assert_eq!(graph.edges.get(3).unwrap().get(2).unwrap(), &true);
        println!("{}", graph);
        graph.disconnect("3".to_string(), "2".to_string());
        assert_eq!(graph.edges.get(2).unwrap().get(3).unwrap(), &false);
        assert_eq!(graph.edges.get(3).unwrap().get(2).unwrap(), &false);
        println!("{}", graph);
    }

    #[test]
    fn bfs_easy_test() {
        use super::MGraph;
        use super::Matrix;
        let n = 5;
        let mut graph = MGraph::new_empty(n);
        graph.connect("2".to_string(), "3".to_string());
        graph.connect("2".to_string(), "4".to_string());
        graph.connect("1".to_string(), "4".to_string());
        graph.connect("0".to_string(), "3".to_string());
        graph.bfs(|str:String|println!("{}",str));
    }
}