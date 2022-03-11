mod rbtree;
mod cli;
use crate::cli::CLI;


fn main() {
    let mut cli = CLI::new();
    cli.run();
}
