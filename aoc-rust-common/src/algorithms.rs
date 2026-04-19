use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::hash::Hash;

/// A generic implementation of Breadth-First Search (BFS).
///
/// # Arguments
/// * `start` - The initial state.
/// * `successors` - A function that returns a list of neighboring states for a given state.
/// * `is_goal` - A function that returns true if the goal state has been reached.
///
/// # Returns
/// * `Some(distance)` - The number of steps to the goal.
/// * `None` - If the goal is unreachable.
pub fn bfs<S, FN, FG>(start: S, mut successors: FN, mut is_goal: FG) -> Option<usize>
where
    S: Hash + Eq + Clone,
    FN: FnMut(&S) -> Vec<S>,
    FG: FnMut(&S) -> bool,
{
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();

    queue.push_back((start.clone(), 0));
    visited.insert(start, 0);

    while let Some((curr, dist)) = queue.pop_front() {
        if is_goal(&curr) {
            return Some(dist);
        }

        for next in successors(&curr) {
            if !visited.contains_key(&next) {
                visited.insert(next.clone(), dist + 1);
                queue.push_back((next, dist + 1));
            }
        }
    }

    None
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State<S> {
    cost: usize,
    position: S,
}

impl<S: Ord> Ord for State<S> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl<S: Ord> PartialOrd for State<S> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// A generic implementation of Dijkstra's algorithm.
///
/// # Arguments
/// * `start` - The initial state.
/// * `successors` - A function that returns a list of (neighboring state, edge cost) for a given state.
/// * `is_goal` - A function that returns true if the goal state has been reached.
///
/// # Returns
/// * `Some(cost)` - The minimum cost to reach the goal.
/// * `None` - If the goal is unreachable.
pub fn dijkstra<S, FN, FG>(start: S, mut successors: FN, mut is_goal: FG) -> Option<usize>
where
    S: Hash + Eq + Clone + Ord,
    FN: FnMut(&S) -> Vec<(S, usize)>,
    FG: FnMut(&S) -> bool,
{
    let mut heap = BinaryHeap::new();
    let mut dists = HashMap::new();

    dists.insert(start.clone(), 0);
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if is_goal(&position) {
            return Some(cost);
        }

        if let Some(&best) = dists.get(&position) {
            if cost > best {
                continue;
            }
        }

        for (next, edge_cost) in successors(&position) {
            let next_cost = cost + edge_cost;
            let current_best = dists.get(&next).cloned().unwrap_or(usize::MAX);

            if next_cost < current_best {
                dists.insert(next.clone(), next_cost);
                heap.push(State {
                    cost: next_cost,
                    position: next,
                });
            }
        }
    }

    None
}
