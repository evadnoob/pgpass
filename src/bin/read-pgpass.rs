extern crate regex;

use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::env;
use std::io::prelude::*;
use regex::Regex;

#[derive(Debug, Clone)]
struct PgPassEntry {
    username: String,
    hostname: String,
    port: String,
    database: String,
    password: String
}

 fn main() {
    for argument in env::args() {
        println!("{}", argument);
    }
    
    match env::args().last() {
        Some(arg) => {
            println!("arg {}", arg);
            let pgpass_entry_match = read_pgpass_file("~/.pgpass", "auser", "a.host.name", "5432", "database");
            println!("pgpass_entry_match {:?}", pgpass_entry_match);
        },
        _ => {
            println!("missing required command line arguments.")
        }
    }
 }

fn read_pgpass_file(path_to_pgpass: &str, username: &str, hostname: &str, port: &str, database: &str) -> Option<PgPassEntry> {

    let path = Path::new(path_to_pgpass);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                           Error::description(&why)),
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

    pgpass_entries.iter().find(|&x| x.hostname == hostname
                                                        && x.database == database
                                                        && x.port == port
                                                        && x.username == username)
}
