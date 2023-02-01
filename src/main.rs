use std::io::Error;
use std::io::{stdin, prelude};
use std::process::exit;
use std::fs::{create_dir, create_dir_all, File};
use std::path::{Path, PathBuf};
use whoami::username;
const VERSION: &str = "0.1.1 2023-01-26";

pub struct Configuration {
    pub default_path: PathBuf,
    pub config_path: PathBuf,
}

impl Configuration {
    pub fn new(default_path: PathBuf, config_path: PathBuf) -> Configuration {
        Configuration { default_path, config_path }
    }
    pub fn set_default_path(&mut self, default_path: PathBuf) {
        self.default_path = default_path;
    }
    pub fn get_default_path(&self) -> &PathBuf {
        &self.default_path
    }
    pub fn get_config_path(&self) -> &PathBuf {
        &self.config_path
    }
    pub fn write_config(&self) -> Result<(), Error>{
        todo!();
    }
    pub fn read_config(&self) -> Result<(), Error>{
        todo!();
    }
    pub fn create_config_file(&self, path: PathBuf) -> Result<(), Error> {
        // match path.try_exists() {
        //     Ok(true) =>
        //     Ok(false) =>
        //     Err(_) => eprintln!("Unable to verify if file exists.")
        // }
    }
}

fn main() {
    println!("v{}\n\n< Pretzel >\n", VERSION);
    println!("[ C ] Create new default project");
    println!("[ S ] Change the configuration");
    println!("[ X ] Exit");
    let mut config = Configuration::new(
        PathBuf::from(format!("C:\\Users\\{}\\Projects", username())),
        PathBuf::from(format!("C:\\Users\\{}\\.config", username()))
    );

    match read_selection("cCxXsS") {
        'c' | 'C' => create_project_default(config).expect("Error creating project"),
        's' | 'S' => show_configs(config),
        'x' | 'X' => exit(0),
        _ => println!("Error!"),
    };
}

fn read_selection(accepted_inputs:&str) -> char {
    let input_chars: Vec<char> = accepted_inputs.chars().collect();
    loop {
        let buffer_chars: Vec<char> = read_line().chars().collect();
        if input_chars.contains(&buffer_chars[0]) {
            return buffer_chars[0];
        }
        println!("Wrong input! Please input one of the displayed characters.");
    }
}

fn read_line() -> String {
    let mut buffer: String = "".to_string();
    stdin().read_line(&mut buffer)
        .expect("Error reading input");
    buffer
}

fn read_integer() -> i32 {
    loop {
        let number_string = read_line();
        match number_string.trim().parse() {
            Ok(num) => return num,
            Err(_) => eprintln!("Error reading number. Please input a valid number.")
        }
    }
}

pub fn show_configs(mut config: Configuration) {
    println!("Root folder path: {}", config.get_default_path().to_str().unwrap());
    println!("Configuration file path: {}", config.get_config_path().to_str().unwrap());
    println!("[ D ] Modify default path");
    println!("[ X ] Back");
    match read_selection("cCxX") {
        'd' | 'D' => {
            // config.set_default_path(PathBuf::from(read_line()));
            // config.write_config().expect("ERROR WRITING TO CONFIGURATION!");
            show_configs(config);
        },
        'x' | 'X' => main(),
        _ => println!("Error!"),
    }
}

fn create_project_default(config: Configuration) -> Result<(), Error> {
    println!("---Project creation---");
    println!("Where should the directory be?\nInput the path:");
    let mut project_path = PathBuf::from(config.default_path);
    project_path.push(read_line().trim());
    println!("{:?}", project_path);
    println!("How many topics will there be?\nInput a number:");
    let topic_number: i32 = read_integer();

    create_dir_all(&project_path)?;

    for topic in 1..=topic_number {
        let mut topic_path = project_path.clone();
        topic_path.push(format!("T_{}", topic));
        create_dir(&topic_path)?;
        let recordings_path = topic_path.join("Recordings");
        create_dir(&recordings_path)?;
        let theory_path = topic_path.join("Theory");
        create_dir(&theory_path)?;
        let a1_path = topic_path.join("A_1");
        create_dir(&a1_path)?;
    }
    println!("---End---");
    Ok(())
}

