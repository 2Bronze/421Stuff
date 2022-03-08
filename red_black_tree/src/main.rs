use std::cell::RefCell;
use std::rc::Rc;
use std::io::{self, BufRead, Write};

#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}

type Tree<T> = Rc<RefCell<TreeNode<T>>>;
type RedBlackTree<T> = Option<Tree<T>>;
#[derive(Debug)]
struct TreeNode<T> {
    pub color: NodeColor,
    pub key: T,
    pub parent: RedBlackTree<T>,
    left: RedBlackTree<T>,
    right: RedBlackTree<T>,
}
#[derive(Debug)]
struct RBTree<T> {
    pub root: RedBlackTree<T>,
    count: usize,
    height: usize,
}

impl<T: std::cmp::PartialOrd + Copy + std::fmt::Display + std::fmt::Debug> RBTree<T> {
    pub fn new() -> Self {
        RBTree{root: None, count: 0, height: 0}
    }

    pub fn insert(&mut self, key: T) {
        match &self.root {
            Some(rc_node) => {
                RBTree::node_insert(rc_node.clone(), key);
                self.count += 1;
            },
            None => {
                self.root = Some(Rc::new(RefCell::new(TreeNode{color: NodeColor::Black, key: key, parent: None, left: None, right: None})));
                self.count = 1;
            },
        }
        self.height = RBTree::tree_height(self.root.as_ref().cloned().unwrap().clone());
    }

    fn node_insert(rc_node: Tree<T>, key: T) {
        let mut node = rc_node.as_ref().borrow_mut();
        let new_node = TreeNode{color: NodeColor::Red, key:key, parent: Some(rc_node.clone()), left: None, right: None};
        let rc_new_node = Rc::new(RefCell::new(new_node));
        if key < node.key {
            match &node.left {
                Some(rc_node) => RBTree::node_insert(rc_node.clone(), key),
                None => {
                    node.left = Some(rc_new_node.clone());
                    RBTree::recolor(rc_new_node.clone());
                },
            }
        } else {
            match &node.right {
                Some(rc_node) => RBTree::node_insert(rc_node.clone(), key),
                None => {
                    node.right = Some(rc_new_node.clone());
                    RBTree::recolor(rc_new_node.clone());
                },
            }
        }
    }

    fn recolor(rc_node: Tree<T>) {
        // let current = rc_node.as_ref().borrow_mut();
        // let mut option_parent = current.parent.take();
        // let parent = option_parent.to_owned().unwrap().as_ref().borrow_mut();
    }

    pub fn find(&mut self, key: T) -> bool {
        match &self.root {
            Some(rc_node) => RBTree::node_find(rc_node.clone(), key),
            None => false,
        }
    }

    fn node_find(rc_node: Tree<T>, key: T) -> bool {
        let node = rc_node.as_ref().borrow();
        if node.key == key {
            return true
        } else if key < node.key {
            match &node.left {
                Some(rc_node) => RBTree::node_find(rc_node.clone(), key),
                None => false,
            }
        } else {
            match &node.right {
                Some(rc_node) => RBTree::node_find(rc_node.clone(), key),
                None => false,
            }
        }
    }

    pub fn delete(&mut self, key: T) {
        match &self.root {
            Some(rc_node) => {
                if RBTree::node_find(rc_node.clone(), key) {
                    let new_root = RBTree::node_delete(rc_node.clone(), key);
                    self.set_root(new_root);
                    self.count -= 1;
                }
            },
            None => {},
        }
        self.height = RBTree::tree_height(self.root.as_ref().cloned().unwrap().clone());
    }

