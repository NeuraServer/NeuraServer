package main

import (
    "encoding/json"
    "fmt"
    "log"
    "net/http"

    "github.com/colinmarc/hdfs/v2"
    "github.com/gorilla/mux"
    "github.com/samuel/go-zookeeper/zk"
)

var (
    hdfsClient *hdfs.Client
    zkConn     *zk.Conn
)

type HDFSResponse struct {
    FileContent string `json:"file_content"`
}

func main() {
    var err error

    // Connect to HDFS
    hdfsClient, err = hdfs.New("namenode:9000")
    if err != nil {
        log.Fatalf("Failed to connect to HDFS: %v", err)
    }
    defer hdfsClient.Close()

    // Connect to ZooKeeper
    zkServers := []string{"localhost:2181"}
    zkConn, _, err = zk.Connect(zkServers, 10*time.Second)
    if err != nil {
        log.Fatalf("Failed to connect to ZooKeeper: %v", err)
    }
    defer zkConn.Close()

    // Set up HTTP server
    r := mux.NewRouter()
    r.HandleFunc("/read_hdfs", readHDFSHandler).Methods("GET")
    http.Handle("/", r)

    fmt.Println("Server is running at http://localhost:8080")
    log.Fatal(http.ListenAndServe(":8080", nil))
}

func readHDFSHandler(w http.ResponseWriter, r *http.Request) {
    path := r.URL.Query().Get("path")
    if path == "" {
        http.Error(w, "Path is required", http.StatusBadRequest)
        return
    }

    file, err := hdfsClient.Open(path)
    if err != nil {
        http.Error(w, "Failed to open HDFS file", http.StatusInternalServerError)
        return
    }
    defer file.Close()

    content, err := ioutil.ReadAll(file)
    if err != nil {
        http.Error(w, "Failed to read HDFS file", http.StatusInternalServerError)
        return
    }

    response := HDFSResponse{FileContent: string(content)}
    w.Header().Set("Content-Type", "application/json")
    json.NewEncoder(w).Encode(response)
}
