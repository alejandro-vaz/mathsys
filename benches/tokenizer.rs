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
    Resolver
};

//> HEAD -> LIBUTILS
use libutils::{
    active_reporting::{
        Root,
        Report
    },
    systemstd::{
        System,
        OpenMode
    }
};

//> HEAD -> CORE
use core::{
    hint::black_box,
    marker::PhantomCovariantLifetime
};

//> HEAD -> ELSA
use elsa::FrozenMap;


//^
//^ BENCHES
//^

//> BENCHES -> SETUP
criterion_group!(tokenizer, benches);
criterion_main!(tokenizer);

//> BENCHES -> HANDLER
struct Handler<'valid> {
    cache: FrozenMap<&'valid str, String>
} impl<'valid> Resolver<'valid> for Handler<'valid> {
    fn resolve(
        &'valid self, 
        filename: &'valid str, 
        report: Report<"Resolver">
    ) -> &'valid str {
        return match self.cache.get(filename) {
            Some(cached) => cached,
            None => self.cache.insert(
                filename, 
                System::expect(System::expect(
                    System::path(filename).file::<{OpenMode::Read}>(None), 
                    &*report
                ).read(), &*report)
            )
        };
    }
}

//> BENCHES -> RUN
fn benches(criterion: &mut Criterion) -> () {
    let mut group = criterion.benchmark_group("tokenizer");
    group.throughput(Throughput::Elements(1));
    let mut root = Root::default();
    let interpreter = Interpreter {
        lifetime: PhantomCovariantLifetime::new(),
        resolver: Handler {
            cache: FrozenMap::new()
        },
        warning: |failure, chain| System::warning(failure, chain),
        error: |failure, chain| System::error(failure, chain),
        critical: |failure, chain| System::critical(failure, chain)
    };
    group.bench_function("full", |bencher| bencher.iter(|| {
        let result = interpreter.latex("std/file.msm", root.to());
        black_box(result);
    }));
}