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
