use std::collections::{BTreeMap, HashSet};
use std::collections::btree_map::Entry;

type List = BTreeMap<String, HashSet<String>>;

#[derive(Debug)]
struct LGraph {
    vertex: BTreeMap<String, usize>,
    edges: List,
}

impl LGraph {
    pub fn new_empty(n: usize) -> Self {
        let v: Vec<(String, usize)> = (0..n).collect::<Vec<usize>>().into_iter().enumerate()
            .map(|(x, y)| (x.to_string(), y)).collect();
        let vertex: BTreeMap<_, _> = v.into_iter().collect();
        let mut edges: BTreeMap<String, HashSet<String>> = BTreeMap::new();
        for x in 0..n {
            edges.insert(x.to_string(), HashSet::<String>::new());
        };

        LGraph { vertex, edges }
    }

    pub fn edges(&self) -> &List {
        &self.edges
    }
    pub fn vertex(&self) -> &BTreeMap<String, usize> {
        &self.vertex
    }

    pub fn connect(& mut self, v1: String, v2: String) {
        if v1 == v2 {
            return;
        }
        let first_vertex = self.vertex.get(&v1[..]);
        let second_vertex = self.vertex.get(&v2[..]);
        if let Some(first) = first_vertex {
            if let Some(second) = second_vertex {
                self.edges.entry(v1.clone()).and_modify(|x|{
                    x.insert(v2.clone());
                });
                self.edges.entry(v2.clone()).and_modify(|x|{
                    x.insert(v1.clone());
                });
            }
        }
    }

    pub fn disconnect(& mut self, v1: String, v2: String) {
        if v1 == v2 {
            return;
        }
        let first_vertex = self.vertex.get(&v1[..]);
        let second_vertex = self.vertex.get(&v2[..]);
        if let Some(first) = first_vertex {
            if let Some(second) = second_vertex {
                self.edges.entry(v1.clone()).and_modify(|x|{
                    x.remove(&v2.clone());
                });
                self.edges.entry(v2.clone()).and_modify(|x|{
                    x.remove(&v1.clone());
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    #[test]
    fn new_empty_test() {
        use super::LGraph;
        let n = 5;
        let graph = LGraph::new_empty(n);
        let mut expected_vertex = BTreeMap::new();
        for c in 0..n {
            expected_vertex.insert(c.to_string(), c);
        }
        assert_eq!(&graph.vertex, &expected_vertex);

        for (k, v) in graph.edges {
            assert_eq!(v.len(), 0);
        }
    }

    #[test]
    fn connect_test() {
        use super::LGraph;
        let n = 5;
        let mut graph = LGraph::new_empty(n);
        graph.connect("2".to_string(), "3".to_string());
        graph.connect("4".to_string(), "1".to_string());
        graph.connect("4".to_string(), "2".to_string());
        graph.connect("0".to_string(), "0".to_string());
        println!("{:?}", graph.edges);
        graph.disconnect("4".to_string(), "2".to_string());
        println!("{:?}", graph.edges);
    }
}