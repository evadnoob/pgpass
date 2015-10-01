extern crate regex;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::env;
use std::io::prelude::*;
use regex::Regex;
use std::fs;
    
#[derive(Debug, Clone)]
struct PgPassEntry {
    username: String,
    hostname: String,
    port: String,
    database: String,
    password: String
}

impl PgPassEntry {
    pub fn to_string(&self) -> String {
        format!("postgresql://user@hostname:port/{}", self.database)
    }
}

pub fn find_matching_pgpass_entry(connect_str: String, override_path_to_pgpass: Option<String>) {
    println!("override_path_to_pg_pass: {:?}", override_path_to_pgpass);

    let path_to_pgpass: Result<PathBuf, String> = match override_path_to_pgpass {
        Some(path_to_pgpass) => {
            Ok(PathBuf::from(path_to_pgpass))
        },
        _ => get_path_to_pgpass()
    };
    
    match path_to_pgpass {
        Ok(path_to_pgpass) => { 
            let pgpass_entry_match = read_pgpass_file(&path_to_pgpass.as_path(), &connect_str);
            println!("pgpass_entry_matchass_entry_match {:?}", pgpass_entry_match);
        },
        Err(e) => println!("uh oh {}", e)
    }
}

fn get_path_to_pgpass() -> Result<PathBuf, String> {

    let home_dir = env::home_dir().expect("unable to exract home environment variable.");
    println!("home_dir {:?}", home_dir);

    let mut path_to_pgpass = PathBuf::from(home_dir);
    path_to_pgpass.push(".pgpass");
    println!("path_to_pg_pass {:?}", path_to_pgpass);
    match fs::metadata(&path_to_pgpass) {
        Ok(md) => {
            println!("metdata {}", md.is_file());
            //let path_to_pgpass_as_path = &path_to_pgpass.as_path();
            Ok(path_to_pgpass)
        },
        Err(e) => {
            println!("{}", e);
            Err("unable to read .pgpass file".to_string())
        }
    }
}

//fn read_pgpass_file(path_to_pgpass: &str, username: &str, hostname: &str, port: &str, database: &str) -> Option<PgPassEntry> {
fn read_pgpass_file(path_to_pgpass: &Path, connection_string: &str) -> Option<PgPassEntry> {

    let path = Path::new(path_to_pgpass);
    let display = path.display();
    println!("display {}", display);
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}, {}", display,
                           Error::description(&why), why),
        Ok(file) => file,
    };

    let mut pgpass_entries: Vec<PgPassEntry> = Vec::new();
    
    let mut reader = std::io::BufReader::new(&file);
    let re = Regex::new(r":").unwrap();

    for line in reader.lines() {
        println!("line {:?}", line);

        match line {
            Ok(ref l) => {
                let mut parts = re.split(l);
                
                let host = parts.next().unwrap();
                println!("parts[0] {:?}", host);

                let port = parts.next().unwrap();
                println!("parts[1] {:?}", port);
                
                let database = parts.next().unwrap();
                println!("parts[2] {:?}", database);
                
                let username = parts.next().unwrap();
                println!("parts[3] {:?}", username);
                
                let password = parts.next().unwrap();
                println!("parts[4] {:?}", password);

                
                let mut parts_vec = re.split(l).collect::<Vec<&str>>();
                println!("parts_vec {:?}", parts_vec);

                pgpass_entries.push(PgPassEntry{
                    hostname: String::from(host),
                    port: String::from(port),
                    username: String::from(username),
                    database: String::from(database),
                    password:  String::from(password)});

                
            },
            Err(e) => println!("no matching line {}", e)
        }
    }

    //println!("pgpass_entries {:?}", pgpass_entries);

    // pgpass_entries.into_iter().find(|x| x.hostname == hostname
    //                                 && x.database == database
    //                                 && x.port == port
    //                                 && x.username == username)

           pgpass_entries.into_iter().find(|x|
                                    format!("postgresql://{}@{}:{}/{}", x.username, x.hostname, x.port, x.database) == connection_string)

        
}


#[test]
fn it_works() {
    match std::env::current_exe() {
        Ok(exe_path) => {
            println!("Path of this executable is: {}", exe_path.display());
            //expect a test file in the current directory

                //let pgpass_file = exe_path
                //let connect_str = "postgresql://user1@localhost:5432/database1".to_string();
            let x = find_matching_pgpass_entry("postgresql://user1@localhost:5432/database1".to_string(), Some("./pgpass_test_data".to_string()));
            println!("x: {:?}", x);
            
        },
        Err(e) => {
            println!("failed to get current exe path: {}", e);
            panic!("failed tests");
        }
    };
    
}
