package main

import (
    "fmt"
    "log"
    "os"
    "strings"
    "time"

    "github.com/confluentinc/confluent-kafka-go/kafka"
    "github.com/samuel/go-zookeeper/zk"
)

func main() {
    // Connect to ZooKeeper
    zookeeperServers := []string{"localhost:2181"}
    conn, _, err := zk.Connect(zookeeperServers, time.Second*10)
    if err != nil {
        log.Fatalf("Failed to connect to ZooKeeper: %v", err)
    }
    defer conn.Close()

    // Set up Kafka producer
    kafkaConfig := &kafka.ConfigMap{"bootstrap.servers": "localhost:9092"}
    producer, err := kafka.NewProducer(kafkaConfig)
    if err != nil {
        log.Fatalf("Failed to create Kafka producer: %v", err)
    }
    defer producer.Close()

    topic := "neura_topic"
    message := "Hello, this is a message from NeuraServer!"

    // Produce message to Kafka
    deliveryChan := make(chan kafka.Event)
    err = producer.Produce(&kafka.Message{
        TopicPartition: kafka.TopicPartition{Topic: &topic, Partition: kafka.PartitionAny},
        Value:          []byte(message),
    }, deliveryChan)
    if err != nil {
        log.Fatalf("Failed to produce message: %v", err)
    }

    e := <-deliveryChan
    m := e.(*kafka.Message)
    if m.TopicPartition.Error != nil {
        log.Fatalf("Failed to deliver message: %v", m.TopicPartition.Error)
    } else {
        fmt.Printf("Message delivered to %v\n", m.TopicPartition)
    }
    close(deliveryChan)

    // Consume messages from Kafka
    consumer, err := kafka.NewConsumer(&kafka.ConfigMap{
        "bootstrap.servers": "localhost:9092",
        "group.id":          "neura_group",
        "auto.offset.reset": "earliest",
    })
    if err != nil {
        log.Fatalf("Failed to create Kafka consumer: %v", err)
    }
    defer consumer.Close()

    err = consumer.Subscribe(topic, nil)
    if err != nil {
        log.Fatalf("Failed to subscribe to topic: %v", err)
    }

    for {
        msg, err := consumer.ReadMessage(-1)
        if err == nil {
            fmt.Printf("Received message: %s\n", string(msg.Value))
        } else {
            fmt.Printf("Consumer error: %v (%v)\n", err, msg)
        }
    }
}
