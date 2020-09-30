#[macro_use]
extern crate criterion;
use criterion::{BenchmarkId, Criterion};
use redirectionio::api::{Rule, RulesMessage};
use redirectionio::http::{QueryParamSkipBuilder, Request};
use redirectionio::router::Router;
use std::fs::read_to_string;

fn create_router(filename: String) -> Router<Rule> {
    let content = read_to_string(filename).expect("Cannot open file");
    let rules: RulesMessage = serde_json::from_str(content.as_str()).expect("Cannot deserialize");
    let mut router = Router::<Rule>::default();

    for rule in rules.rules {
        router.insert(rule.into_route())
    }

    router
}

fn no_match_bench(c: &mut Criterion) {
    let files = vec![
        "../bench-files/large-rules-1k.json".to_string(),
        "../bench-files/large-rules-10k.json".to_string(),
        "../bench-files/large-rules-50k.json".to_string(),
        "../bench-files/large-rules-150k.json".to_string(),
        "../bench-files/large-rules-200k.json".to_string(),
    ];

    let mut group = c.benchmark_group("no_match");
    group.sample_size(10);

    for filename in files {
        let router = create_router(filename.clone());
        let path_builder = QueryParamSkipBuilder::default();
        let path = path_builder.build_query_param_skipped("/no-match");
        let request = Request::new(path, None, None, None).to_http_request().expect("");

        group.bench_with_input(BenchmarkId::from_parameter(filename.clone()), &filename, |b, _f| {
            b.iter(|| {
                router.match_request(&request);
            });
        });
    }

    group.finish();
}

fn no_match_cache_bench(c: &mut Criterion) {
    let files = vec![
        "../bench-files/large-rules-1k.json".to_string(),
        "../bench-files/large-rules-10k.json".to_string(),
        "../bench-files/large-rules-50k.json".to_string(),
        "../bench-files/large-rules-150k.json".to_string(),
        "../bench-files/large-rules-200k.json".to_string(),
    ];

    let mut group = c.benchmark_group("no_match_cache");
    group.sample_size(10);

    for filename in files {
        let mut router = create_router(filename.clone());
        let path_builder = QueryParamSkipBuilder::default();
        let path = path_builder.build_query_param_skipped("/no-match");
        let request = Request::new(path, None, None, None).to_http_request().expect("");

        router.cache(1000);

        group.bench_with_input(BenchmarkId::from_parameter(filename.clone()), &filename, |b, _f| {
            b.iter(|| {
                router.match_request(&request);
            });
        });
    }

    group.finish();
}

criterion_group!(benches, no_match_bench, no_match_cache_bench);
criterion_main!(benches);
