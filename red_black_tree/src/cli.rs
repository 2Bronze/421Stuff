use crate::rbtree::RBTree;
use std::io::{stdin, stdout, BufRead, Write};

pub struct CLI {
    tree: RBTree<i64>,
}

impl CLI {
    pub fn new()->Self {
        CLI{tree: RBTree::new()}
    }

    pub fn run(&mut self) {
        let mut line = String::new();
        println!("===== WELCOME TO TREE CLI =====");
        println!("COMMANDS:");
        println!("Insert: 'I <Integer>'");
        println!("Delete: 'D <Integer>'");
        println!("Height of Tree: HT");
        println!("Count of Nodes: CN");
        println!("Count of Leaves: CL");
        println!("Clear: 'C'");
        println!("Traverse: 'T'");
        println!("Print Tree: 'P'");
        println!("Change Tree: 'CT <'RB'/'AVL'>'");
        println!("More Help: 'H'");
        println!("Exit CLI: 'E'");
        println!("===============================");
        println!("");
        println!("Enter Commands:");
        loop {
            stdout().flush();
            stdin().lock().read_line(&mut line).unwrap();
            let args: Vec<&str> = line.trim().split(' ').collect();
            let command = args[0];

            if command == "I" { //Insert
                if args.len() == 2 {
                    match args[1].parse::<i64>() {
                        Ok(value) => self.insert(value),
                        Err(_) => println!("Please Enter an Integer"),
                    }
                } else {
                    println!("Invalid Number of Arguments.");
                }
            } else if command == "D" { //Delete
                if args.len() == 2 {
                    match args[1].parse::<i64>() {
                        Ok(value) => self.delete(value),
                        Err(_) => println!("Please Enter an Integer"),
                    }
                } else {
                    println!("Invalid Number of Arguments.");
                }
            } else if command == "HT" {
                self.height();
            } else if command == "CN" {
                self.count();
            } else if command == "CL" {
                self.count_leaves();
            } else if command == "C" { //Clear
                self.clear();
            } else if command == "T" { //Traverse
                println!("=== Traversal Menu ===");
                println!("Please enter the type of traversal:\n 1 - In-order Traversal\n 2 - Post-order Traversal\n H - Help\n E - Exit to main");
                println!("Enter Commands");
                loop {
                    stdout().flush();
                    line.clear();
                    stdin().lock().read_line(&mut line).unwrap();
                    let traversal_args : Vec<&str> = line.trim().split(' ').collect();
                    let traversal_type = traversal_args[0];
                    if traversal_type == "1" {
                        self.inorder_traversal();
                    } else if traversal_type == "2" {
                        self.postorder_traversal();
                    } else if traversal_type == "H" {
                        println!("COMMANDS:\n 1 - In-order Traversal\n 2 - Post-order Traversal\n H - Help\n E - Exit to main");
                    } else if traversal_type == "E" {
                        println!("Back to main...");
                        println!("");
                        println!("Enter Commands:");
                        break;
                    }
                }
            } else if command == "P" { //Print
                self.print();
            } else if command == "CT" {
                if args.len() == 2 {
                    if args[1] == "RB" {
                        self.tree = RBTree::new();
                        println!("Using RB Tree...");
                    } else if args[1] == "AVL" {
                        println!("Currently not Implemented!");
                    } else {
                        println!("Invalid Tree Type");
                    }
                } else {
                    println!("Invalid Number of Arguments.");
                }
            } else if command == "H" { //CLI Help
                self.help();
            } else if command == "E" { //Exit
                println!("Exiting...");
                println!("Thank you for using TREE CLI");
                break;
            } else {
                println!("Please Enter a Valid Command. Use Command 'H' for Help.")
            }
            line.clear();
        }
    }

    fn insert(&mut self, value: i64) {
        let mut tree = &mut self.tree;
        println!("Inserting {}...", value);
        tree.insert(value);
        println!("Finished");
    }

    fn delete(&mut self, value: i64) {
        let mut tree = &mut self.tree;
        println!("Deleteing {}...", value);
        tree.delete(value);
        println!("Finished");
    }

    fn clear(&mut self) {
        let mut tree = &mut self.tree;
        println!("Clearing Tree...");
        tree.clear();
        println!("Finished");
    }

    fn height(&self) {
        let tree = &self.tree;
        println!("Height: {}", tree.get_height());
    }

    fn count(&self) {
        let tree = &self.tree;
        println!("Count: {}", tree.get_count());
    }

    fn count_leaves(&self) {
        let tree = &self.tree;
        println!("Leaf Count: {}", tree.get_leaf_count());
    }

    fn inorder_traversal(&mut self) {
        let tree = &self.tree;
        println!("=== In-Order Traversal ===");
        match &tree.root {
            Some(rc_root) => RBTree::inorder_traversal(rc_root.clone()),
            None => println!("TREE IS EMPTY!"),
        }
        println!("===========================");
    }

    fn postorder_traversal(&mut self) {
        let tree = &self.tree;
        println!("=== Post-Order Traversal ===");
        match &tree.root {
            Some(rc_root) => RBTree::postorder_traversal(rc_root.clone()),
            None => println!("TREE IS EMPTY!"),
        }
        println!("===========================");
    }

    fn print(&mut self) {
        let tree = &self.tree;
        println!("=== Tree ===");
        tree.print();
        println!("===========================");
    }

    fn help(&mut self) {
        println!("===== HELP MENU ======");
        println!("Insert");
        println!("\t I <Integer>");
        println!("\t This command inserts an integer into the tree.");
        println!("");
        println!("Delete");
        println!("\t D <Integer>");
        println!("\t This command deletes an integer from the tree");
        println!("");
        println!("Clear");
        println!("\t C");
        println!("\t This command clears the tree");
        println!("");
        println!("Traverse");
        println!("\t T");
        println!("\t This command opens the traverse menu");
        println!("\t\t In the traverse menu:");
        println!("\t\t In-Order Traversal");
        println!("\t\t\t 1");
        println!("\t\t\t This command traverses the tree in order and prints out the keys");
        println!("\t\t Post-Order Traversal:");
        println!("\t\t\t 2");
        println!("\t\t\t This command traverses the tree post order and prints out the keys");
        println!("");
        println!("Print");
        println!("\t P");
        println!("\t This command prints out the tree");
        println!("");
        println!("Help");
        println!("\t H");
        println!("\t This command will show this menu again");
        println!("");
        println!("Exit");
        println!("\t E");
        println!("\t This command exit the CLI");
        println!("");
        println!("=======================");
    }
}
