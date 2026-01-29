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


