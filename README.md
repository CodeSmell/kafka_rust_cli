# Kafka Publish Util

This utility is designed to be run from the CLI. It will monitor a specified directory and will publish each file in the directory to a Kafka topic. This CLI utility was built to enable easy testing of products that consume from Kafka topics. 

This version is based on [the Java version](https://github.com/CodeSmell/KafkaPubCLI) and is written in Rust. The primary goal was to experiment with Rust language on a practical but small application.

The project setup for this project:

```
kafka_rust_cli/
  ├── Cargo.toml
  ├── src/
  │   └── main.rs
  |   └── args.rs
  |   └── file.rs 
  |   └── content.rs
  |   └── kafka.rs
  └── tests/
```

## Overview 
| Rust file  | Description                        	      | Java class     |
|---------	 |-----------------------------------------   |--------------- |
| main.rs    | The Entry Point into the application	      | KafkaMain      |
| args.rs 	 | The values for the input params from CLI	  | ProducerArgs   |
| file.rs    | File Polling                               | DirectoryPollingService  |
| content.rs | Parses the File contents                   | KafkaContentHandler  |
| kafka.rs 	 | Kafka publishing utility                   | KafkaProducerUtil    |

## Build the Rust executable
Unlike Java, building the Rust project produces an executable artifact.

```
cargo build -q
```

To run the tests

```
cargo tests
```

To verify the format of the code 

```
cargo fmt --all -- --check
```

Catch common mistakes and improve your Rust code

```
cargo clippy --all-targets --all-features -- -D warnings
```

### Making life easier - Development Tools

These tools are **optional CLI utilities** for development

#### cargo-nextest - Better Test Reports
For improved test output and parallel test execution:

```
cargo install --locked cargo-nextest
```

Then run:
```
cargo nextest run
```

#### cargo-make - Task Runner
To combine multiple commands into single tasks, install `cargo-make`:

```
cargo install cargo-make
```

This allows us to create custom commands in `Makefile.toml`. The checked-in version has several useful commands that run multiple cargo commands together. This simplifies development and allows a comprehensive check on Rust code before creating a PR.

**Available tasks:**

| command                | Description                        	      | 
|---------------------	 |-----------------------------------------   |
| cargo make             | default will run pre-pr                    |
| cargo make check       | format and clippy only                     | 
| cargo make pre-pr      | format, clippy, build and nextest          |

### Running the Util
The default mode is to continually poll the directory (`messageLocation`) for files that should be published to Kafka. Once a file is published to the Kafka topic it will be deleted. 

Note: set the environment variable so info statements are printed

```
// zsh
export RUST_LOG=info

// fish
set -x RUST_LOG=info

// with program
RUST_LOG=info ./target/debug/kafka_pub_cli ...
```

If the user only wants to run the utility against the directory once then add the parameter (`runOnce`). 
If the user doesn't want to remove the file then add the parameter (`noDeleteFiles`).

```
RUST_LOG=info ./target/debug/kafka_pub_cli \
            --topic foo \
            --bootstrap-server localhost:9092 \
            --acks 1 \
            --messageLocation ~/dev/myKafkaFiles \
            --runOnce --noDeleteFiles
```

Use the `-h` parameter or review the `args.rs` file to see all of the available parameters. 