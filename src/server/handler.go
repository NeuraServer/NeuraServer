// ----------------------------------
// --- NeuraServer Copyright 2024 ---
// ----------------------------------

package main

import (
    "net/http"
    "github.com/gorilla/mux"
    "log"
    "encoding/json"
)

type Message struct {
    Content string `json:"content"`
}

func main() {
    r := mux.NewRouter()
    r.HandleFunc("/", homeHandler)
    r.HandleFunc("/message", messageHandler).Methods("POST")
    log.Fatal(http.ListenAndServe(":8080", r))
}

func homeHandler(w http.ResponseWriter, r *http.Request) {
    w.Write([]byte("Welcome to the API Gateway"))
}

func messageHandler(w http.ResponseWriter, r *http.Request) {
    var msg Message
    err := json.NewDecoder(r.Body).Decode(&msg)
    if err != nil {
        http.Error(w, err.Error(), http.StatusBadRequest)
        return
    }
    w.Write([]byte("Message received: " + msg.Content))
}


