# partition-numbers

Finds the partitions of a number.

A partition of a non-negative integer *n* is a set of positive integers that sum to n.
The number of partitions is given by [https://oeis.org/A000041](https://oeis.org/A000041).

## Example

```rust
use partition_numbers::get_partitions;

fn main() {
    for i in 0..=6 {
        let partitions = get_partitions(i);

        let plural = if partitions.len() == 1 { "" } else { "s" };
        println!("{} has {} partition{}", i, partitions.len(), plural);

        for partition in partitions {
            let representation = if !partition.is_empty() {
                let strings: Vec<String> = partition.iter().map(|n| format!("{}", n)).collect();
                strings.join(" + ")
            } else {
                "0 (empty partition)".to_string()
            };
            println!("  {} = {}", i, representation);
        }
    }
}
```

Output:
```text
0 has 1 partition
  0 = 0 (empty partition)
1 has 1 partition
  1 = 1
2 has 2 partitions
  2 = 1 + 1
  2 = 2
3 has 3 partitions
  3 = 1 + 1 + 1
  3 = 1 + 2
  3 = 3
4 has 5 partitions
  4 = 1 + 1 + 1 + 1
  4 = 1 + 1 + 2
  4 = 1 + 3
  4 = 2 + 2
  4 = 4
5 has 7 partitions
  5 = 1 + 1 + 1 + 1 + 1
  5 = 1 + 1 + 1 + 2
  5 = 1 + 1 + 3
  5 = 1 + 2 + 2
  5 = 1 + 4
  5 = 2 + 3
  5 = 5
6 has 11 partitions
  6 = 1 + 1 + 1 + 1 + 1 + 1
  6 = 1 + 1 + 1 + 1 + 2
  6 = 1 + 1 + 1 + 3
  6 = 1 + 1 + 2 + 2
  6 = 1 + 1 + 4
  6 = 1 + 2 + 3
  6 = 1 + 5
  6 = 2 + 2 + 2
  6 = 2 + 4
  6 = 3 + 3
  6 = 6
```

## License

Licensed under either of

* Apache License, Version 2.0
* MIT license

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
