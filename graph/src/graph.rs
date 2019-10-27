use std::collections::HashMap;


//Troubles with lifetimes for Vec<&V> ((
trait Graph<V> {
    fn vertices(&mut self) -> &Vec<V>;

    fn edges(&mut self) -> &Vec<(V, V)>;

    fn add_edges(a: V, b: V) -> Self;

    fn neighbours(vertex: V) -> Vec<V>;
}

//Single-thread variant
struct DirectedGraph<V: Clone> {
    adj_list: HashMap<V, Vec<V>>,
    vertices: Vec<V>,
    edges: Vec<(V, V)>,
    edges_memoize: bool,
}

impl<V: Clone> DirectedGraph<V> {
    pub fn new(adj_list: HashMap<V, Vec<V>>) -> Self {
        let vertices: Vec<V> = adj_list.keys().map(|x| (*x).clone()).collect();
        let edges: Vec<(V, V)> = adj_list.iter().flat_map(
            |x| x.1.into_iter().map(
                |y| (
                    (*x.0).clone(),
                    (*y).clone())).collect::<Vec<(V, V)>>())
            .collect();
        DirectedGraph { adj_list, vertices, edges, edges_memoize: false }
    }
}

impl<V: Clone> Graph<V> for DirectedGraph<V> {
    fn vertices(&mut self) -> &Vec<V> {
        &self.vertices
    }

    fn edges(&mut self) -> &Vec<(V, V)> {
        if self.edges_memoize {
            &self.edges
        } else {
            self.edges = self.adj_list.iter().flat_map(
                |x| x.1.into_iter().map(
                    |y| (
                        (*x.0).clone(),
                        (*y).clone())).collect::<Vec<(V, V)>>())
                .collect::<Vec<(V, V)>>();
            self.edges_memoize = true;
            &self.edges
        }
    }

    fn add_edges(a: V, b: V) -> Self {
        unimplemented!()
    }

    fn neighbours(vertex: V) -> Vec<V> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::{DirectedGraph, Graph};
    use std::collections::HashMap;

    fn sort_persistence<T: Clone + Ord + PartialOrd + PartialEq>(input: &Vec<T>) -> Vec<T> {
        let mut output = input.clone();
        output.sort();
        output
    }

    #[test]
    fn vertices_test() {
        let mut adj_list = HashMap::new();
        adj_list.insert(1, vec![4, 3]);
        adj_list.insert(4, vec![1]);
        adj_list.insert(3, vec![]);
        let mut graph = DirectedGraph::new(adj_list);
        assert_eq!(sort_persistence(graph.vertices()), vec![1, 3, 4]);
    }

    #[test]
    fn edges_test() {
        let mut adj_list = HashMap::new();
        adj_list.insert(1, vec![4, 3]);
        adj_list.insert(4, vec![1]);
        adj_list.insert(3, vec![4]);
        let mut graph = DirectedGraph::new(adj_list);
        assert_eq!((graph.edges().len()), 4);
        graph.edges().iter().for_each(|x|println!("{:?}",x));
    }
}