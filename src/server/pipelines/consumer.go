package main

import (
    "context"
    "fmt"
    "log"
    "os"
    "github.com/segmentio/kafka-go"
)

const (
    kafkaBroker = "127.0.0.1:9092"
    topic       = "data_pipeline"
    groupID     = "data_pipeline_group"
)

func main() {
    r := kafka.NewReader(kafka.ReaderConfig{
        Brokers:  []string{kafkaBroker},
        Topic:    topic,
        GroupID:  groupID,
        MinBytes: 10e3, // 10KB
        MaxBytes: 10e6, // 10MB
    })

    file, err := os.OpenFile("data/output.txt", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
    if err != nil {
        log.Fatal("could not open output file " + err.Error())
    }
    defer file.Close()

    for {
        m, err := r.FetchMessage(context.Background())
        if err != nil {
            log.Fatal("could not read message " + err.Error())
        }
        fmt.Printf("Received: %s\n", string(m.Value))
        if _, err := file.WriteString(fmt.Sprintf("%s\n", m.Value)); err != nil {
            log.Fatal("could not write to output file " + err.Error())
        }

        if err := r.CommitMessages(context.Background(), m); err != nil {
            log.Fatal("could not commit message " + err.Error()))
        }
    }
}
