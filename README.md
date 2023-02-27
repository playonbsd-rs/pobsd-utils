## pobsd
pobsd is a tool written in Rust designed to interact with the PlayOnBSD
database that can be find [here](https://github.com/playonbsd/OpenBSD-Games-Database)

It makes use of both `plege(2)` and `unveil(2)`.

At the moment, it provides three functionalities:
- checking the database integrity (at the moment, in a minimal way)
- exporting the database in a different format (at the moment, only json)
- browsing the database (at the moment, only in read-only mode)


### Installing
At the moment, pobsd is not available as a crate.
Therefore, you need to clone this repository and install
it using `cargo install`. Make sure to update to your `$PATH`
to be able to use it (usually by adding `$HOME/.cargo/bin`).

### Checking the database integrity
For now, it boils down to checking if the parser can
parse the database and tells where the parser found
errors. 
```
$ pobsd check games.db 
> 356 games parsed without error
$ pobsd check faulty-games.db
> 356 games parsed
> Errors occured at lines 15, 24.
```
If there is an error, it should be fixed before using the
other functions of pobsd since this could lead to wrong
data being displayed or exported.

### Exporting the database
At the moment, the only format available is json.
```
$ pobsd export games.db games.json 
```

### Browsing
The last (but not the least) function is the browse function.
It provides a nice yet simple terminal interface for browsing
and searching the database.
[![asciicast](https://asciinema.org/a/563130.svg)](https://asciinema.org/a/563130)

### Credits
Reading [https://github.com/graymind75/passmng](https://github.com/graymind75/passmng) source
code helped me a lot to implement the TUI.

