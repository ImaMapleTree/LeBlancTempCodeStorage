use std::ops::Deref;
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Bencher};
use leblanc::leblanc::core::interpreter::instructions2::Instruction2;



fn vanilla_instruct_unwrap(instruct: &Instruction2) {
    Instruction2::bytes(instruct);
}

fn pointer_instruct_unwrap(instruct: &Instruction2) {
    Instruction2::bytes2(instruct, 2);
}

fn pointer_single_instruct_unwrap(instruct: &Instruction2) {
    Instruction2::byte(instruct, 1);
}


type InstructBenchFn = fn(&Instruction2) -> ();

fn instruction_setup(bench: &mut Bencher, input: &InstructBenchFn) {
    bench.iter(|| {
        input(&Instruction2::CALL_NORMAL(0, [9999, 9999]))
    })
}



pub fn bench_instruction_unwraps(c: &mut Criterion) {
    let mut group = c.benchmark_group("Instruction Group");
    group.bench_with_input(BenchmarkId::new("INSTRUCTION.UNWRAP.VANILLA", 1), &(vanilla_instruct_unwrap as InstructBenchFn), instruction_setup);
    group.bench_with_input(BenchmarkId::new("INSTRUCTION.UNWRAP.PTR", 1), &(pointer_instruct_unwrap as InstructBenchFn), instruction_setup);
    group.bench_with_input(BenchmarkId::new("INSTRUCTION.UNWRAP.PTR.SINGLE", 1), &(pointer_single_instruct_unwrap as InstructBenchFn), instruction_setup);
    group.finish();
}

criterion_group!(benches, bench_instruction_unwraps);