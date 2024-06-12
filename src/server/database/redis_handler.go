package main

import (
    "github.com/go-redis/redis/v8"
    "context"
    "net/http"
    "github.com/gorilla/mux"
    "log"
    "encoding/json"
)

var ctx = context.Background()

type AppState struct {
    client       *redis.Client
    allowedKeys map[string]bool
}

var appState AppState

func getValue(w http.ResponseWriter, r *http.Request) {
    key := mux.Vars(r)["key"]
    
    if !appState.allowedKeys[key] {
        http.Error(w, "Access denied", http.StatusForbidden)
        return
    }

    val, err := appState.client.Get(ctx, key).Result()
    if err != nil {
        http.Error(w, "Key not found", http.StatusNotFound)
        return
    }

    w.Write([]byte(val))
}

func setValue(w http.ResponseWriter, r *http.Request) {
    var data map[string]string
    json.NewDecoder(r.Body).Decode(&data)

    key := data["key"]
    value := data["value"]

    err := appState.client.Set(ctx, key, value, 0).Err()
    if err != nil {
        http.Error(w, "Error setting value", http.StatusInternalServerError)
        return
    }

    w.Write([]byte("Value set"))
}

func main() {
    client := redis.NewClient(&redis.Options{
        Addr: "localhost:6379",
    })

    appState = AppState{
        client:       client,
                   allowedKeys: map[string]bool{
                "allowed_key": true,
            },
        }

        r := mux.NewRouter()
        r.HandleFunc("/get/{key}", getValue).Methods("GET")
        r.HandleFunc("/set", setValue).Methods("POST")

        log.Fatal(http.ListenAndServe("127.0.0.1:5500", r))
    }
