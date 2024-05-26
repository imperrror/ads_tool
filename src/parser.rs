pub mod parser {
    use clap::Parser;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io;
    use std::io::Read;
    use jwalk::WalkDir;

    /// Program that should parse NTFS alternative data streams
    #[derive(Parser, Debug)]
    #[command(version, about, long_about = None)]
    pub struct Args {
        /// Interactive program mode
        #[arg(short, long, default_value_t = false)]
        pub(crate) interactive: bool,
        /// Root of NTFS path. Required if mode is not interactive
        #[arg(short, long, required_unless_present = "interactive")]
        root_dir: Option<String>,
        /// Output CSV-file path
        #[arg(short, long, default_value = None)]
        output_file: Option<String>,
        /// Verbose mode for printing program state in stdout
        #[arg(short, long, default_value_t = false)]
        verbose: bool,
        /// Excludes ADS by their name
        #[arg(short, long)]
        exclude_list: Vec<String>,
    }

    pub struct ParserSettings {
        root_dir: String,
        exclude_list: Vec<String>,
        verbose: bool
    }

    impl ParserSettings {
        pub fn from_args(args: Args) -> Result<ParserSettings, io::Error> {
            let root_dir = args.root_dir.ok_or(
                io::Error::other("Failed to getting root dir")
            )?;
            Ok(
                ParserSettings  {
                    root_dir,
                    exclude_list: args.exclude_list,
                    verbose: args.verbose,
                }
            )
        }
    }

    #[derive(Debug)]
    pub struct FileData {
        filepath: String,
        ads: HashMap<String, String>
    }

    pub fn parse_streams(settings: ParserSettings) -> Result<Vec<FileData>, io::Error> {
        if settings.verbose {
            println!(
                "Start filesystem scanning by \n\tRoot dir: {}\n\tExclude ADS: {}",
                settings.root_dir,
                settings.exclude_list.join(", ")
            )
        }
        let mut streams_with_data: Vec<FileData> = Vec::new();

        for entry in WalkDir::new(settings.root_dir) {

            let entry = entry?;

            if entry.file_type.is_dir() {
                continue;
            }

            if entry.path().components().any(
                |x| { x.as_os_str() == "System Volume Information" }
            ) { continue; }

            let mut streams: Vec<String> = Vec::new();
            let entry_path = entry.
                path().to_str()
                .ok_or(io::Error::other("Failed to cast path_str"))?.to_string();
            if settings.verbose {
                println!("File scanning: {}", entry_path);
            }
            if cfg!(target_os = "linux") {
                let file_streams_list = xattr::get(
                    entry_path.clone(), "ntfs.streams.list"
                )?;
                match file_streams_list {
                    Some(xattr_streams) => {
                        let mut buffer = String::new();
                        for value in  xattr_streams {
                            if value == 0 {
                                streams.push(buffer.trim().to_string());
                                buffer.clear();
                            }
                            else {
                                buffer.push(value as char);
                            }
                        }
                        let stream = buffer.trim().to_string();
                        if stream != "" {
                            streams.push(stream);
                        }
                        Some(())
                    },
                    _ => {None}
                };
            }

            if streams.len() == 0 {
                if settings.verbose {
                    println!("Not found streams in file: {}", entry_path);
                }
                continue;
            }
            let mut file = FileData {
                filepath: entry_path.clone(),
                ads: HashMap::new(),
            };
            for stream in streams {
                if settings.exclude_list.contains(&stream) {
                    continue;
                }
                let mut path_str = entry_path.clone();
                path_str.push_str(format!(":{}", stream).as_str());
                let mut content = String::from("");
                File::open(path_str)?.read_to_string(&mut content)?;
                file.ads.insert(stream, content);
            }
            streams_with_data.push(file);
        }

        Ok(streams_with_data)
    }

}