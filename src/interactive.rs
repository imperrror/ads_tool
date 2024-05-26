pub mod interactive {
    use std::fs::File;
    use std::io;
    use std::io::{BufRead, BufReader};

    ///Reads partitions names from /proc/partitions
    #[cfg(target_os = "linux")]
    fn linux_read_partitions() -> Result<Vec<String>, io::Error> {
        const PARTITION_PATH: &str = "/proc/partitions";

        let file: File = File::open(PARTITION_PATH)?;
        let buffer = BufReader::new(file);
        let mut partitions: Vec<String> = Vec::new();

        for line in buffer.lines().skip(2) {
            let record = line?.trim().to_string();
            let partition_name = record
                .split_whitespace()
                .skip(3)
                .next()
                .ok_or(io::Error::other("Failed to read partition name"))?;
            partitions.push(String::from(partition_name));
        }

        Ok(partitions)
    }

}