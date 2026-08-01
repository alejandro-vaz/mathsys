//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![feature(phantom_variance_markers)]

//> HEAD -> CRITERION
use criterion::{
    Criterion,
    Throughput,
    criterion_main,
    criterion_group
};

//> HEAD -> MATHSYS
use mathsys::{
    Interpreter,
    Runtime,
    Failure
};

//> HEAD -> CORE
use core::{
    hint::black_box,
    marker::PhantomCovariantLifetime
};


//^
//^ BENCHES
//^

//> BENCHES -> SETUP
criterion_group!(tokenizer, benches);
criterion_main!(tokenizer);

//> BENCHES -> RUN
fn benches(criterion: &mut Criterion) -> () {
    let mut group = criterion.benchmark_group("tokenizer");
    group.throughput(Throughput::Bytes(include_bytes!("../data/root.msm").len() as u64));
    struct Handler;
    impl<'valid> Runtime<'valid> for Handler {
        fn critical(&'valid self, _failure: Failure<'valid>) -> ! {panic!()}
        fn resolve(&'valid self, module: &'valid str) -> &'valid [u8] {return match module {
            "data/root.msm" => include_bytes!("../data/root.msm"),
            _ => unreachable!()
        }}
    }
    let interpreter = Interpreter {
        runtime: Handler,
        lifetime: PhantomCovariantLifetime::new()
    };
    group.bench_function("full", |bencher| bencher.iter(|| {
        let result = interpreter.latex("data/root.msm");
        black_box(result);
    }));
}