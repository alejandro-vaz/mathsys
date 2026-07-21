//^
//^ HEAD
//^

//> HEAD -> CRITERION
use criterion::{
    Criterion,
    Throughput,
    criterion_main,
    criterion_group
};

//> HEAD -> MATHSYS
use mathsys::Interpreter;

//> HEAD -> LIBUTILS
use libutils::{
    active_reporting::Root,
    systemstd::System,
    systemio::Dump
};

//> HEAD -> CORE
use core::hint::black_box;


//^
//^ BENCHES
//^

//> BENCHES -> SETUP
criterion_group!(tokenizer, benches);
criterion_main!(tokenizer);

//> BENCHES -> RUN
fn benches(criterion: &mut Criterion) -> () {
    let mut group = criterion.benchmark_group("tokenizer");
    group.throughput(Throughput::Elements(1));
    let mut root = Root::default();
    let interpreter = Interpreter {
        resolver: |filename, _report| match filename {
            "data/root.msm" => include_bytes!("../data/root.msm"),
            _ => panic!()
        },
        systemio: System::dump()
    };
    group.bench_function("full", |bencher| bencher.iter(|| {
        let result = interpreter.latex("data/root.msm", root.to());
        black_box(result);
    }));
}