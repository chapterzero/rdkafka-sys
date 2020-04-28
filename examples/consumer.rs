use rdkafka_sys::kafka::config::Config;

fn main() {
    let mut conf = Config::new();
    conf.set("bootstrap.servers", "127.0.0.1:9092")
        .set("group.id", "rdkafka_sysv1");

    let mut consumer = conf.build_consumer().unwrap();
    consumer.subscribe(&vec!["some_topic"]);
    loop {
        for message in consumer.get_messages(1000).unwrap() {
            println!("Message: {:?}", message);
        }
    }
}
