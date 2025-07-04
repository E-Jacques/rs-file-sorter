use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType {
    File,
    Directory,
    Other,
}

impl NodeType {
    fn guess(name: &str, is_leaf: bool) -> NodeType {
        if is_leaf {
            if name.contains('.') {
                NodeType::File
            } else {
                NodeType::Other
            }
        } else {
            NodeType::Directory
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub node_type: NodeType,
    pub children: HashMap<String, Node>,
}

impl From<&Path> for NodeType {
    fn from(path: &Path) -> Self {
        match std::fs::symlink_metadata(path) {
            Ok(meta) => {
                if meta.file_type().is_file() {
                    NodeType::File
                } else if meta.file_type().is_dir() {
                    NodeType::Directory
                } else {
                    NodeType::Other
                }
            }
            Err(_) => NodeType::Other,
        }
    }
}

impl From<Vec<PathBuf>> for Node {
    fn from(value: Vec<PathBuf>) -> Self {
        let mut root_node = Node::new("");

        for pathbuf in value {
            root_node.insert(&pathbuf.as_path());
        }

        root_node
    }
}

impl Node {
    pub fn new(name: &str) -> Self {
        Node {
            name: name.to_string(),
            node_type: NodeType::guess(name, true),
            children: HashMap::new(),
        }
    }

    pub fn insert(&mut self, path: &Path) {
        let mut current = self;

        for component in path.components() {
            use std::path::Component;
            if let Component::Normal(os_str) = component {
                let name = os_str.to_str().unwrap();
                current.node_type = NodeType::guess(name, false);
                current = current
                    .children
                    .entry(name.to_string())
                    .or_insert(Node::new(name));
            }
        }
    }

    pub fn canonicalize(&self) -> Node {
        let mut name = self.name.clone();
        let mut current = self;

        while current.children.len() == 1 {
            let (child_name, child) = current.children.iter().next().unwrap();

            if child.node_type == current.node_type {
                name = format!("{}/{}", name, child_name);
                current = child;
            } else {
                break;
            }
        }

        let children = current
            .children
            .iter()
            .map(|(_, v)| {
                let canocalized_node = v.canonicalize();
                (canocalized_node.name.clone(), canocalized_node)
            })
            .collect();

        Node {
            name,
            node_type: self.node_type.clone(),
            children,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::Node;

    #[test]
    fn test_from_vec_pathbuf_to_node() {
        let value = vec![
            PathBuf::from("/test/dir-1/file.txt"),
            PathBuf::from("/test/dir-1/file_2.txt"),
            PathBuf::from("/etc/dir-1/file.txt"),
            PathBuf::from("/test/dir-2/file.txt"),
        ];
        let node = Node::from(value);

        assert_eq!(node.name, "");
        assert_eq!(node.children.len(), 2);
        let etc_dir = node.children.get("etc").expect("Missing 'etc' directory");
        assert_eq!(etc_dir.name, "etc");
        assert_eq!(etc_dir.children.len(), 1);

        let etc_dir_1 = etc_dir
            .children
            .get("dir-1")
            .expect("Missing 'dir-1' in 'etc'");
        assert_eq!(etc_dir_1.name, "dir-1");
        assert_eq!(etc_dir_1.children.len(), 1);
        assert!(etc_dir_1.children.contains_key("file.txt"));

        let test_dir = node.children.get("test").expect("Missing 'test' directory");
        assert_eq!(test_dir.name, "test");
        assert_eq!(test_dir.children.len(), 2);

        let test_dir_1 = test_dir
            .children
            .get("dir-1")
            .expect("Missing 'dir-1' in 'test'");
        assert_eq!(test_dir_1.name, "dir-1");
        assert_eq!(test_dir_1.children.len(), 2);
        assert!(test_dir_1.children.contains_key("file.txt"));
        assert!(test_dir_1.children.contains_key("file_2.txt"));

        let test_dir_2 = test_dir
            .children
            .get("dir-2")
            .expect("Missing 'dir-2' in 'test'");
        assert_eq!(test_dir_2.name, "dir-2");
        assert_eq!(test_dir_2.children.len(), 1);
        assert!(test_dir_2.children.contains_key("file.txt"));
    }

    #[test]
    fn test_canonicalize() {
        let mut root = Node::new("");

        // One chain: /etc/dir-1/file.txt
        root.insert(Path::new("/etc/dir-1/file.txt"));

        // One branching: /test/dir-1/file1.txt and file2.txt
        root.insert(Path::new("/test/dir-1/file1.txt"));
        root.insert(Path::new("/test/dir-1/file2.txt"));

        let canonical = root.canonicalize();

        assert!(canonical.children.contains_key("etc/dir-1"));

        let dir2 = canonical.children.get("etc/dir-1").unwrap();
        assert!(dir2.children.contains_key("file.txt"));

        assert!(canonical.children.contains_key("test/dir-1"));

        let dir1 = canonical.children.get("test/dir-1").unwrap();
        assert!(dir1.children.contains_key("file1.txt"));
        assert!(dir1.children.contains_key("file2.txt"));
    }
}
