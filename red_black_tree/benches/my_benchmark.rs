use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
#[path = "../src/rbtree.rs"]
mod rbtree;
use crate::rbtree::RBTree;

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
