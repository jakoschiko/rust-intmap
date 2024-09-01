use criterion::{black_box, criterion_group, criterion_main, Criterion};
use indexmap::IndexMap;
use intmap::{Entry, IntMap};
use std::collections::HashMap;

const VEC_COUNT: usize = 10_000;

criterion_group!(
    benches,
    u64_insert_built_in,
    u64_insert_built_in_without_capacity,
    u64_get_built_in,
    u64_insert_indexmap,
    u64_get_indexmap,
    u64_insert_intmap,
    u64_insert_intmap_checked,
    u64_insert_intmap_entry,
    u64_insert_intmap_without_capacity,
    u64_resize_intmap,
    u64_get_intmap,
);
criterion_main!(benches);

// ********** Built in **********

fn u64_insert_built_in(c: &mut Criterion) {
    let data = get_random_range(VEC_COUNT);
    let mut map = HashMap::with_capacity(data.len());

    c.bench_function("u64_insert_built_in", |b| {
        b.iter(|| {
            map.clear();

            for s in data.iter() {
                black_box(map.insert(s, s));
            }
        })
    });
}

fn u64_insert_built_in_without_capacity(c: &mut Criterion) {
    let data = get_random_range(VEC_COUNT);

    c.bench_function("u64_insert_built_in_without_capacity", |b| {
        b.iter(|| {
            let mut map = HashMap::new();

            for s in data.iter() {
                black_box(map.insert(s, s));
            }

            black_box(&map);
        })
    });
}

fn u64_get_built_in(c: &mut Criterion) {
    let data = get_random_range(VEC_COUNT);
    let mut map: HashMap<&u64, &u64> = HashMap::with_capacity(data.len());

    for s in data.iter() {
        black_box(map.insert(s, s));
    }

    c.bench_function("u64_get_built_in", |b| {
        b.iter(|| {
            for s in data.iter() {
                black_box({
                    map.contains_key(s);
                });
            }
        })
    });
}

// ********** IndexMap **********

fn u64_insert_indexmap(c: &mut Criterion) {
    let data = get_random_range(VEC_COUNT);
    let mut map = IndexMap::with_capacity(data.len());

    c.bench_function("u64_insert_indexmap", |b| {
        b.iter(|| {
            map.clear();

            for s in data.iter() {
                black_box(map.insert(s, s));
            }
        })
    });
}

fn u64_get_indexmap(c: &mut Criterion) {
    let data = get_random_range(VEC_COUNT);
    let mut map: IndexMap<&u64, &u64> = IndexMap::with_capacity(data.len());

    for s in data.iter() {
        black_box(map.insert(s, s));
    }

    c.bench_function("u64_get_indexmap", |b| {
        b.iter(|| {
            for s in data.iter() {
                black_box({
                    map.contains_key(s);
                });
            }
        })
    });
}

// ********** Intmap **********

fn u64_insert_intmap(c: &mut Criterion) {
    let data = get_random_range(VEC_COUNT);
    let mut map = IntMap::with_capacity(data.len());

    c.bench_function("u64_insert_intmap", |b| {
        b.iter(|| {
            map.clear();

            for s in data.iter() {
                black_box(map.insert(*s, s));
            }
        })
    });
}

fn u64_insert_intmap_checked(c: &mut Criterion) {
    let data = get_random_range(VEC_COUNT);
    let mut map = IntMap::with_capacity(data.len());

    c.bench_function("u64_insert_intmap_checked", |b| {
        b.iter(|| {
            map.clear();

            for s in data.iter() {
                black_box(map.insert_checked(*s, s));
            }
        })
    });
}

fn u64_insert_intmap_entry(c: &mut Criterion) {
    let data = get_random_range(VEC_COUNT);

    let mut map = IntMap::with_capacity(data.len());

    c.bench_function("u64_insert_intmap_entry", |b| {
        b.iter(|| {
            map.clear();

            for s in data.iter() {
                black_box(match map.entry(*s) {
                    Entry::Occupied(_) => panic!("unexpected while insert, i = {}", s),
                    Entry::Vacant(entry) => entry.insert(s),
                });
            }
        })
    });
}

fn u64_insert_intmap_without_capacity(c: &mut Criterion) {
    let data = get_random_range(VEC_COUNT);

    c.bench_function("u64_insert_intmap_without_capacity", |b| {
        b.iter(|| {
            let mut map = IntMap::new();

            for s in data.iter() {
                black_box(map.insert(*s, s));
            }

            black_box(&map);
        })
    });
}

fn u64_resize_intmap(c: &mut Criterion) {
    c.bench_function("u64_resize_intmap", |b| {
        b.iter(|| {
            let mut map: IntMap<u64> = IntMap::new();
            map.reserve(VEC_COUNT);
            black_box(&map);
        })
    });
}

fn u64_get_intmap(c: &mut Criterion) {
    let data = get_random_range(VEC_COUNT);

    let mut map = IntMap::with_capacity(data.len());
    for s in data.iter() {
        map.insert(*s, s);
    }

    c.bench_function("u64_get_intmap", |b| {
        b.iter(|| {
            for s in data.iter() {
                black_box(map.contains_key(*s));
            }
        })
    });
}

// ********** Misc **********

fn get_random_range(count: usize) -> Vec<u64> {
    use rand::prelude::StdRng;
    use rand::{Rng, SeedableRng};

    let mut vec = Vec::new();
    let mut rng = StdRng::seed_from_u64(4242);

    for _ in 0..count {
        vec.push(rng.gen::<u64>());
    }

    vec.sort();
    vec.dedup();

    vec
}
