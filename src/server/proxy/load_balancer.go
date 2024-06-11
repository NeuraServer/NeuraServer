package main

import (
    "net/http"
    "github.com/gorilla/mux"
    "log"
)

func loadBalance(w http.ResponseWriter, r *http.Request) {
    w.Write([]byte("Load Balancer handling request"))
}

func main() {
    r := mux.NewRouter()
    r.HandleFunc("/loadbalance", loadBalance)

    log.Fatal(http.ListenAndServe(":8080", r))
}
