use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::path::PathBuf;
use std::process::Command;

fn bench_tpm(c: &mut Criterion) {
    let bin = env!("CARGO_BIN_EXE_rsomics-tpm");
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let counts = manifest.join("tests/golden/counts.tsv");
    let lengths = manifest.join("tests/golden/lengths.tsv");
    c.bench_function("rsomics-tpm golden", |b| {
        b.iter(|| {
            let out = Command::new(black_box(bin))
                .args([
                    counts.to_str().unwrap(),
                    "-l",
                    lengths.to_str().unwrap(),
                ])
                .output()
                .unwrap();
            assert!(out.status.success());
        });
    });
}

criterion_group!(benches, bench_tpm);
criterion_main!(benches);
