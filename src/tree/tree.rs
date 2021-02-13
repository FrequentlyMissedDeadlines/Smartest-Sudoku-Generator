#[derive(Debug)]
pub struct Tree<T> {
    children: Vec<Tree<T>>,
    node: T

}

impl<T> Tree<T> {
    pub fn new(node: T) -> Tree<T> { 
        Tree {
            children: Vec::new(),
            node: node
        }
    }
}