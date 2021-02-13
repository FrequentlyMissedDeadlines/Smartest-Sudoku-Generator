pub struct Tree<T> {
    pub children: Vec<Tree<T>>,
    pub node: T

}

impl<T> Tree<T> {
    pub fn new(node: T) -> Tree<T> { 
        Tree {
            children: Vec::new(),
            node: node
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let tested = Tree::new(10);
        assert_eq!(tested.node, 10);
        assert_eq!(tested.children.len(), 0);
    }
}