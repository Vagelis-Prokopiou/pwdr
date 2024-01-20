# Password Repeater

A command line utility for repeating a base password.

Other manipulations that are currently supported are alternating the direction of the password.

## Examples:

```shell
pwdr --password abc45 # returns abc45
pwdr --password abc45 --repeat 3 # returns abc45abc45abc45
pwdr --password abc45 --repeat 3 --alternate-direction # returns abc4554cbaabc45
```

## Building the project

You need [Rust](https://www.rust-lang.org/tools/install) in order to build the project.

```shell
git clone git@github.com:Vagelis-Prokopiou/pwdr.git;
cd pwdr;
cargo build --release;
```
