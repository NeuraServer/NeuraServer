package main

import (
    "bufio"
    "context"
    "fmt"
    "os"
    "time"
    "github.com/segmentio/kafka-go"
)

const (
    kafkaBroker = "127.0.0.1:9092"
    topic       = "data_pipeline"
)

func main() {
    writer := kafka.NewWriter(kafka.WriterConfig{
        Brokers:  []string{kafkaBroker},
        Topic:    topic,
        Balancer: &kafka.LeastBytes{},
    })

    file, err := os.Open("data/input.txt")
    if err != nil {
        panic("could not open input file " + err.Error())
    }
    defer file.Close()

    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        chunk := scanner.Text()
        err := writer.WriteMessages(context.Background(),
            kafka.Message{
                Key:   []byte(fmt.Sprintf("Key-%d", time.Now().UnixNano())),
                Value: []byte(chunk),
            },
        )
        if err != nil {
            panic("could not write message " + err.Error())
        }
        fmt.Println("Sent:", chunk)
    }

    if err := scanner.Err(); err != nil {
        panic("error reading input file " + err.Error())
    }

    writer.Close()
}
