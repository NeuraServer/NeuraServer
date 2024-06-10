package main

import (
    "encoding/json"
    "net/http"
    "github.com/gorilla/mux"
    "sync"
)

type AppState struct {
    sync.Mutex
    ProcessedData []int
}

func processData(w http.ResponseWriter, r *http.Request) {
    rawData := []int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}
    var processedData []int

    for _, value := range rawData {
        processedData = append(processedData, value*value)
    }

    state.Lock()
    state.ProcessedData = processedData
    state.Unlock()

    json.NewEncoder(w).Encode(processedData)
}

var state = &AppState{}

func main() {
    r := mux.NewRouter()
    r.HandleFunc("/process_data", processData).Methods("GET")

    http.ListenAndServe(":8080", r)
}
