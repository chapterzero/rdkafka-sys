use rdkafka_sys::kafka::config::Config;
use std::{thread, time};

fn main() {
    let mut cons_conf = Config::new();
    cons_conf.set("bootstrap.servers", "127.0.0.1:9092")
        .set("group.id", "rdkafka_sysv1");

    let mut prod_conf = Config::new();
        prod_conf.set("bootstrap.servers", "127.0.0.1:9092");

    println!("Running...");
    run(cons_conf, prod_conf);

    println!("Finished");
    thread::sleep(time::Duration::from_secs(10));
}

fn run(cons_conf: Config, prod_conf: Config) {
    let mut consumer = cons_conf.build_consumer().unwrap();
    consumer.subscribe(&vec!["some_topic", "2nd_topic"]);

    let mut producer = prod_conf.build_producer().unwrap();
    producer.set_topics(&["some_topic", "2nd_topic"]);
    let t = thread::Builder::new()
        .name(String::from("producer_thread"));
    let handle = t.spawn(move||{
        for i in 1..=5 {
            let payload = format!("Message: {}", i);
            producer.send(payload.as_bytes(), "some_topic", None).unwrap();

            let payload2 = format!("Message 2: {}", i);
            producer.send(payload2.as_bytes(), "2nd_topic", None).unwrap();
            thread::sleep(time::Duration::from_secs(1));
        }
        producer.flush(1000);
    }).unwrap();

    for _ in 0..7 {
        println!("Waiting for message...");
        for message in consumer.get_messages(1000).unwrap() {
            let m = message.unwrap();
            println!("Message: {}", m.read().unwrap());
        }
        thread::sleep(time::Duration::from_secs(1));
    }

    handle.join().unwrap();
}
