# balance

[![CICD](https://github.com/lloydlobo/balance/actions/workflows/CICD.yml/badge.svg)](https://github.com/lloydlobo/balance/actions/workflows/CICD.yml)

> CLI tool to balance your budget 

![balance](https://github.com/lloydlobo/balance/blob/master/assets/demo.gif)

## Features

* Cross-platform
* Arbitrary shell commands are supported.

## Usage

`balance --help` will provide a menu of all available commands and optional arguments.

### Transaction

```sh
$ balance tx -a 300 -A "expense:grocery"
transaction = {
    date: None,
    amount: 300,
    account: "expense:grocery",
    offset_account: None,
    description: None,
}
$ balance tx -a 300 -A "expense:grocery" -d 20221231 -O "asset:cash_checking" -D "Weekly grocery"
transaction = {
    date: Some(20221231),
    amount: 300,
    account: "expense:grocery",
    offset_account: Some("asset:cash_checking"),
    description: Some("Weekly grocery"),
}
```

## Setup

### API

Add `SECRET` to the `.env` file as `<your SECRET key>` without `<`/`>`.

```bashls
<!-- BALANCE_API_KEY=<your SECRET key> -->
```

## Install

### From Cargo

TODO:

 ```sh
 cargo install balance
 ```

### Build from Source

Alternatively, clone this repo and do the following:

* If Rust is not installed on your machine, follow the instructions
  on how to do that here: <https://www.rust-lang.org/tools/install>
* run cargo build --release to compile the binary
* copy the /target/release/balance binary to /usr/bin
  or wherever your system maintains application binaries

```sh
git clone https://github.com/lloydlobo/balance.git
cd balance
cargo build --release
cargo install --path .
```

## Dev

### Build

#### Docker

```make
docker_image := "balance"
docker_container_name := 'balance1'
username := 'lloydlobo'

docker build -t {{docker_image}} .
docker run -dp 8080:3030 --rm --name {{docker_container_name}} {{docker_image}}
docker run -it --rm --name {{docker_container_name}} --entrypoint bin/bash {{docker_image}}
docker stop {{docker_container_name}}
docker logs -f {{docker_container_name}}
```

##### Publish with Docker

```make
docker tag {{docker_image}} {{username}}/{{docker_image}}
docker push {{username}}/{{docker_image}}
```

### Test

```sh
CARGO_LOG=error cargo test
```

## License

`balance` is dual-licensed under the terms of the MIT License and the Apache License 2.0.

See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files for details.
