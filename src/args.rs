/// Command-line argument parsing
///
/// Using clap as a CLI parser to manage command-line arguments
/// Reference: Java ProducerArgs.java
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "kafka_pub_cli")]
#[command(about = "Publish files from a directory to Kafka", long_about = None)]
pub struct ProducerArgs {
    /// identifies the product working w/ Kafka
    #[arg(long = "client.id", default_value = "kafkautil.rust.producer")]
    pub client_id: String,

    /// the Kafka topic
    #[arg(long = "topic", required = true)]
    pub topic: String,

    /// comma-separated list of kafka brokers (host:port)
    #[arg(long = "bootstrap-server", required = true)]
    pub bootstrap: String,

    /// how many replicas must receive message (0, 1, all)
    #[arg(long = "acks", required = true)]
    pub ack_mode: String,

    /// how many times failures will be retried
    #[arg(long = "retries", default_value_t = 0)]
    pub retries: i32,

    /// the delay in ms between retries (retry.backoff.ms)
    #[arg(long = "retryDelays", default_value_t = 100)]
    pub retry_delay: i32,

    /// the number of batches on a connection that can be sent to broker without a response
    #[arg(long = "maxInflight", default_value_t = 1)]
    pub max_inflight: i32,

    /// the maximum size in bytes of the buffer used to batch messages before sending to Kafka (batch.size)
    #[arg(long = "batchSizeBytes", default_value_t = 16_384)]
    pub batch_size_bytes: i32,

    /// the delay in ms that producer will wait for buffer to be filled (linger.ms)
    #[arg(long = "batchDelay", default_value_t = 0)]
    pub batch_delay: i32,

    /// app will connect to the broker in a secure way
    #[arg(long = "isSecure", default_value_t = false)]
    pub is_secure: bool,

    /// the security protocol used to communicate w/ brokers
    #[arg(long = "securityProtocol")]
    pub security_protocol: Option<String>,

    /// SASL mechanism
    #[arg(long = "saslMechanism")]
    pub sasl_mechanism: Option<String>,

    /// SASL JaaS config
    #[arg(long = "saslJaasConfig")]
    pub sasl_jaas_config: Option<String>,

    /// the format of the trust store
    #[arg(long = "trustStoreType")]
    pub trust_store_type: Option<String>,

    /// the path to the trust store
    #[arg(long = "trustStoreLocation")]
    pub truststore_location: Option<String>,

    /// the password for the trust store
    #[arg(long = "trustStorePassword")]
    pub truststore_password: Option<String>,

    //
    // CLI args related to where payload files are located
    //
    /// directory where files are located that will be published to topic
    #[arg(long = "messageLocation", required = true)]
    pub message_location: String,

    /// how long to wait between file polls looking for new messages
    #[arg(long = "delayInMillis", default_value_t = 1000)]
    pub delay_millis: u64,

    /// only poll messageLocation once when this parameter is added
    #[arg(long = "runOnce", default_value_t = false)]
    pub run_once: bool,

    /// app will delete the files after a poll unless this parameter is added
    #[arg(long = "noDeleteFiles", default_value_t = false)]
    pub no_delete_files: bool,
}