    fn node_delete(rc_node: Tree<T>, key: T) -> Option<Tree<T>> {
        let mut node = rc_node.as_ref().borrow_mut();

        if node.key == key {
            if node.left.is_none() && node.right.is_none() {
                return None;
            } else if node.left.is_none() {
                let option_right_node = node.right.take();
                let right_node = option_right_node.as_ref().cloned().unwrap();
                right_node.as_ref().borrow_mut().parent = node.parent.take();
                return option_right_node;
            } else if node.right.is_none() {
                let option_left_node = node.left.take();
                let left_node = option_left_node.as_ref().cloned().unwrap();
                left_node.as_ref().borrow_mut().parent = node.parent.take();
                return option_left_node;
            } else {
                let option_left_node = node.left.as_ref().cloned();
                let left_node = option_left_node.as_ref().cloned().unwrap();
                let right_node = node.right.as_ref().cloned().unwrap();
                let mut option_leftmost_node = node.right.take();
                let mut prev_option = option_leftmost_node.as_ref().cloned();
                while let Some(ref rc_node) = option_leftmost_node {
                    let option_new_node = rc_node.as_ref().borrow_mut().left.as_ref().cloned();
                    if option_new_node.is_none() {
                        let leftmost_node = option_leftmost_node.as_ref().cloned().unwrap();
                        //Set parent to new node
                        leftmost_node.as_ref().borrow_mut().parent = node.parent.take();
                        //Set left node
                        leftmost_node.as_ref().borrow_mut().left = node.left.take();
                        //Set right node
                        if !Rc::ptr_eq(&leftmost_node, &right_node) {
                            prev_option.as_ref().cloned().unwrap().as_ref().borrow_mut().left.take();
                            leftmost_node.as_ref().borrow_mut().right = Some(right_node);
                        }

                        left_node.as_ref().borrow_mut().parent.insert(leftmost_node);
                        return option_leftmost_node;
                    }
                    prev_option = option_leftmost_node;
                    option_leftmost_node = option_new_node;
                }
            }
        } else if key < node.key {
            match &node.left {
                Some(rc_node) => node.left = RBTree::node_delete(rc_node.clone(), key),
                None => {},
            }
        } else {
            match &node.right {
                Some(rc_node) => node.right = RBTree::node_delete(rc_node.clone(), key),
                None => {},
            }
        }
        return Some(rc_node.clone());
    }


    fn rotate_right(rc_node: Tree<T>) {
        let mut node = rc_node.as_ref().borrow_mut();
        let rc_left_node;
        match &node.left {
            Some(rc_node) => rc_left_node = rc_node.clone(),
            None => panic!("No left node"),
        }
        let mut left_node = rc_left_node.as_ref().borrow_mut();

        //Swap parents
        left_node.parent = node.parent.take();
        node.parent.insert(rc_left_node.clone());
        //Move left_right_node to left of current node
        match left_node.right.take() {
            Some(new_rc_node) => {
                let mut new_node = new_rc_node.as_ref().borrow_mut();
                new_node.parent.replace(rc_node.clone());
                node.left.replace(new_rc_node.clone());
            },
            None => node.left = None,
        }

        //Set new top nodes' right node
        left_node.right.replace(rc_node.clone());
    }

    fn rotate_left(rc_node: Tree<T>) {
        let mut node = rc_node.as_ref().borrow_mut();
        let rc_right_node;
        match &node.right {
            Some(rc_node) => rc_right_node = rc_node.clone(),
            None => panic!("No right node"),
        }
        let mut right_node = rc_right_node.as_ref().borrow_mut();

        //Swap parents
        right_node.parent = node.parent.take();
        node.parent.insert(rc_right_node.clone());
        //Move right_left_node to right of current node
        match right_node.left.take() {
            Some(new_rc_node) => {
                let mut new_node = new_rc_node.as_ref().borrow_mut();
                new_node.parent.replace(rc_node.clone());
                node.right.replace(new_rc_node.clone());
            },
            None => node.right = None,
        }

        //Set new top nodes' left node
        right_node.left.replace(rc_node.clone());
    }

    fn fix_root(tree: &mut Self) {
        let mut parent = tree.root.as_ref().unwrap().as_ref().borrow().parent.as_ref().cloned();
        while let Some(ref rc_node) = parent {
            let new_parent = rc_node.as_ref().borrow().parent.as_ref().cloned();
            if new_parent.is_none() {
                tree.root = parent;
            }
            parent = new_parent;
        }
    }

    fn set_root(&mut self, new_root: RedBlackTree<T>) {
        self.root = new_root;
    }

    fn inorder_traversal(rc_node: Tree<T>) {
        let node = rc_node.as_ref().borrow();
        match &node.left {
            Some(rc_node) => RBTree::inorder_traversal(rc_node.clone()),
            None => {},
        };
        println!("{}", node.key);
        match &node.right {
            Some(rc_node) => RBTree::inorder_traversal(rc_node.clone()),
            None => {},
        }
    }

    fn postorder_traversal(rc_node: Tree<T>) {
        let node = rc_node.as_ref().borrow();
        match &node.left {
            Some(rc_node) => RBTree::postorder_traversal(rc_node.clone()),
            None => {},
        };
        match &node.right {
            Some(rc_node) => RBTree::postorder_traversal(rc_node.clone()),
            None => {},
        };
        println!("{}", node.key);
    }

    pub fn clear(&mut self) {
        match &self.root {
            Some(rc_node) => RBTree::node_clear(rc_node.clone()),
            None => {},
        }
        self.root = None;
        self.count = 0;
        self.height = 0;
    }

