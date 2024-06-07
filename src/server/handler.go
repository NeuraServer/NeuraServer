// ----------------------------------
// --- NeuraServer Copyright 2024 ---
// ----------------------------------

package main

import (
	"fmt"
	"net"
	"os"
	"os/exec"
)

func handleConnections(listener net.Listener) {
	for {
		conn, err := listener.Accept()
		if err != nil {
			fmt.Println("Error accepting connection:", err)
			continue
		}
		go handleRequest(conn)
	}
}

func handleRequest(conn net.Conn) {
	buf := make([]byte, 1024)
	for {
		n, err := conn.Read(buf)
		if err != nil {
			if err != os.EOF {
				fmt.Println("Error reading from connection:", err)
			}
			break
		}
		if _, err := conn.Write(buf[:n]); err != nil {
			fmt.Println("Error writing to connection:", err)
			break
		}
	}
}

func main() {
	cmd := exec.Command("./neuraserver")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	if err := cmd.Run(); err != nil {
		fmt.Println("Error running NeuraServer:", err)
	}
}


