stuck on lifetime issues:


```

❯ cargo build
   Compiling read-pgpass v0.1.0 (file:///Users/daveboon/Projects/read-pgpass)
src/bin/read-pgpass.rs:96:5: 99:83 error: mismatched types:
 expected `core::option::Option<PgPassEntry>`,
    found `core::option::Option<&PgPassEntry>`
(expected struct `PgPassEntry`,
    found &-ptr) [E0308]
src/bin/read-pgpass.rs:96     pgpass_entries.iter().find(|&x| x.hostname == hostname
src/bin/read-pgpass.rs:97                                                         && x.database == database
src/bin/read-pgpass.rs:98                                                         && x.port == port
src/bin/read-pgpass.rs:99                                                         && x.username == username)
src/bin/read-pgpass.rs:96:5: 99:83 help: run `rustc --explain E0308` to see a detailed explanation
error: aborting due to previous error
Could not compile `read-pgpass`.

``
