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
use mathsys::{
    Interpreter,
    Runtime,
    Failure
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
    group.throughput(Throughput::Bytes(include_bytes!("../data/root.msm").len() as u64));
    struct Handler; impl<'valid> Runtime<'valid> for Handler {
        fn critical(_failure: Failure<'valid>) -> ! {panic!()}
        fn resolve(&'valid self, module: &'valid str) -> &'valid [u8] {return match module {
            "data/root.msm" => include_bytes!("../data/root.msm"),
            _ => unreachable!()
        }}
        fn error(_failure: Failure<'valid>) -> () {}
        fn warning(_failure: Failure<'valid>) -> () {}
    }
    let interpreter = Interpreter::from(Handler);
    group.bench_function("full", |bencher| bencher.iter(|| {
        let result = interpreter.latex("data/root.msm");
        black_box(result);
    }));
}