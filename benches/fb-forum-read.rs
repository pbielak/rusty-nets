#[macro_use]
extern crate criterion;

use std::path::PathBuf;

use criterion::Criterion;

use rusty_nets::network::reader::*;


fn bench_fb_forum_read(c: &mut Criterion) {
    c.bench_function("fb-forum read", move |b| b.iter(|| {
        let reader = EdgeListReader::new(',');
        let path: PathBuf = "resources/nets/fb-forum.txt".parse().unwrap();
        reader.read(path).unwrap()
    }));
}

criterion_group!(benches, bench_fb_forum_read);
criterion_main!(benches);