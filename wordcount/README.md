# Word Count

Counts the number of times each word occurs in the entire text corpus.

## Testing python script

```shell
❯ cat data/* \
  | ./python/mapper.py \
  | sort -k1,1 \
  | ./python/reducer.py \
  > output.txt
```

## Testing rust script

```shell
❯ cd rust && cargo build --release && cd ..

❯ cat data/* \
  | ./rust/target/release/wordcount_mapper \
  | sort -k1,1 \
  | ./rust/target/release/wordcount_reducer \
  > output.txt
```
