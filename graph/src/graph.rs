use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;
use std::io::Error;
use std::hash::Hash;
use std::fmt::Debug;


//Troubles with lifetimes for Vec<&V> ((
trait Graph<V> {
    fn vertices(&self) -> &HashSet<V>;

    fn edges(&mut self) -> &Vec<(V, V)>;

    fn add_edges(&mut self, a: V, b: V) -> Result<(), Error>;

    fn neighbours(&self, vertex: &V) -> HashSet<V>;
}

//Single-thread variant
struct DirectedGraph<V: Clone + Hash + Eq + Debug + PartialOrd + Ord> {
    adj_list: HashMap<V, Vec<V>>,
    vertices: Rc<HashSet<V>>,
    edges: Vec<(V, V)>,
    edges_memoize: bool,
}

impl<V: Clone + Hash + Eq + Debug + PartialOrd + Ord> DirectedGraph<V> {
    pub fn new(adj_list: HashMap<V, Vec<V>>) -> Self {
        let vertices_1: HashSet<&V> = adj_list.keys().collect();
        let vertices_2: HashSet<&V> = adj_list.iter().flat_map(|x| x.1.iter()).collect();
        let vertices = (vertices_1.union(&vertices_2)).into_iter().map(|x| (*x).clone()).collect();
        let vertices_rc = Rc::new(vertices);
        let edges: Vec<(V, V)> = adj_list.iter().flat_map(
            |x| x.1.into_iter().map(
                |y| (
                    (*x.0).clone(),
                    (*y).clone())).collect::<Vec<(V, V)>>())
            .collect();
        DirectedGraph { adj_list, vertices: vertices_rc, edges, edges_memoize: true }
    }

    pub fn bfs<'a>(&self, start: V, process: &dyn Fn(&V) -> ()) {
        let mut discovered = VecDeque::new();
        let mut queue = VecDeque::new();
        queue.push_front(start.clone());
        discovered.push_front(start.clone());
        process(&start);
        while !queue.is_empty() {
            let current_vertex = queue.pop_back().unwrap_or_else(|| panic!("Queue is empty!")).clone();
            /*
            print!("cv:{:?} -> ", current_vertex);
            self.neighbours(&current_vertex).iter().for_each(|x| print!(" nv:{:?}", x));
            println!();
            */
            for n in self.neighbours(&current_vertex) {
                if !discovered.contains(&n) {
                    process(&n);
                    discovered.push_front(n.clone());
                    queue.push_front(n.clone());
                }
            }
        }
    }


    pub fn dfs<'a>(&self, start: V, process: &dyn Fn(&V) -> ()) {
        let mut entry_time = HashMap::new();
        let mut exit_time = HashMap::new();
        let mut time = 0_usize;
        let mut discovered = VecDeque::new();
        let mut queue = VecDeque::new();
        queue.push_front(start.clone());
        discovered.push_front(start.clone());
        time+=1;
        entry_time.insert(start.clone(),time);
        process(&start);
        while !queue.is_empty() {
            let current_vertex = queue.front().unwrap_or_else(|| panic!("`Queue is empty!` This is absolutely unreachable err.")).clone();
            //Чтобы всегда в одном порядке возвращалось
            dbg!(&current_vertex);
            let mut iterator: Vec<V> = self.neighbours(&current_vertex).into_iter().filter(|x|!discovered.contains(x)).collect();

            iterator.sort();
            dbg!(&iterator);
            if iterator.is_empty() {
                time += 1;
                exit_time.insert(queue.pop_front().unwrap(), time);
            } else {
                for n in &iterator {
                    if !discovered.contains(&n) {
                        time += 1;
                        entry_time.insert(n.clone(), time);
                        discovered.push_front(n.clone());
                        queue.push_front(n.clone());
                        process(&n);
                    }
                }
            }
        }
        println!("Exit times");
        exit_time.iter().for_each(|x| println!("{:?}",x));
        println!("Entry times");
        entry_time.iter().for_each(|x| println!("{:?}",x));
    }

    /*
        pub fn dfs_rec<'a>(&self, start: V, time:i32,
                           entry_time:HashMap< V,i32>,
                           exit_time:HashMap<V,i32>,
                           discovered:VecDeque<V>,
                           queue:VecDeque<V>,
                           process: &dyn Fn(&V) -> ()) {

            let mut new_queue =  queue;
            new_queue.push_front()


            queue.push_front(start.clone());
            discovered.push_front(start.clone());
            process(&start);
            while !queue.is_empty() {
                let current_vertex = queue.pop_front().unwrap_or_else(|| panic!("Queue is empty!")).clone();
                time += 1;
                exit_time.insert(current_vertex.clone(), time);
                //Чтобы всегда в одном порядке возвращалось
                let mut iterator: Vec<V> = self.neighbours(&current_vertex).into_iter().collect();
                iterator.sort();
                for n in &iterator {
                    if !discovered.contains(&n) {
                        discovered.push_front(n.clone());
                        queue.push_front(n.clone());
                        process(&n);
                        time += 1;
                        entry_time.insert(n.to_owned(), time);
                    }
                }
            }
            println!("Exit times");
            exit_time.iter().for_each(|x| println!("{:?}",x));
            println!("Entry times");
            entry_time.iter().for_each(|x| println!("{:?}",x));
        }
    */
}

