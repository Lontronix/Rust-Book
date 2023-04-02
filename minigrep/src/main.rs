use std::env;

fn main() {
    let args:Vec<String> = env::args().collect();

    match minigrep::parse_config(&args) {
        Ok(config) => {
            if let Err(e) = minigrep::run(config) {
                println!("Application error: {e}");
            }
        },
        Err(error) => {
            match error {
                minigrep::ParseConfigError::TooFewArgsError => println!("Too few arguments specified")
            }
        }
    }
}
