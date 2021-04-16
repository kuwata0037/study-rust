use std::borrow::Borrow;
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

pub trait AdjacentNodes {
    type Node;

    fn adjacent_nodes(&self, v: &Self::Node) -> Vec<Self::Node>;
}

pub struct Crawler<'a, G: AdjacentNodes> {
    graph: &'a G,
    visit: VecDeque<G::Node>,
    visited: HashSet<G::Node>,
}

impl<'a, G> Crawler<'a, G>
where
    G: AdjacentNodes,
    G::Node: Clone + Hash + Eq + Borrow<G::Node>,
{
    pub fn new(graph: &'a G, start: G::Node) -> Self {
        let mut visit = VecDeque::new();
        let visited = HashSet::new();

        visit.push_back(start);

        Self {
            graph,
            visit,
            visited,
        }
    }
}

impl<'a, G> Iterator for Crawler<'a, G>
where
    G: AdjacentNodes,
    G::Node: Clone + Hash + Eq + Borrow<G::Node>,
{
    type Item = G::Node;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(v) = self.visit.pop_front() {
            if self.visited.contains(&v) {
                continue;
            }

            let adj = self.graph.adjacent_nodes(&v);
            for u in adj.into_iter() {
                if !self.visited.contains(&u) {
                    self.visit.push_back(u);
                }
            }

            self.visited.insert(v.clone());

            return Some(v);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    struct AdjVec(Vec<Vec<usize>>);

    impl AdjacentNodes for AdjVec {
        type Node = usize;

        fn adjacent_nodes(&self, v: &Self::Node) -> Vec<Self::Node> {
            self.0.get(*v).cloned().unwrap_or_default()
        }
    }

    #[rstest(input, expected,
        case(AdjVec(vec![vec![1, 2], vec![0, 3], vec![3], vec![2, 0]]), vec![0, 1, 2, 3]),
        case(AdjVec(vec![vec![1], vec![0, 2, 4], vec![0, 3], vec![0], vec![0]]), vec![0, 1, 2, 4, 3])
    )]
    fn test_bfs(input: AdjVec, expected: Vec<usize>) {
        let crawler = Crawler::new(&input, 0);
        let nodes = crawler.collect::<Vec<usize>>();

        assert_eq!(nodes, expected);
    }
}
