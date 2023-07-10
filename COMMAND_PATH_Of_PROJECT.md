
# Command path of project


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

## create README.md and FROM_HERE.md files

```bash
touch README.md
touch FROME_HERE.md
touch COMMAND_PATH_Of_PROJECT.md
```

## add necessary libs

```bash
cargo add --dev criterion
```

## 