    fn node_clear(rc_node: Tree<T>) {
        //Remove all references to ensure no rc memory leaks
        let mut node = rc_node.as_ref().borrow_mut();
        match &node.left {
            Some(rc_node) => {
                RBTree::node_clear(rc_node.clone());
                node.parent = None;
                node.left = None;
                node.right = None;
            },
            None => {}
        };
        match &node.right {
            Some(rc_node) => {
                RBTree::node_clear(rc_node.clone());
                node.parent = None;
                node.left = None;
                node.right = None;
            },
            None => {}
        }
    }

    pub fn get_count(&self) -> usize {
        self.count
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn tree_height(rc_node: Tree<T>) -> usize {
        let node = rc_node.as_ref().borrow();
        let mut left_height = 0;
        let mut right_height = 0;
        match &node.left {
            Some(rc_node) => left_height = RBTree::tree_height(rc_node.clone()) + 1,
            None => left_height = 1,
        }
        match &node.right {
            Some(rc_node) => right_height = RBTree::tree_height(rc_node.clone()) + 1,
            None => right_height = 1,
        }
        if left_height > right_height {
            return left_height;
        } else {
            return right_height;
        }
    }
}


fn main() {
    let mut tree = RBTree::<u32>::new();
    tree.insert(1);
    tree.insert(0);
    tree.insert(10);
    tree.insert(9);
    tree.insert(11);
    tree.insert(12);
    tree.insert(7);
    tree.insert(6);
    // tree.insert(5);
    RBTree::rotate_left(tree.root.as_ref().unwrap().clone());
    RBTree::fix_root(&mut tree);
    println!("=== Post Order Traversal ===");
    RBTree::postorder_traversal(tree.root.as_ref().unwrap().clone());
    println!("=== In Order Traversal ===");
    RBTree::inorder_traversal(tree.root.as_ref().unwrap().clone());
    println!("Height: {}", tree.get_height());
    println!("DELETE");
    tree.delete(9);
    println!("=== Post Order Traversal ===");
    RBTree::postorder_traversal(tree.root.as_ref().unwrap().clone());
    println!("=== In Order Traversal ===");
    RBTree::inorder_traversal(tree.root.as_ref().unwrap().clone());
    println!("Count: {}", tree.get_count());
    println!("Height: {}", tree.get_height());
    println!("Empty? {}", tree.is_empty());
    tree.clear();
    println!("Empty? {}", tree.is_empty());
    // println!("=== Start CLI ===");
    // println!("INSTRUCTIONS:");
    // println!("Insert: I <integer>");
    // println!("Delete: D <integer>");
    // println!("In-order Traversal: T");
    // println!("Empty Tree: E");
    // println!("Exit Program: EXIT");
    let stdin = io::stdin();
    let mut line = String::new();
    let mut loop_tree = RBTree::<i64>::new();
    return;
    loop {
        println!("Enter Command: ");
        io::stdout().flush();
        stdin.lock().read_line(&mut line).unwrap();
        let mut args: Vec<&str> = line.trim().split(' ').collect();
        if args[0] == "" {
            println!("Please Enter A Valid Command. Use Command H for Help!");
        } else if args[0] == "I" {
            if args.len() == 2 {
                match args[1].parse::<i64>() {
                    Ok(T) => {loop_tree.insert(T); println!("{} has been inserted", T)},
                    Err(E) => println!("Invalid Argument"),
                }
            } else {
                println!("Invalid number of arguments!");
            }
        } else if args[0] == "D" {
            if args.len() == 2 {
                match args[1].parse::<i64>() {
                    Ok(T) => {loop_tree.delete(T); println!("{} has been deleted", T)},
                    Err(E) => println!("Invalid Argument"),
                }
            } else {
                println!("Invalid number of arguments!");
            }
        } else if args[0] == "T" {
            println!("===In-order Traversal===");
            if !loop_tree.root.is_none() {
                RBTree::inorder_traversal(loop_tree.root.as_ref().cloned().unwrap().clone());
            } else {
                println!("No root");
            }
        } else if args[0] == "H" {
            println!("INSTRUCTIONS:");
            println!("Insert: I <integer>");
            println!("Delete: D <integer>");
            println!("In-order Traversal: T");
            println!("Empty Tree: E");
            println!("Exit Program: EXIT");
        } else if args[0] == "E" {
            loop_tree.clear();
            println!("Tree Has Been Emptied");
        } else if args[0] == "EXIT" {
            break;
        }
        args.clear();
        line.clear();
    }
}
