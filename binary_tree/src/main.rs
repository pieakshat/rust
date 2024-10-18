struct Node<T> {
    data: T, 
    left: Option<Box<Node<T>>>, 
    right: Option<Box<Node<T>>>, 
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node { 
            data, 
        left: None, 
        right: None, 
    }
    }

    fn add_left_child(&mut self, left_child_data: T) {
        self.left = Some(Box::new(Node::new(left_child_data))); 
    } 

    fn add_right_child(&mut self, right_child_data: T) {
        self.right = Some(Box::new(Node::new(right_child_data))); 
    } 
}

fn main() {
let mut root = Node::new(10); 
root.add_left_child(5); 
root.add_right_child(7); 
println!("root node data {}", root.data); 
println!("left child data {}", root.left.as_ref().unwrap().data); 

}