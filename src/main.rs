#[derive(Debug)]
struct Node {
    data: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(data: u32) -> Self {
        Node {
            data,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
struct BinaryTree {
    root: Option<Box<Node>>,
}

impl BinaryTree {
    fn new() -> Self {
        BinaryTree { root: None }
    }

    // insert
    fn insert(&mut self, data: u32) {
        self.root = Self::insert_recursive(self.root.take(), data);
    }

    fn insert_recursive(root: Option<Box<Node>>, data: u32) -> Option<Box<Node>> {
        match root {
            Some(mut node) => {
                if data <= node.data {
                    node.left = Self::insert_recursive(node.left.take(), data);
                } else {
                    node.right = Self::insert_recursive(node.right.take(), data);
                }
                Some(node)
            }

            None => Some(Box::new(Node::new(data))),
        }
    }

    // pre order traversal and printing of binary tree
    fn print_pre_order(&self) {
        Self::pre_order_traversal(self.root.as_ref());
    }

    fn pre_order_traversal(root: Option<&Box<Node>>) {
        if let Some(node) = root {
            println!("{}", node.data);
            Self::pre_order_traversal(node.left.as_ref());
            Self::pre_order_traversal(node.right.as_ref());
        }
    }

    // in order traversal and printing of binary tree
    fn print_in_order(&self) {
        Self::in_order_traversal(self.root.as_ref());
    }

    fn in_order_traversal(root: Option<&Box<Node>>) {
        if let Some(node) = root {
            Self::in_order_traversal(node.left.as_ref());
            println!("{}", node.data);
            Self::in_order_traversal(node.right.as_ref());
        }
    }

    // post order traversal and printing of binary tree
    fn print_post_order(&self) {
        Self::post_order_traversal(self.root.as_ref());
    }

    fn post_order_traversal(root: Option<&Box<Node>>) {
        if let Some(node) = root {
            Self::post_order_traversal(node.left.as_ref());
            Self::post_order_traversal(node.right.as_ref());
            println!("{}", node.data);
        }
    }
}

fn main() {
    let mut tree = BinaryTree::new();

    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(2);
    tree.insert(4);
    tree.insert(6);
    tree.insert(8);

    println!("pre order traversal: ");
    tree.print_pre_order();

    println!("in order traversal: ");
    tree.print_in_order();

    println!("post order traversal: ");
    tree.print_post_order();
}
