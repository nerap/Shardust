# Brokust

Brokust is a message broker written in Rust, designed to be a lightweight and efficient alternative to Kafka. It features ZooKeeper-like coordination, segment storage, partition management, and more.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Installation](#installation)
  - [Usage](#usage)
- [Architecture](#architecture)
  - [Broker](#broker)
  - [Segments](#segments)
  - [Partitions](#partitions)
  - [ZooKeeper-like Coordination](#zookeeper-like-coordination)
- [Contributing](#contributing)
- [License](#license)


## Introduction

Brokust is a distributed message broker implemented in Rust. Inspired by Kafka, it aims to provide a high-throughput, low-latency platform for handling real-time data feeds.

## Features

- **Broker Management**: Create and manage topics, publish and consume messages.
- **Segment Storage**: Efficient storage management using segments.
- **Partition Management**: Distribute load across multiple partitions.
- **ZooKeeper-like Coordination**: Manage cluster state and configurations.

### Installation

To install Brokust, clone the repository and build the project using Cargo.

```sh
git clone https://github.com/yourusername/brokust.git
cd brokust
cargo build --release
```

### Usage
To start using Brokust, you can run the broker and interact with it via the provided commands or integrate it into your applications.

## Architecture

Classic message broker architure

![image](https://github.com/nerap/Brokust/assets/44852526/ea18ba15-c8c5-4194-8c0d-b51838c4bc71)

### Hexagonnal 

![image](https://github.com/nerap/brokust/assets/44852526/290979ba-f225-4279-8b15-cf8981cdcc13)

### Broker
The broker is the central component responsible for managing topics, partitions, and handling client requests.

### Segments
Segments are used for efficient storage management. Messages are appended to segments, and segments are rolled over based on size or time policies.

### Partitions
Topics are divided into partitions to allow parallel processing. Each partition can be hosted on a different broker node, providing scalability and fault tolerance.

### ZooKeeper-like Coordination
Brokust uses a coordination mechanism similar to ZooKeeper for managing the cluster state, leader election, and configuration management.
