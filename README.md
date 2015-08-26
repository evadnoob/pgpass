stuck on lifetime issues:


```
❯ cargo build
   Compiling read-pgpass v0.1.0 (file:///Users/daveboon/Projects/read-pgpass)
src/bin/read-pgpass.rs:59:16: 59:21 error: `(line:core::result::Result::Ok).0` does not live long enough
src/bin/read-pgpass.rs:59             Ok(ref l) => {
                                         ^~~~~
note: in expansion of for loop expansion
src/bin/read-pgpass.rs:55:5: 92:6 note: expansion site
src/bin/read-pgpass.rs:36:138: 100:2 note: reference must be valid for the lifetime 'a as defined on the block at 36:137...
src/bin/read-pgpass.rs:36 fn read_pgpass_file<'a>(path_to_pgpass: &str, username: &str, hostname: &str, port: &str, database: &str) -> Option<&'a PgPassEntry<'a>> {
src/bin/read-pgpass.rs:37
src/bin/read-pgpass.rs:38     let path = Path::new(path_to_pgpass);
src/bin/read-pgpass.rs:39     let display = path.display();
src/bin/read-pgpass.rs:40
src/bin/read-pgpass.rs:41     // Open the path in read-only mode, returns `io::Result<File>`
                          ...
src/bin/read-pgpass.rs:55:9: 55:13 note: ...but borrowed value is only valid for the for at 55:8
src/bin/read-pgpass.rs:55     for line in reader.lines() {
                                  ^~~~
note: in expansion of for loop expansion
src/bin/read-pgpass.rs:55:5: 92:6 note: expansion site
src/bin/read-pgpass.rs:96:5: 96:19 error: `pgpass_entries` does not live long enough
src/bin/read-pgpass.rs:96     pgpass_entries.iter().find(|&x| x.hostname == hostname
                              ^~~~~~~~~~~~~~
src/bin/read-pgpass.rs:36:138: 100:2 note: reference must be valid for the lifetime 'a as defined on the block at 36:137...
src/bin/read-pgpass.rs:36 fn read_pgpass_file<'a>(path_to_pgpass: &str, username: &str, hostname: &str, port: &str, database: &str) -> Option<&'a PgPassEntry<'a>> {
src/bin/read-pgpass.rs:37
src/bin/read-pgpass.rs:38     let path = Path::new(path_to_pgpass);
src/bin/read-pgpass.rs:39     let display = path.display();
src/bin/read-pgpass.rs:40
src/bin/read-pgpass.rs:41     // Open the path in read-only mode, returns `io::Result<File>`
                          ...
src/bin/read-pgpass.rs:50:59: 100:2 note: ...but borrowed value is only valid for the block suffix following statement 3 at 50:58
src/bin/read-pgpass.rs:50     let mut pgpass_entries: Vec<PgPassEntry> = Vec::new();
src/bin/read-pgpass.rs:51
src/bin/read-pgpass.rs:52     let mut reader = std::io::BufReader::new(&file);
src/bin/read-pgpass.rs:53     let re = Regex::new(r":").unwrap();
src/bin/read-pgpass.rs:54
src/bin/read-pgpass.rs:55     for line in reader.lines() {
                          ...
error: aborting due to 2 previous errors
Could not compile `read-pgpass`.

``
