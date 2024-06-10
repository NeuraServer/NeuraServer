package com.neuraserver.flink

import org.apache.flink.streaming.api.scala._
import org.apache.flink.streaming.api.windowing.time.Time

object DynamicDataProcessor {
  def main(args: Array[String]): Unit = {
    val env = StreamExecutionEnvironment.getExecutionEnvironment

    // Configure your source, e.g., reading from Kafka
    val source = env.socketTextStream("localhost", 9999)

    val processedStream = source
      .flatMap(_.split("\\s"))
      .map((_, 1))
      .keyBy(0)
      .timeWindow(Time.seconds(5))
      .sum(1)

    // Configure your sink, e.g., writing to a database or another sink
    processedStream.print()

    env.execute("Flink Dynamic Data Processor")
  }
}
