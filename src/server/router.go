package main

import (
    "net/http"
    "github.com/gorilla/mux"
)

func routes() *mux.Router {
    r := mux.NewRouter()
    r.HandleFunc("/secure", secureHandler).Methods("GET")
    r.HandleFunc("/data", dataHandler).Methods("GET")
    return r
}

func secureHandler(w http.ResponseWriter, r *http.Request) {
    w.Write([]byte("Secure Route"))
}

func dataHandler(w http.ResponseWriter, r *http.Request) {
    w.Write([]byte("Data Route"))
}

func main() {
    r := routes()
    r.Use(loggingMiddleware)
    r.Use(authMiddleware)

    http.Handle("/", r)
    http.ListenAndServe(":8080", nil)
}
