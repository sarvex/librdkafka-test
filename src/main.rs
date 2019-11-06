use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::get_rdkafka_version;
use rdkafka::message::OwnedHeaders;

fn produce(brokers: &str, topic_name: &str) {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");


    // This loop is non blocking: all messages will be sent one after the other, without waiting
    // for the results.
    let futures = (0..5)
        .map(|i| {
            // The send operation on the topic returns a future, that will be completed once the
            // result or failure from Kafka will be received.
            producer
                .send(
                    FutureRecord::to(topic_name)
                        .payload(&format!("Message {}", i))
                        .key(&format!("Key {}", i))
                        .headers(OwnedHeaders::new().add("header_key", "header_value")),
                    0,
                )
        })
        .collect::<Vec<_>>();
}

fn main() {
    produce("toto", "tata")
}
