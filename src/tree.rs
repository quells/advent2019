
#[derive(Clone, Debug)]
struct Node {
    id: String,
    parent: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct Tree {
    nodes: Vec<Node>,
    depth: Vec<usize>,
}

impl Tree {
    pub fn new(root: &str) -> Self {
        let root = Node { id: root.to_string(), parent: None };
        Self { nodes: vec![root], depth: vec![0] }
    }

    pub fn ingest(&mut self, children: &[(&str, &str)]) {
        let mut queue = std::collections::VecDeque::new();
        for (id, p) in children {
            queue.push_back((*id, *p));
        }
        loop {
            let (id, p) = match queue.pop_front() {
                Some(next) => next,
                None => break,
            };
            match self.insert(id, p) {
                Ok(_) => (),
                Err(_) => {
                    queue.push_back((id, p));
                },
            }
        }
    }

    fn find(&self, id: &str) -> Option<usize> {
        self.nodes.clone().into_iter()
            .enumerate()
            .filter(|(_, n)| n.id == id)
            .next()
            .map(|(id, _)| id)
    }

    pub fn insert(&mut self, id: &str, parent: &str) -> Result<(), &'static str> {
        match self.find(parent) {
            Some(parent) => {
                self.nodes.push(Node { id: id.to_string(), parent: Some(parent) });
                let depths = self.depth.clone();
                let p_depth = depths.get(parent).unwrap();
                self.depth.push(p_depth + 1);
                Ok(())
            },
            None => {
                Err("could not find parent")
            }
        }
    }

    pub fn depth_at(&self, idx: usize) -> usize {
        self.depth[idx]
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn distance(&self, a_id: &str, b_id: &str) -> usize {
        let a_idx = self.find(a_id).unwrap();
        let b_idx = self.find(b_id).unwrap();
        let a_depth = self.depth[a_idx];
        let b_depth = self.depth[b_idx];
        let (mut common_idx, delta, lower) = if a_depth < b_depth {
            (a_idx, b_depth - a_depth, b_depth)
        } else {
            (b_idx, a_depth - b_depth, a_depth)
        };
        for _ in 0..delta {
            common_idx = match dbg!(&self.nodes[common_idx]).parent {
                Some(n) => n,
                None => return a_depth + b_depth,
            };
        }
        lower - self.depth[common_idx] + delta
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orbit() {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
K)L
J)K";
        let mut system = Tree::new(&"COM");
        let children: Vec<(&str, &str)> = input.split("\n")
            .map(|line| {
                let obj: Vec<&str> = line.split(")").collect();
                (obj[1], obj[0])
            })
            .collect();
        system.ingest(&children);

        let orbit_count = (0..system.nodes.len())
            .map(|idx| system.depth_at(idx))
            .fold(0, |acc, x| acc + x);
        assert_eq!(orbit_count, 42);
    }

    #[test]
    fn transfers() {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
        let mut system = Tree::new(&"COM");
        let children: Vec<(&str, &str)> = input.split("\n")
            .map(|line| {
                let obj: Vec<&str> = line.split(")").collect();
                (obj[1], obj[0])
            })
            .collect();
        system.ingest(&children);

        let distance = system.distance(&"YOU", &"SAN");
        assert_eq!(distance, 6);

        // not quite right for part b
    }
}
