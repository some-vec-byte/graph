use std::fmt::Debug;

/// A graph structure with support for appending, traversing and removing nodes.
pub struct Graph<D, K: PartialEq + Clone + Debug> {
    nodes: Vec<Node<D, K>>,
}
impl<D, K> Graph<D, K>
where
    K: PartialEq + Clone + Debug,
{
    /// Returns a empty graph.
    pub fn new() -> Self {
        Graph { nodes: vec![] }
    }
    /// Appends a node to graph. The first node shouldn't have a fathers key. All others need one. Will panic if the first node has a fathers key, if one except the first has none or father is not found.
    /// # Arguments
    ///
    /// * `node` - The node to append.
    ///
    pub fn append_node(&mut self, node: Node<D, K>) {
        let mut added_key = false;
        if !self.nodes.is_empty() {
            for current_node in self.nodes.iter_mut() {
                if let Some(father_key) = &node.father_key {
                    if &current_node.key == father_key {
                        current_node.children.push(node.key.clone());
                        added_key = true;
                        break;
                    }
                }
            }
        } else if node.father_key.is_none() && !self.nodes.is_empty() {
            panic!("Every other node except the first needs a father key.");
        } else if node.father_key.is_some() && self.nodes.is_empty() {
            panic!("First node cant have a fathers key.");
        }

        match (self.nodes.is_empty(), added_key) {
            (true, _) => {}
            (_, true) => {}
            (false, false) => {
                panic!("Needs father and no father found");
            }
        }
        self.nodes.push(node);
    }
    /// Travels the graph with given path and returns a node if one is found.
    ///
    /// # Arguments
    ///
    /// * `route` - A slice of keys. Will return the last key of the route if found.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph::{Graph,Node};
    /// let first_node = Node::new("Hallo, willst du etwas Essen gehen, oder einen Sitzplatz buchen?","Start",None);
    /// let second_node = Node::new("Ok, was willst du essen? Pizza oder Pasta?","Essen",Some("Start"));
    /// let third_node = Node::new("Ok, willst du am Fenster oder am Gang sitzen?","Sitzplatz",Some("Start"));
    /// let fourth_node = Node::new("Ok, dann einen Sitzlatz am Gang. Bis dann!","Gang",Some("Sitzplatz"));
    ///
    /// let nodes = vec![first_node,second_node,third_node,fourth_node];
    /// let mut graph = Graph::new();
    ///
    /// for node in nodes.into_iter(){
    /// graph.append_node(node);
    /// }
    /// assert_eq!(graph.len(),4);
    /// let node = graph.travel_to_node(&["Sitzplatz", "Gang"]);
    /// assert_eq!(node.unwrap().data,"Ok, dann einen Sitzlatz am Gang. Bis dann!");
    /// ```
    pub fn travel_to_node(&self, route: &[K]) -> Option<&Node<D, K>> {
        let mut start_node = None;

        for node in self.nodes.iter() {
            if node.father_key.is_none() {
                start_node = Some(node);
                break;
            }
        }

        for key in route {
            let mut found_node = false;

            if let Some(current_node) = start_node {
                if current_node.has_child(key) {
                    if let Some(new_start_node) = find_node_with_key(&self.nodes, key) {
                        start_node = Some(new_start_node);
                        found_node = true;
                    } else {
                        println!("Couldnt set node to current");
                    }
                }
                if !found_node {
                    return None;
                }
            }
        }
        start_node
    }
    /// Returns the amount of nodes in the graph.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
    /// Returns if the graph is empty.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
    /// Removes the node with given key and all attached nodes.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the node to remove.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph::{Graph,Node};
    /// let first_node = Node::new("Hallo, willst du etwas Essen gehen, oder einen Sitzplatz buchen?","Start",None);
    /// let second_node = Node::new("Ok, was willst du essen? Pizza oder Pasta?","Essen",Some("Start"));
    /// let third_node = Node::new("Ok, willst du am Fenster oder am Gang sitzen?","Sitzplatz",Some("Start"));
    /// let fourth_node = Node::new("Ok, dann einen Sitzlatz am Gang. Bis dann!","Gang",Some("Sitzplatz"));
    ///
    /// let nodes = vec![first_node,second_node,third_node,fourth_node];
    /// let mut graph = Graph::new();
    ///
    /// for node in nodes.into_iter(){
    /// graph.append_node(node);
    /// }
    /// assert_eq!(graph.len(),4);
    /// graph.remove_node_with_childs("Sitzplatz");
    /// assert_eq!(graph.len(),2);
    /// ```
    pub fn remove_node_with_childs(&mut self, key: K) {
        let mut all_nodes_to_remove = find_all_child_nodes(&self.nodes, &key);
        all_nodes_to_remove.push(key);
        for node in all_nodes_to_remove {
            delete_node(&mut self.nodes, &node);
        }
    }
}
/// Deletes given node from given Vector.
///
/// # Arguments
///
/// * `key` - Key of the node to delete
/// * `nodes` - The vector where the node should be deleted from.
///
fn delete_node<'a, D, K: PartialEq + Clone + Debug>(nodes: &'a mut Vec<Node<D, K>>, key: &'a K) {
    if let Some(index) = nodes.iter().position(|x| &x.key == key) {
        nodes.remove(index);
    }
}
/// Returns all attached keys of given key.
///
/// # Arguments
///
/// * `key` - Key which childs should be found
///
fn find_all_child_nodes<'a, D, K: PartialEq + Clone + Debug>(
    nodes: &'a Vec<Node<D, K>>,
    key: &'a K,
) -> Vec<K> {
    let mut all_nodes = vec![];
    if let Some(node_to_delete) = find_node_with_key(nodes, key) {
        for child in node_to_delete.children.iter() {
            let mut found_childs = find_all_child_nodes(nodes, child);

            all_nodes.append(&mut found_childs);

            all_nodes.push(child.clone());
        }
    }
    all_nodes
}
fn find_node_with_key<'a, D, K: PartialEq + Clone + Debug>(
    nodes: &'a [Node<D, K>],
    key: &K,
) -> Option<&'a Node<D, K>> {
    for node in nodes.iter() {
        if &node.key == key {
            return Some(node);
        }
    }
    None
}
/// A Node for the graph structure.
pub struct Node<D, K: PartialEq + Clone + Debug> {
    /// The data the node holds.
    pub data: D,
    /// All the keys of the children nodes.
    children: Vec<K>,
    /// The key of the father of the node.
    father_key: Option<K>,
    /// Key
    key: K,
}
impl<D, K> Node<D, K>
where
    K: PartialEq + Clone + Debug,
{
    /// Returns a new Node without children. Fathers key has to be empty only for the first node of the graph. All other nodes need a valid fathers key.
    pub fn new<T: Into<Option<K>>>(data: D, key: K, father_key: T) -> Self {
        Node {
            data,
            key,
            father_key: father_key.into(),
            children: vec![],
        }
    }
    /// Returns true if the node holds a childs key of given type.
    fn has_child(&self, key: &K) -> bool {
        for child_key in self.children.iter() {
            if child_key == key {
                return true;
            }
        }
        false
    }
}
impl<D, K> Default for Graph<D, K>
where
    K: PartialEq + Clone + Debug,
{
    fn default() -> Self {
        Graph::new()
    }
}
#[cfg(test)]
mod tests {
    use crate::{Graph, Node};

    #[test]
    fn it_works() {
        let first_node = Node::new(
            "Hallo, willst du etwas Essen gehen, oder einen Sitzplatz buchen?",
            "Start",
            None,
        );
        let second_node = Node::new(
            "Ok, was willst du essen? Pizza oder Pasta?",
            "Essen",
            "Start",
        );
        let third_node = Node::new(
            "Ok, willst du am Fenster oder am Gang sitzen?",
            "Sitzplatz",
            "Start",
        );
        let fourth_node = Node::new(
            "Ok, dann einen Sitzlatz am Gang. Bis dann!",
            "Gang",
            "Sitzplatz",
        );

        let nodes = vec![first_node, second_node, third_node, fourth_node];
        let mut graph = Graph::new();

        for node in nodes.into_iter() {
            graph.append_node(node);
        }
        assert_eq!(graph.len(), 4);
        let node = graph.travel_to_node(&["Sitzplatz", "Gang"]);
        assert_eq!(
            node.unwrap().data,
            "Ok, dann einen Sitzlatz am Gang. Bis dann!"
        );

        graph.remove_node_with_childs("Sitzplatz");

        assert_eq!(graph.len(), 2);
    }
}
