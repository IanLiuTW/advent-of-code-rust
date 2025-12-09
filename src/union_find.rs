#[derive(Debug, Clone)]
pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    groups: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
            groups: n,
        }
    }

    pub fn find(&mut self, mut i: usize) -> usize {
        let mut root = i;
        while root != self.parent[root] {
            root = self.parent[root];
        }
        while i != root {
            let next = self.parent[i];
            self.parent[i] = root;
            i = next;
        }
        root
    }

    pub fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i == root_j {
            return false;
        }

        if self.size[root_i] < self.size[root_j] {
            self.parent[root_i] = root_j;
            self.size[root_j] += self.size[root_i];
        } else {
            self.parent[root_j] = root_i;
            self.size[root_i] += self.size[root_j];
        }

        self.groups -= 1;
        true
    }

    pub fn get_component_sizes(&self) -> impl Iterator<Item = usize> + '_ {
        self.parent.iter().enumerate().filter_map(
            move |(i, &p)| {
                if i == p { Some(self.size[i]) } else { None }
            },
        )
    }

    pub fn get_groups(&self) -> usize {
        self.groups
    }
}
