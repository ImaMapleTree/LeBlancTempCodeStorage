use criterion::{Criterion};

mod benchmarks;

/*criterion_main! {
    benchmarks::execution::instruction_test::benches,
}*/

fn main() {
    benchmarks::execution::instruction_test::benches();
    benchmarks::execution::leblanc_object_benches::crud();

    Criterion::default()
        .configure_from_args()
        .with_output_color(true)
        .final_summary();
}