package main

import (
    "encoding/json"
    "fmt"
    "log"
    "net/http"
    "time"

    "github.com/confluentinc/confluent-kafka-go/kafka"
    "github.com/gorilla/mux"
    "github.com/samuel/go-zookeeper/zk"
)

var (
    producer *kafka.Producer
    consumer *kafka.Consumer
    conn     *zk.Conn
)

func main() {
    // Initialize ZooKeeper connection
    zookeeperServers := []string{"localhost:2181"}
    var err error
    conn, _, err = zk.Connect(zookeeperServers, time.Second*10)
    if err != nil {
        log.Fatalf("Failed to connect to ZooKeeper: %v", err)
    }
    defer conn.Close()

    // Initialize Kafka producer
    kafkaConfig := &kafka.ConfigMap{"bootstrap.servers": "localhost:9092"}
    producer, err = kafka.NewProducer(kafkaConfig)
    if err != nil {
        log.Fatalf("Failed to create Kafka producer: %v", err)
    }
    defer producer.Close()

    // Initialize Kafka consumer
    consumer, err = kafka.NewConsumer(&kafka.ConfigMap{
        "bootstrap.servers": "localhost:9092",
        "group.id":          "neura_group",
        "auto.offset.reset": "earliest",
    })
    if err != nil {
        log.Fatalf("Failed to create Kafka consumer: %v", err)
    }
    defer consumer.Close()

    // Set up HTTP server
    r := mux.NewRouter()
    r.HandleFunc("/produce", produceHandler).Methods("POST")
    r.HandleFunc("/consume", consumeHandler).Methods("GET")
    http.Handle("/", r)

    fmt.Println("Server is running at http://localhost:8080")
    log.Fatal(http.ListenAndServe(":8080", nil))
}

func produceHandler(w http.ResponseWriter, r *http.Request) {
    var req struct {
        Topic   string `json:"topic"`
        Message string `json:"message"`
    }
    err := json.NewDecoder(r.Body).Decode(&req)
    if err != nil {
        http.Error(w, "Invalid request payload", http.StatusBadRequest)
        return
    }

    err = producer.Produce(&kafka.Message{
        TopicPartition: kafka.TopicPartition{Topic: &req.Topic, Partition: kafka.PartitionAny},
        Value:          []byte(req.Message),
    }, nil)
    if err != nil {
        http.Error(w, "Failed to produce message", http.StatusInternalServerError)
        return
    }

    w.WriteHeader(http.StatusOK)
    w.Write([]byte("Message produced successfully"))
}

func consumeHandler(w http.ResponseWriter, r *http.Request) {
    topic := r.URL.Query().Get("topic")
    if topic == "" {
        http.Error(w, "Topic is required", http.StatusBadRequest)
        return
    }

    err := consumer.Subscribe(topic, nil)
    if err != nil {
        http.Error(w, "Failed to subscribe to topic", http.StatusInternalServerError)
        return
    }

    msg, err := consumer.ReadMessage(-1)
    if err != nil {
        http.Error(w, "Failed to consume message", http.StatusInternalServerError)
        return
    }

    w.Header().Set("Content-Type", "application/json")
    json.NewEncoder(w).Encode(struct {
        Message string `json:"message"`
    }{
        Message: string(msg.Value),
    })
}
