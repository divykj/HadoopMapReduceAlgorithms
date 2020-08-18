# Hadoop MapReduce Algorithms

Set of algorithms implemented for Hadoop MapReduce using `mapper` and `reducer`
scripts in `python` and `rust`.

## Usage

### With Hadoop

Run scripts with Hadoop 3 MapReduce streaming:

```shell
❯ mapred streaming \
  -input <INPUT> \
  -mapper <MAPPER_SCRIPT> \
  -reducer <REDUCER_SCRIPT> \
  -output <OUTPUT>
```

### Without Hadoop

Run scripts with Unix command shimming:

```shell
❯ cat <INPUT> \
  | ./<MAPPER_SCRIPT> \
  | sort -k1,1 \
  | ./<REDUCER_SCRIPT> \
  > <OUTPUT>
```

### Using Python Scripts

Running python scripts:

```shell
❯ mapred streaming \
  -input <INPUT> \
  -mapper ./mapper.py \
  -reducer ./reducer.py \
  -output <OUTPUT>
```

### Using Rust Scripts

Running rust scripts:

```shell
❯ cargo build --release

❯ mapred streaming \
  -input <INPUT> \
  -mapper ./target/release/<SCRIPT>_mapper \
  -reducer ./target/release/<SCRIPT>_reducer \
  -output <OUTPUT>
```
