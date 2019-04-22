use structopt::StructOpt;

mod upload;

#[derive(StructOpt, Debug)]
#[structopt(name = "funkwhale-cli")]
enum Opt {
    #[structopt(name = "upload")]
    Upload {
        #[structopt(short = "i", long = "interactive")]
        interactive: bool,

        #[structopt(short = "f", long = "file", parse(from_os_str))]
        file: std::path::PathBuf,

        #[structopt(short = "l", long = "library")]
        library: Option<String>,

        #[structopt(short = "u", long = "instance-url")]
        instance_url: String,

        #[structopt(short = "t", long = "token-file", parse(from_os_str))]
        token_file: std::path::PathBuf,

        #[structopt(short = "m", long = "timeout", default_value = "500")]
        timeout: u64,
    },
}

fn parse_token_file(path: std::path::PathBuf) -> String {
    let file = std::fs::read_to_string(path).expect("Could not read the token file :(");
    return file.trim().to_string();
}

fn parse_file(file: std::path::PathBuf) -> Vec<std::path::PathBuf> {
    if !file.exists() {
        panic!("File or directory doesn't exist");
    }

    if file.is_dir() {
        return std::fs::read_dir(file)
            .unwrap()
            .map(|res| res.unwrap().path())
            .collect();
    } else {
        return vec![file];
    }

}

fn main() {
    let args = Opt::from_args();

    if let Opt::Upload { interactive, file, library, instance_url, token_file, timeout } = args {
        let token = parse_token_file(token_file);
        let all_files = parse_file(file);

        match upload::main(all_files, library, instance_url, token, interactive, timeout) {
            Ok(v) => println!("\nUpload successful!"),
            Err(e) => panic!("{}", e),
        }
    }
}
