use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use regular_expression_matching::{dp, nfa};

fn bench_input_repeat(c: &mut Criterion) {
    let inputs = &[500, 1_000, 2_000];
    let mut group = c.benchmark_group("Input repeat");
    let pattern = "a*a*a*a*a*a*a*a*".to_owned();
    for &x in inputs.iter() {
        let input = String::from_utf8(vec![b'a'; x as usize]).unwrap();
        group.throughput(Throughput::Elements(x));
        group.bench_function("DP", |b| {
            b.iter(|| assert!(dp::Solution::is_match(input.clone(), pattern.clone())))
        });
        group.bench_function("NFA", |b| {
            b.iter(|| assert!(nfa::Solution::is_match(input.clone(), pattern.clone())))
        });
    }
}

fn bench_pattern_repeat(c: &mut Criterion) {
    let inputs = &[4, 8, 16];
    let mut group = c.benchmark_group("Pattern repeat");
    let input = String::from_utf8(vec![b'a'; 500]).unwrap();
    for &x in inputs.iter() {
        let pattern = vec!["a*"; x as usize].join("");
        group.throughput(Throughput::Elements(x));
        group.bench_function("DP", |b| {
            b.iter(|| assert!(dp::Solution::is_match(input.clone(), pattern.clone())))
        });
        group.bench_function("NFA", |b| {
            b.iter(|| assert!(nfa::Solution::is_match(input.clone(), pattern.clone())))
        });
    }
}

fn bench_regular(c: &mut Criterion) {
    let inputs = &[500, 1_000, 2_000];
    let mut group = c.benchmark_group("Regular");
    let pattern = "aab*ccd*eef*gg".to_owned();
    for &x in inputs.iter() {
        let input = "aa".to_owned()
            + &String::from_utf8(vec![b'b'; x as usize]).unwrap()
            + "cc"
            + &String::from_utf8(vec![b'd'; x as usize]).unwrap()
            + "ee"
            + &String::from_utf8(vec![b'f'; x as usize]).unwrap()
            + "gg";
        group.throughput(Throughput::Elements(x));
        group.bench_function("DP", |b| {
            b.iter(|| assert!(dp::Solution::is_match(input.clone(), pattern.clone())))
        });
        group.bench_function("NFA", |b| {
            b.iter(|| assert!(nfa::Solution::is_match(input.clone(), pattern.clone())))
        });
    }
}

criterion_group!(
    benches,
    bench_input_repeat,
    bench_pattern_repeat,
    bench_regular
);
criterion_main!(benches);
