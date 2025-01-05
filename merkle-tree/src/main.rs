use sha2::{Digest, Sha256};

struct Node {
    hash: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(hash: String) -> Node {
        Node {
            hash,
            left: None,
            right: None,
        }
    }

    fn add_left_node(&mut self, node: Node) {
        if self.left.is_some() {
            panic!("left node already exists");
        }
        self.left = Some(Box::new(node));
    }

    fn add_right_node(&mut self, node: Node) {
        if self.right.is_some() {
            panic!("right node already exists");
        }
        self.right = Some(Box::new(node));
    }
}

fn generate_hash<T: AsRef<[u8]>>(input: T) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn concat_hashes(left: &str, right: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(left);
    hasher.update(right);
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn take_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read the input");
    input.trim().to_string()
}

fn main() {
    println!("Enter data for the four leaf nodes:");
    let data1 = take_input();
    let data2 = take_input();
    let data3 = take_input();
    let data4 = take_input();

    // Leaf nodes
    let mut node1 = Node::new(generate_hash(data1.as_bytes()));
    let mut node2 = Node::new(generate_hash(data2.as_bytes()));
    let mut node3 = Node::new(generate_hash(data3.as_bytes()));
    let mut node4 = Node::new(generate_hash(data4.as_bytes()));

    // Intermediate nodes
    let mut node5 = Node::new(concat_hashes(&node1.hash, &node2.hash));
    let mut node6 = Node::new(concat_hashes(&node3.hash, &node4.hash));

    // Root node
    let mut root_node = Node::new(concat_hashes(&node5.hash, &node6.hash));

    // Build the tree
    node5.add_left_node(node1);
    node5.add_right_node(node2);
    node6.add_left_node(node3);
    node6.add_right_node(node4);
    root_node.add_left_node(node5);
    root_node.add_right_node(node6);

    println!("Root hash: {}", root_node.hash);
}
