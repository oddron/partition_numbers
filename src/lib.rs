use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug)]
struct PartitionCache {
    partition_map: HashMap<(usize, usize), Vec<Vec<usize>>>,
    zero: Vec<Vec<usize>>,
}

impl PartitionCache {
    fn new() -> Self {
        Self {
            partition_map: HashMap::new(),
            zero: vec![Vec::new()], // Special case: This is the only partition of zero.
        }
    }
}

impl PartitionCache {
    // Gets all partitions of a number.
    //
    // A partition of a non-negative integer *n* is a set of positive integers that add to n.
    fn partitions(&mut self, total: usize) -> &Vec<Vec<usize>> {
        if total == 0 { // Special case
            return &self.zero;
        }
        self.partitions_with_floor(total, 1).unwrap()
    }

    // Gets the partitions of a number, with the constraint that
    // every addend is at least as large as the given floor.
    fn partitions_with_floor(&mut self, total: usize, floor: usize) -> Option<&Vec<Vec<usize>>> {
        if total < floor {
            return None;
        }

        if !self.partition_map.contains_key(&(total, floor)) {
            self.calc_partitions(total, floor);
        }

        self.partition_map.get(&(total, floor))
    }

    // Actually calculates the partitions of a number, with the given constraint.
    fn calc_partitions(&mut self, total: usize, floor: usize) {
        if total == floor {
            // There is only one possible partition.
            self.partition_map.insert((total, floor), vec![vec![total]]);
            return;
        }

        let mut results = Vec::new();

        // total = floor + { partitions of the remainder, with the same constraint }
        if let Some(ps) = self.partitions_with_floor(total - floor, floor) {
            for p in ps {
                let mut current = vec![floor];
                current.append(&mut p.clone());
                results.push(current);
            }
        }

        // total = { partitions with a larger minimum addend }
        if let Some(ps) = self.partitions_with_floor(total, floor + 1) {
            for p in ps {
                results.push(p.clone());
            }
        }

        self.partition_map.insert((total, floor), results.clone());
    }
}

static PARTITIONS: Mutex<Option<PartitionCache>> = Mutex::new(None);

/// Gets all partitions of a number.
///
/// A partition of a non-negative integer *n* is a set of positive integers that add to n.
pub fn get_partitions(total: usize) -> Vec<Vec<usize>> {
    let mut cache = PARTITIONS.lock().unwrap();

    if cache.is_none() {
        *cache = Some(PartitionCache::new());
    }

    cache.as_mut().unwrap().partitions(total).clone()
}

#[cfg(test)]
mod tests {
    use super::get_partitions;

    #[test]
    fn it_works() {
        // number of partitions of n, starting with n = 0
        // https://oeis.org/A000041
        let expected_counts: Vec<usize> = vec![
            1, 1, 2, 3, 5, 7, 11, 15, 22, 30, 42, 56, 77, 101, 135, 176, 231, 297, 385, 490, 627,
            792, 1002, 1255, 1575, 1958, 2436, 3010, 3718, 4565, 5604,
        ];

        for (expected_total, expected_len) in expected_counts.iter().enumerate() {
            let partitions = get_partitions(expected_total);
            assert_eq!(partitions.len(), *expected_len);

            for partition in partitions {
                let total: usize = partition.iter().sum();
                assert_eq!(total, expected_total);
            }
        }
    }
}
