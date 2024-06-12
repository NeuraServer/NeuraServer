#!/bin/bash

# Download Kafka
curl -O http://apache.mirrors.tds.net/kafka/2.8.0/kafka_2.13-2.8.0.tgz
tar -xzf kafka_2.13-2.8.0.tgz
cd kafka_2.13-2.8.0

# Start Zookeeper
bin/zookeeper-server-start.sh config/zookeeper.properties &

# Start Kafka
bin/kafka-server-start.sh config/server.properties &

# Create Kafka topic
bin/kafka-topics.sh --create --topic data_pipeline --bootstrap-server localhost:9092 --replication-factor 1 --partitions 1
