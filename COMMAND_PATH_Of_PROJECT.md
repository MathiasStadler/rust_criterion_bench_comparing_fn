
# Command path of project

## used code on ubuntu

```bash
uname -a
Linux trapapa-ThinkPad-T430 5.19.0-46-generic 
22.04.1-Ubuntu SMP PREEMPT_DYNAMIC 
Wed Jun 21 15:35:31 UTC 2 
x86_64 x86_64 x86_64 GNU/Linux

```

## code

```bash
code --version
1.80.0
660393deaaa6d1996740ff4880f1bad43768c814
x64

code --list-extensions --show-versions
belfz.search-crates-io@1.2.1
bungcip.better-toml@0.3.2
DavidAnson.vscode-markdownlint@0.51.0
donjayamanne.python-environment-manager@1.0.4
formulahendry.code-runner@0.12.0
foxundermoon.shell-format@7.2.5
MS-CEINTL.vscode-language-pack-de@1.80.2023070509
ms-python.isort@2023.10.0
ms-python.python@2023.12.0
ms-python.vscode-pylance@2023.7.10
ms-toolsai.jupyter@2023.6.1001861915
ms-toolsai.jupyter-keymap@1.1.2
ms-toolsai.jupyter-renderers@1.0.17
ms-toolsai.vscode-jupyter-cell-tags@0.1.8
ms-toolsai.vscode-jupyter-slideshow@0.1.5
rust-lang.rust-analyzer@0.3.1583
streetsidesoftware.code-spell-checker@2.20.5
tomoki1207.pdf@1.2.2
vadimcn.vscode-lldb@1.9.2
```

# rust update

```bash
rustup update

`stable-x86_64-unknown-linux-gnu updated - rustc 1.70.0 (90c541806 2023-05-31) (from rustc 1.69.0 (84c898d65 2023-04-16))``
```

# rustup -V

```bash
rustup -V
rustup 1.26.0 (5af9b9484 2023-04-05)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.70.0 (90c541806 2023-05-31)`
```

## call website via firefox

```bash
export DISPLAY=:0
echo $DISPLAY
WEB_SITE_LINK="https://bheisler.github.io/criterion.rs/book/getting_started.html#getting-started"
firefox $WEB_SITE_LINK &
```

## create project

```bash
mkdir rust_Criterion_bench_comparing_fn && $_
cargo init --lib .
```

> ``` $_ ``` to retrieve the last argument of the previous command instead

## Create *.md

```bash
touch README.md
touch FROME_HERE.md
touch COMMAND_PATH_Of_PROJECT.md <= THIS FILE
```

## Add Dependency to Cargo.toml

```bash
cargo add --dev criterion
```

---

# Tutorial start now

## delete file from template

```bash
ls src/lib.rs && rm $_
```

## create new lib.rs with content

```bash
tee -a  ./src/lib.rs << END 
#[inline]
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}
END
```

## Add bench Dependency to Cargo.toml

```bash
tee -a  ./Cargo.toml <<  END 

[[bench]]
name = "my_benchmark"
harness = false

END
```

## create benches subfolder

```bash
mkdir ./benches
```

## Add Benchmark

```bash
tee -a  ./benches/my_benchmark.rs << END
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_criterion_bench_comparing_fn::fibonacci;
//use mycrate::fibonacci;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

END
```
