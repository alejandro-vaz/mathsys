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
    static DATA: &'static [u8; 3199] = include_bytes!("../data/root.msm");
    let mut group = criterion.benchmark_group("tokenizer");
    let mut root = Root::default();
    group.throughput(Throughput::Bytes(DATA.len() as u64));
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