# timestamp-benchmark

Benchmarking Rust's two main ways to get a timestamp that roughly represent now. 

- [SystemTime::now](https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.now)
- [Instant::now](https://doc.rust-lang.org/std/time/struct.Instant.html#method.now)

# Running

```
  cargo bench
```

The pretty output will be in `./target/criterion/report/index.html`

# tl;dr

On a M2 Macbook Pro Instant::now() is about 1ns slower than SystemTime::now(). This is about 6-7% faster, and in most cases shouldn't be an issue. However in a tight enough loop every nanosecond is precious!

![tl;dr](violin.svg)