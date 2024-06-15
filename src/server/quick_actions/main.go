package main

import (
	"fmt"
	"net"
	"sync"
	"time"
)

var (
	running = true
	mtx     sync.Mutex
	cv      = sync.NewCond(&mtx)
)

func liveMonitor() {
	for running {
		time.Sleep(1 * time.Second)
		mtx.Lock()
		fmt.Println("Live monitoring the server status...")
		mtx.Unlock()
	}
}

func checkServer(ip string, port int) {
	address := fmt.Sprintf("%s:%d", ip, port)
	conn, err := net.DialTimeout("tcp", address, 2*time.Second)
	if err != nil {
		fmt.Printf("Server %s:%d is DOWN: %s\n", ip, port, err)
		return
	}
	defer conn.Close()
	fmt.Printf("Server %s:%d is UP\n", ip, port)
}

func main() {
	// Start live monitoring in a separate goroutine
	go liveMonitor()

	// Check server status
	ip := "127.0.0.1"
	port := 8080
	checkServer(ip, port)

	// Simulate running for a certain duration
	time.Sleep(10 * time.Second)

	// Stop the live monitor
	mtx.Lock()
	running = false
	cv.Signal()
	mtx.Unlock()
}