impl<V: Clone + Hash + Eq + Debug + PartialOrd + Ord> Graph<V> for DirectedGraph<V> {
    fn vertices(&self) -> &HashSet<V> {
        self.vertices.as_ref()
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

    fn add_edges(&mut self, a: V, b: V) -> Result<(), Error> {
        self.edges_memoize = false;
        Rc::get_mut(&mut self.vertices).unwrap().insert(a.clone());
        if let Some(v) = self.adj_list.get(&a) {
            self.adj_list.entry(a).and_modify(|x| x.push(b));
        } else {
            self.adj_list.insert(a, vec![b]);
        }
        Ok(())
    }

    fn neighbours(&self, vertex: &V) -> HashSet<V> {
        if let Some(v) = self.adj_list.get(vertex) {
            v.iter().map(|x| (*x).clone()).collect()
        } else {
            HashSet::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::{DirectedGraph, Graph};
    use std::collections::{HashMap, HashSet};

    fn sort_persistence<T: Clone + Ord + PartialOrd + PartialEq>(input: &Vec<T>) -> Vec<T> {
        let mut output = input.clone();
        output.sort();
        output
    }

    fn base_graph() -> DirectedGraph<i32> {
        let mut adj_list = HashMap::new();
        adj_list.insert(1, vec![4, 3]);
        adj_list.insert(4, vec![1]);
        adj_list.insert(3, vec![4]);
        let mut graph = DirectedGraph::new(adj_list);
        graph
    }

    #[test]
    fn vertices_test() {
        let mut graph = base_graph();
        let vertices_as_vec = graph.vertices().into_iter().map(|x| *x).collect::<Vec<i32>>();
        assert_eq!(sort_persistence(&vertices_as_vec), vec![1, 3, 4]);
    }

    #[test]
    fn edges_test() {
        let mut graph = base_graph();
        assert_eq!((graph.edges().len()), 4);
        graph.edges().iter().for_each(|x| println!("{:?}", x));
    }

    #[test]
    fn add_edges_test() {
        let mut graph = base_graph();
        assert_eq!((graph.edges().len()), 4);
        graph.edges().iter().for_each(|x| println!("{:?}", x));
        graph.add_edges(2, 1);
        assert_eq!((graph.edges().len()), 5);
        graph.edges().iter().for_each(|x| println!("{:?}", x));
        graph.add_edges(5, 4);
        assert_eq!((graph.edges().len()), 6);
        graph.edges().iter().for_each(|x| println!("{:?}", x));
    }

    #[test]
    fn neighbours_test() {
        let graph = base_graph();
        assert!(graph.neighbours(&1).contains(&3));
        assert!(graph.neighbours(&1).contains(&4));
        assert!(!graph.neighbours(&1).contains(&2));
        assert!(!graph.neighbours(&1).contains(&1));
    }

    #[test]
    fn bfs_test() {
        let mut graph = base_graph();
        graph.add_edges(1, 5);
        graph.add_edges(2, 3);
        graph.add_edges(2, 4);
        graph.add_edges(5, 3);
        graph.add_edges(5, 2);
        graph.add_edges(3, 6);
        graph.edges().iter().for_each(|x| println!("{:?}", x));
        graph.bfs(1, &|x| println!("{:?}", x));
    }

    #[test]
    fn dfs_test() {
        let mut graph = base_graph();
        graph.add_edges(1, 5);
        graph.add_edges(2, 3);
        graph.add_edges(2, 4);
        graph.add_edges(5, 3);
        graph.add_edges(5, 2);
        graph.add_edges(3, 6);

        graph.dfs(1, &|x| println!("{:?}", x));
        //graph.dfs(1, &|x| ());
    }
}