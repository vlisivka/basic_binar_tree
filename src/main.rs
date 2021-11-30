// Author: Volodymyr M. Lisivka <vlisivka@gmail.com>

#[derive(Debug)]
struct Node<K, V> {
    key: K,
    value: V,

    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
    parent: Option<*mut Node<K, V>>,
}

impl<K, V> Node<K, V> {
    fn new_root(key: K, value: V) -> Self {
        Self {
            key,
            value,
            left: None,
            right: None,
            parent: None,
        }
    }

    fn new_leaf(key: K, value: V, parent: *mut Node<K, V>) -> Self {
        Self {
            key,
            value,
            left: None,
            right: None,
            parent: Some(parent),
        }
    }
}

#[derive(Debug)]
struct Tree<K, V> {
    root: Option<Box<Node<K, V>>>,
}

impl<K, V> Tree<K, V>
where
    K: std::cmp::PartialEq + std::cmp::PartialOrd,
{
    pub fn new() -> Self {
        Self { root: None }
    }

    /**
     * Insert key-value pair into tree.
     */
    pub fn insert(&mut self, key: K, value: V) -> bool {
        match self.root {
            Some(ref mut node) => Self::insert_at(node, key, value),
            None => {
                self.root = Some(Box::new(Node::new_root(key, value)));
                true
            }
        }
    }

    fn insert_at(current_node: &mut Node<K, V>, key: K, value: V) -> bool {
        match (
            current_node.left.as_deref_mut(),
            current_node.right.as_deref_mut(),
        ) {
            _ if key == current_node.key => false,

            (Some(left_node), _) if key < current_node.key => {
                Self::insert_at(left_node, key, value)
            }

            (None, _) if key < current_node.key => {
                let new_node = Node::new_leaf(key, value, current_node);
                current_node.left = Some(Box::new(new_node));
                true
            }

            (_, Some(right_node)) => Self::insert_at(right_node, key, value),

            (_, None) => {
                let new_node = Node::new_leaf(key, value, current_node);
                current_node.right = Some(Box::new(new_node));
                true
            }
        }
    }

    /**
     * Find a node in the tree by the key.
     */
    pub fn find(&self, key: K) -> Option<&Node<K, V>> {
        match self.root {
            None => None,
            Some(ref node) => Self::find_at(node, key),
        }
    }

    fn find_at(current_node: &Node<K, V>, key: K) -> Option<&Node<K, V>> {
        match (current_node.left.as_ref(), current_node.right.as_ref()) {
            _ if current_node.key == key => Some(current_node),

            (Some(left_node), _) if current_node.key >= key => Self::find_at(left_node, key),
            (None, _) if current_node.key >= key => None,

            (_, Some(right_node)) => Self::find_at(right_node, key),
            (_, None) => None,
        }
    }

    /**
     * Remove node with subtrees from tree and return it.
     * Parent pointer still points to original node,
     * it will be valid until next update to this tree.
     */
    fn detach(&mut self, key: K) -> Option<Box<Node<K, V>>> {
        match &mut self.root {
            None => None,

            Some(root_node) if root_node.key == key => self.root.take(),

            Some(root_node) => Self::detach_at(root_node, key),
        }
    }

    fn detach_at(current_node: &mut Node<K, V>, key: K) -> Option<Box<Node<K, V>>> {
        match (
            current_node.left.as_deref_mut(),
            current_node.right.as_deref_mut(),
        ) {
            (Some(left), _) if left.key == key => current_node.left.take(),

            (_, Some(right)) if right.key == key => current_node.right.take(),

            (Some(left_node), _) if current_node.key >= key => Self::detach_at(left_node, key),
            (None, _) if current_node.key >= key => None,

            (_, Some(right_node)) => Self::detach_at(right_node, key),
            (_, None) => None,
        }
    }
}

pub fn main() {
    let mut tree = Tree::new();
    let key = [5, 3, 65, 123, 6, 11, 3, 1, 5, 42];

    for i in key {
        tree.insert(i, i);
    }

    println!("Original tree: {:?}", tree);

    println!("Node 6: {:?}", tree.find(6));

    let detached_node = tree.detach(6);
    println!("Original tree after detaching of node 6: {:?}", tree);
    println!("Node 6 detached: {:?}", detached_node);
}
