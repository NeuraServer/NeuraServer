#!/bin/bash

# Download Hadoop
curl -O https://downloads.apache.org/hadoop/common/hadoop-3.3.0/hadoop-3.3.0.tar.gz
tar -xzf hadoop-3.3.0.tar.gz
cd hadoop-3.3.0

# Setup Hadoop environment variables
export HADOOP_HOME=$(pwd)
export PATH=$PATH:$HADOOP_HOME/bin

# Start Hadoop
sbin/start-dfs.sh
sbin/start-yarn.sh

# Create HDFS directories for data pipeline
hdfs dfs -mkdir /data_pipeline
