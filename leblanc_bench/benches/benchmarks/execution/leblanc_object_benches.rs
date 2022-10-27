use std::time::Duration;
use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group};
use leblanc::leblanc::core::leblanc_object::LeBlancObject;
use leblanc::leblanc::rustblanc::types::LBObject;

const SIZE: usize = 256 * 256;


pub fn test_leblanc_crud(c: &mut Criterion) {
    LeBlancObject::null();

    let mut group = c.benchmark_group("LB_CRUD");
    group.sample_size(5000);

    group.bench_function(BenchmarkId::new("Create", "Single"),
    |b| b.iter_batched(|| (), |_| LeBlancObject::null(), BatchSize::SmallInput));



    group.bench_function(BenchmarkId::new("Drop", "Single"),
                         |b| {
                             b.iter_batched(LeBlancObject::null, drop, BatchSize::NumBatches(1))
                         });


    group.bench_function(BenchmarkId::new("Drop", "Mass"),
                        |b| {
                            let vec = vec![LeBlancObject::null(); SIZE];
                            b.iter_batched(|| vec.clone(), drop, BatchSize::NumBatches(1))
                        });
    group.finish();
}



criterion_group!(crud, test_leblanc_crud);