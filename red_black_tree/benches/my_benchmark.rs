use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
//mod rb_tree;

use std::cell::{RefCell, Cell};
use std::rc::Rc;
use std::io::{self, BufRead, Write};

#[derive(Copy, Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}

enum Rotation<T> {
    LeftLeft(Tree<T>),
    LeftRight(Tree<T>),
    RightLeft(Tree<T>),
    RightRight(Tree<T>),
    None,
}

type Tree<T> = Rc<RefCell<TreeNode<T>>>;
type RedBlackTree<T> = Option<Tree<T>>;
#[derive(Debug)]
struct TreeNode<T> {
    pub color: Cell<NodeColor>,
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
                let mut new_rc_node = RBTree::node_insert(rc_node.clone(), key);
                match RBTree::recolor(new_rc_node.clone()) {
                    Rotation::LeftLeft(rc_node) => {
                        let rc_parent = rc_node.as_ref().borrow().parent.as_ref().cloned().unwrap();
                        let rc_grandparent = rc_parent.as_ref().borrow().parent.as_ref().cloned().unwrap();
                        RBTree::rotate_right(rc_grandparent.clone());
                        let parent = rc_parent.as_ref().borrow();
                        let grandparent = rc_grandparent.as_ref().borrow();
                        parent.color.swap(&grandparent.color);
                    },
                    Rotation::LeftRight(rc_node) => {
                        let rc_parent = rc_node.as_ref().borrow().parent.as_ref().cloned().unwrap();
                        RBTree::rotate_left(rc_parent.clone());
                        let rc_grandparent = rc_parent.as_ref().borrow().parent.as_ref().cloned().unwrap();
                        RBTree::rotate_right(rc_grandparent.clone());
                        let parent = rc_parent.as_ref().borrow();
                        let grandparent = rc_grandparent.as_ref().borrow();
                        parent.color.swap(&grandparent.color);
                    },
                    Rotation::RightLeft(rc_node) => {
                        let rc_parent = rc_node.as_ref().borrow().parent.as_ref().cloned().unwrap();
                        RBTree::rotate_right(rc_parent.clone());
                        let rc_grandparent = rc_parent.as_ref().borrow().parent.as_ref().cloned().unwrap();
                        RBTree::rotate_left(rc_grandparent.clone());
                        let parent = rc_parent.as_ref().borrow();
                        let grandparent = rc_grandparent.as_ref().borrow();
                        parent.color.swap(&grandparent.color);
                    },
                    Rotation::RightRight(rc_node) => {
                        // println!("{:?}", rc_node);
                        let rc_parent = rc_node.as_ref().borrow().parent.as_ref().cloned().unwrap();
                        let rc_grandparent = rc_parent.as_ref().borrow().parent.as_ref().cloned().unwrap();
                        RBTree::rotate_left(rc_grandparent.clone());
                        let parent = rc_parent.as_ref().borrow();
                        let grandparent = rc_grandparent.as_ref().borrow();
                        parent.color.swap(&grandparent.color);
                    },
                    Rotation::None => {},
                }
                self.count += 1;
            },
            None => {
                self.root = Some(Rc::new(RefCell::new(TreeNode{color: Cell::new(NodeColor::Black), key: key, parent: None, left: None, right: None})));
                self.count = 1;
            },
        }
        self.height = RBTree::tree_height(self.root.as_ref().cloned().unwrap().clone());
    }

    fn node_insert(rc_node: Tree<T>, key: T) -> Tree<T>{
        let mut node = rc_node.as_ref().borrow_mut();
        let new_node = TreeNode{color: Cell::new(NodeColor::Red), key:key, parent: Some(rc_node.clone()), left: None, right: None};
        let rc_new_node = Rc::new(RefCell::new(new_node));
        if key < node.key {
            match &node.left {
                Some(rc_node) => return RBTree::node_insert(rc_node.clone(), key),
                None => {
                    node.left.insert(rc_new_node.clone());
                    return rc_new_node.clone();
                },
            }
        } else {
            match &node.right {
                Some(rc_node) => return RBTree::node_insert(rc_node.clone(), key),
                None => {
                    node.right.insert(rc_new_node.clone());
                    return rc_new_node.clone();
                },
            }
        }
    }

    fn recolor(rc_node: Tree<T>) -> Rotation<T> {
        let mut current = rc_node.as_ref().borrow();
        if current.parent.as_ref().is_none() { //if we are the root
            current.color.replace(NodeColor::Black);
            return Rotation::None;
        }

        let rc_parent = current.parent.as_ref().unwrap();
        let mut parent = rc_parent.as_ref().borrow();
        if parent.color.get() == NodeColor::Black {
            return Rotation::None;
        }

        let rc_grandparent = parent.parent.as_ref().unwrap();
        let mut grandparent = rc_grandparent.as_ref().borrow();

        let mut option_uncle;

        if grandparent.right.is_none() {
            option_uncle = grandparent.right.as_ref();
        } else if grandparent.left.is_none() {
            option_uncle = grandparent.left.as_ref();
        } else if Rc::ptr_eq(&grandparent.left.as_ref().unwrap().clone(), &current.parent.as_ref().unwrap().clone()) {
            option_uncle = grandparent.right.as_ref();
        } else {
            option_uncle = grandparent.left.as_ref();
        }

        match option_uncle {
            Some(rc_uncle) => {
                let uncle = rc_uncle.as_ref().borrow();
                if uncle.color.get() == NodeColor::Red {
                    parent.color.replace(NodeColor::Black);
                    uncle.color.replace(NodeColor::Black);
                    grandparent.color.replace(NodeColor::Red);
                    return RBTree::recolor(rc_grandparent.clone());
                } else {
                    let mut parent_left;
                    let mut grandparent_left;
                    if parent.left.as_ref().is_none() {
                        parent_left = false;
                    } else if parent.right.as_ref().is_none() {
                        parent_left = true;
                    } else {
                        parent_left = Rc::ptr_eq(&parent.left.as_ref().unwrap().clone(), &rc_node.clone());
                    }
                    if grandparent.left.as_ref().is_none() {
                        grandparent_left = false;
                    } else if grandparent.right.as_ref().is_none() {
                        grandparent_left = true;
                    } else {
                        grandparent_left = Rc::ptr_eq(&grandparent.left.as_ref().unwrap().clone(), &rc_parent.clone());
                    }

                    if grandparent_left && parent_left {
                        //Left left case
                        return Rotation::LeftLeft(rc_node.clone());
                    } else if grandparent_left && !parent_left {
                        //Left right case
                        return Rotation::LeftRight(rc_node.clone());
                    } else if !grandparent_left && parent_left {
                        //Right left case
                        return Rotation::RightLeft(rc_node.clone());
                    } else {
                        //Right right case
                        return Rotation::RightRight(rc_node.clone());
                    }
                }
            },
            None => {
                let mut parent_left;
                let mut grandparent_left;
                if parent.left.as_ref().is_none() {
                    parent_left = false;
                } else if parent.right.as_ref().is_none() {
                    parent_left = true;
                } else {
                    parent_left = Rc::ptr_eq(&parent.left.as_ref().unwrap().clone(), &rc_node.clone());
                }
                if grandparent.left.as_ref().is_none() {
                    grandparent_left = false;
                } else if grandparent.right.as_ref().is_none() {
                    grandparent_left = true;
                } else {
                    grandparent_left = Rc::ptr_eq(&grandparent.left.as_ref().unwrap().clone(), &rc_parent.clone());
                }

                if grandparent_left && parent_left {
                    //Left left case
                    return Rotation::LeftLeft(rc_node.clone());
                } else if grandparent_left && !parent_left {
                    //Left right case
                    return Rotation::LeftRight(rc_node.clone());
                } else if !grandparent_left && parent_left {
                    //Right left case
                    return Rotation::RightLeft(rc_node.clone());
                } else {
                    //Right right case
                    return Rotation::RightRight(rc_node.clone());
                }
            },
        }
    }

    pub fn find(&mut self, key: T) -> Option<Tree<T>> {
        match &self.root {
            Some(rc_node) => RBTree::node_find(rc_node.clone(), key),
            None => None,
        }
    }

    fn node_find(rc_node: Tree<T>, key: T) -> Option<Tree<T>> {
        let node = rc_node.as_ref().borrow();
        if node.key == key {
            return Some(rc_node.clone());
        } else if key < node.key {
            match &node.left {
                Some(rc_node) => return RBTree::node_find(rc_node.clone(), key),
                None => return None,
            }
        } else {
            match &node.right {
                Some(rc_node) => return RBTree::node_find(rc_node.clone(), key),
                None => return None,
            }
        }
    }

    pub fn delete(&mut self, key: T) {
        match &self.root {
            Some(rc_node) => {
                match RBTree::node_find(rc_node.clone(), key) {
                    Some(_) => {
                        let new_root = RBTree::node_delete(rc_node.clone(), key);
                        self.set_root(new_root);
                        self.count -= 1;
                    },
                    None => {},
                }
            },
            None => {},
        }
        self.height = RBTree::tree_height(self.root.as_ref().cloned().unwrap().clone());
    }

    fn node_delete(rc_node: Tree<T>, key: T) -> Option<Tree<T>> {
        // let rc_replacement_node = RBTree::find_replacement(rc_node.clone());
        // let mut node = rc_node.as_ref().borrow_mut();
        // let rc_parent;
        // match node.parent.as_ref().cloned() {
        //     Some(rc_parent_node) => rc_parent = Some(rc_parent_node.clone()),
        //     None => rc_parent = None,
        // }
        //
        // if rc_replacement_node.is_none() {
        //     if rc_parent.is_none() {
        //
        //     } else {
        //         if node.color.get() == NodeColor::Black {
        //             //Fix Double Black
        //             RBTree::fix_doubleblack(rc_node.clone());
        //         } else {
        //             let rc_parent = rc_parent.unwrap();
        //             let mut parent = rc_parent.as_ref().borrow_mut();
        //             if parent.left.is_none() || parent.right.is_none() {
        //
        //             } else {
        //                 if Rc::ptr_eq(&parent.left.as_ref().cloned().unwrap(), &rc_node) {
        //                     parent.right.as_ref().cloned().unwrap().as_ref().borrow().color.replace(NodeColor::Red);
        //                 } else {
        //                     parent.left.as_ref().cloned().unwrap().as_ref().borrow().color.replace(NodeColor::Red);
        //                 }
        //             }
        //
        //             if Rc::ptr_eq(&parent.left.as_ref().cloned().unwrap(), &rc_node) {
        //                 node.parent = None;
        //                 parent.left = None;
        //             } else {
        //                 node.parent = None;
        //                 parent.right = None;
        //             }
        //             return rc_replacement_node.as_ref().cloned();
        //         }
        //     }
        // }
        //
        //
        //
        // if node.left.is_none() || node.right.is_none() {
        //     if node.parent.is_none() {
        //         //replacement node is now new root
        //         let rc_replacement_node = rc_replacement_node.as_ref().cloned().unwrap();
        //         let mut replacement_node = rc_replacement_node.as_ref().borrow_mut();
        //         replacement_node.parent = None;
        //         replacement_node.left = None;
        //         replacement_node.right = None;
        //     } else {
        //         let rc_parent = node.parent.as_ref().cloned().unwrap();
        //         let mut parent = rc_parent.as_ref().borrow_mut();
        //         let rc_replacement_node = rc_replacement_node.as_ref().cloned().unwrap();
        //         let mut replacement_node = rc_replacement_node.as_ref().borrow_mut();
        //         if parent.left.is_none() {
        //             parent.right.replace(rc_replacement_node.clone());
        //         } else if parent.right.is_none() {
        //             parent.left.replace(rc_replacement_node.clone());
        //         } else if Rc::ptr_eq(&parent.left.as_ref().cloned().unwrap(), &rc_node) {
        //             parent.left.replace(rc_replacement_node.clone());
        //         } else {
        //             parent.right.replace(rc_replacement_node.clone());
        //         }
        //
        //         replacement_node.parent.replace(rc_parent.clone());
        //
        //         if node.color.get() == NodeColor::Black {
        //             RBTree::fix_doubleblack(rc_replacement_node.clone());
        //         } else {
        //             if replacement_node.color.get() == NodeColor::Black && node.color.get() == NodeColor::Black {
        //                 RBTree::fix_doubleblack(rc_replacement_node.clone());
        //             } else {
        //                 replacement_node.color.replace(NodeColor::Black);
        //             }
        //         }
        //     }
        //     return rc_replacement_node.as_ref().cloned();
        // }
        //
        // let rc_replacement_node = rc_replacement_node.as_ref().cloned().unwrap();
        // let mut replacement_node = rc_replacement_node.as_ref().borrow_mut();
        //
        //
        // let r_n_parent = replacement_node.parent.take();
        // let r_n_left = replacement_node.left.take();
        // let r_n_right = replacement_node.right.take();
        //
        // replacement_node.parent = node.parent.take();
        // replacement_node.left = node.left.take();
        // replacement_node.right = node.right.take();
        //
        // return RBTree::node_delete(rc_replacement_node.clone());


        //BST DELETE

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

    // fn fix_doubleblack(rc_node: Tree<T>) {
    //     let node = rc_node.as_ref().borrow_mut();
    //     if node.parent.is_none() {
    //         return;
    //     }
    //
    //     let rc_parent = node.parent.as_ref().cloned().unwrap();
    //     let parent = rc_parent.as_ref().borrow_mut();
    //     let mut is_sibling_left;
    //     let option_sibling;
    //     if Rc::ptr_eq(&parent.left.as_ref().cloned().unwrap(), &rc_node) {
    //         is_sibling_left = false;
    //         option_sibling = parent.right.as_ref().cloned();
    //     } else {
    //         is_sibling_left = true;
    //         option_sibling = parent.left.as_ref().cloned();
    //     }
    //
    //     if option_sibling.is_none() {
    //         RBTree::fix_doubleblack(rc_parent.clone());
    //     } else {
    //         let rc_sibling = option_sibling.as_ref().cloned().unwrap();
    //         let sibling = rc_sibling.as_ref().borrow_mut();
    //         if sibling.color.get() == NodeColor::Red {
    //             parent.color.replace(NodeColor::Red);
    //             sibling.color.replace(NodeColor::Black);
    //             if is_sibling_left {
    //                 RBTree::rotate_right(rc_parent.clone());
    //             } else {
    //                 RBTree::rotate_left(rc_parent.clone());
    //             }
    //             RBTree::fix_doubleblack(rc_node.clone());
    //         } else {
    //             let mut left_red;
    //             let mut right_red;
    //             if sibling.left.is_none() {
    //                 left_red = false;
    //             } else {
    //                 let rc_left_child = sibling.left.as_ref().cloned().unwrap();
    //                 let left_child = rc_left_child.as_ref().borrow();
    //                 left_red = left_child.color.get() == NodeColor::Red;
    //             }
    //
    //             if sibling.right.is_none() {
    //                 right_red = false
    //             } else {
    //                 let rc_right_child = sibling.right.as_ref().cloned().unwrap();
    //                 let right_child = rc_right_child.as_ref().borrow();
    //                 right_red = right_child.color.get() == NodeColor::Red;
    //             }
    //
    //             if left_red || right_red {
    //                 if left_red {
    //                     let rc_left_child = sibling.left.as_ref().cloned().unwrap();
    //                     let left_child = rc_left_child.as_ref().borrow();
    //                     if is_sibling_left {
    //                         left_child.color.replace(sibling.color.get());
    //                         sibling.color.replace(parent.color.get());
    //                         RBTree::rotate_right(rc_parent.clone());
    //                     } else {
    //                         left_child.color.replace(parent.color.get());
    //                         RBTree::rotate_right(rc_sibling.clone());
    //                         RBTree::rotate_left(rc_parent.clone());
    //                     }
    //                 } else {
    //                     let rc_right_child = sibling.right.as_ref().cloned().unwrap();
    //                     let right_child = rc_right_child.as_ref().borrow();
    //                     if is_sibling_left {
    //                         right_child.color.replace(parent.color.get());
    //                         RBTree::rotate_left(rc_sibling.clone());
    //                         RBTree::rotate_right(rc_parent.clone());
    //                     } else {
    //                         right_child.color.replace(parent.color.get());
    //                         sibling.color.replace(parent.color.get());
    //                         RBTree::rotate_left(rc_parent.clone());
    //                     }
    //                 }
    //                 parent.color.replace(NodeColor::Black);
    //             } else {
    //                 sibling.color.replace(NodeColor::Red);
    //                 if parent.color.get() == NodeColor::Black {
    //                     RBTree::fix_doubleblack(rc_parent.clone());
    //                 } else {
    //                     parent.color.replace(NodeColor::Black);
    //                 }
    //             }
    //         }
    //     }
    // }

    fn find_replacement(rc_node: Tree<T>)->Option<Tree<T>> {
        let node = rc_node.as_ref().borrow();
        if node.left.is_none() && node.right.is_none() {
            return None;
        } else if node.left.is_none() {
            return node.right.as_ref().cloned();
        } else if node.right.is_none() {
            return node.left.as_ref().cloned();
        } else {
            let right_node = node.right.as_ref().cloned().unwrap();
            let mut option_leftmost_node = node.right.as_ref().cloned();
            while let Some(ref rc_node) = option_leftmost_node {
                let option_new_node = rc_node.as_ref().borrow().left.as_ref().cloned();
                if option_new_node.is_none() {
                    return option_leftmost_node;
                }
                option_leftmost_node = option_new_node;
            }
            None
        }
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
        match node.parent.take() {
            Some(parent_rc_node) => {
                let mut parents_parent = parent_rc_node.as_ref().borrow_mut();
                let mut is_left = false;
                if parents_parent.left.is_none() {
                    is_left = false;
                } else if parents_parent.right.is_none() {
                    is_left = true;
                } else if Rc::ptr_eq(&parents_parent.left.as_ref().unwrap(), &rc_node) {
                    is_left = true;
                } else {
                    is_left = false;
                }
                if is_left {
                    parents_parent.left.insert(rc_left_node.clone());
                } else {
                    parents_parent.right.insert(rc_left_node.clone());
                }

            },
            None => left_node.parent = None,
        }
        left_node.parent = node.parent.take();
        node.parent.insert(rc_left_node.clone());
        //Move left_right_node to left of current node
        match left_node.right.as_ref().cloned() {
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
            Some(r_rc_node) => rc_right_node = r_rc_node.clone(),
            None => panic!("No right node"),
        }
        let mut right_node = rc_right_node.as_ref().borrow_mut();

        //Swap parents
        match node.parent.as_ref().cloned() {
            Some(parent_rc_node) => {
                let mut parents_parent = parent_rc_node.as_ref().borrow_mut();
                let mut is_left = false;
                if parents_parent.left.is_none() {
                    is_left = false;
                } else if parents_parent.right.is_none() {
                    is_left = true;
                } else if Rc::ptr_eq(&parents_parent.left.as_ref().unwrap(), &rc_node) {
                    is_left = true;
                } else {
                    is_left = false;
                }
                if is_left {
                    parents_parent.left.insert(rc_right_node.clone());
                } else {
                    parents_parent.right.insert(rc_right_node.clone());
                }

            },
            None => right_node.parent = None,
        }
        right_node.parent = node.parent.take();
        node.parent.insert(rc_right_node.clone());
        //Move right_left_node to right of current node
        match right_node.left.take() {
            Some(new_rc_node) => {
                let mut new_node = new_rc_node.as_ref().borrow_mut();
                new_node.parent.insert(rc_node.clone());
                node.right.insert(new_rc_node.clone());
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

    fn print(&self) {
        match &self.root {
            Some(rc_node) => RBTree::node_print(rc_node.clone(), String::from(""), String::from("")),
            None => {},
        }
    }

    fn node_print(rc_node: Tree<T>, mut prefix: String, mut children_prefix: String) {
        let node = rc_node.as_ref().borrow();
        let left;
        let right;
        let parent;
        if node.left.is_none() {
            left = None;
        } else {
            left = Some(node.left.as_ref().cloned().unwrap().as_ref().borrow().key);
        }

        if node.right.is_none() {
            right = None;
        } else {
            right = Some(node.right.as_ref().cloned().unwrap().as_ref().borrow().key);
        }

        if node.parent.is_none() {
            parent = None;
        } else {
            parent = Some(node.parent.as_ref().cloned().unwrap().as_ref().borrow().key);
        }
        println!("{}{}{} Color:{:?} Parent:{:?} Left:{:?} Right:{:?}", children_prefix, prefix, node.key, node.color, parent, left, right);
        children_prefix.push_str("| - ");
        match &node.left {
            Some(rc_node) => {
                // prefix.push_str(" - ");
                RBTree::node_print(rc_node.clone(), prefix.clone(), children_prefix.clone());
            },
            None => {},
        };
        match &node.right {
            Some(rc_node) => RBTree::node_print(rc_node.clone(), prefix.clone(), children_prefix.clone()),
            None => {},
        };
    }
}

// Insert benchmarking
fn add_bench(tree: &mut RBTree<i64>, additions:i64) {
    for key in 0..additions {
        tree.insert(key);
    }
}

fn search_bench(tree: &mut RBTree<i64>, searches: i64) {
    for key in 0..searches {
        tree.find(key);
    }
}



pub fn criterion_benchmark(c: &mut Criterion) {
    let array = [10000, 40000, 70000, 100000, 130000];
    let search_array = [1,2,3,4,5];
    let mut benches = c.benchmark_group("RBTree Benchmarking");
    for i in 0..array.len() {
        let mut tree = &mut RBTree::new();
        let mut num = array[i];
        benches.bench_with_input(BenchmarkId::from_parameter(num), &num, 
        |b,&i| {
            b.iter(|| add_bench(tree, i))
        });
        benches.bench_with_input(BenchmarkId::new("Search", search_array[i]), &num, 
        |b,&i| {
            b.iter(|| search_bench(tree, i))
        });
    }
    benches.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

