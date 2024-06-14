package main

import (
    "encoding/json"
    "log"
    "net/http"
    "strings"
    "time"
)

type Middleware func(http.HandlerFunc) http.HandlerFunc

// LoggingMiddleware logs the details of each request
func LoggingMiddleware(next http.HandlerFunc) http.HandlerFunc {
    return func(w http.ResponseWriter, r *http.Request) {
        start := time.Now()
        next(w, r)
        log.Printf("%s %s %s %s", r.RemoteAddr, r.Method, r.URL.Path, time.Since(start))
    }
}

// AuthMiddleware checks for a valid token in the request header
func AuthMiddleware(token string) Middleware {
    return func(next http.HandlerFunc) http.HandlerFunc {
        return func(w http.ResponseWriter, r *http.Request) {
            authHeader := r.Header.Get("Authorization")
            if authHeader == "" || !strings.HasPrefix(authHeader, "Bearer ") {
                http.Error(w, "Forbidden", http.StatusForbidden)
                return
            }
            if authHeader[7:] != token {
                http.Error(w, "Invalid token", http.StatusUnauthorized)
                return
            }
            next(w, r)
        }
    }
}

// ErrorHandlingMiddleware handles errors and returns a JSON response
func ErrorHandlingMiddleware(next http.HandlerFunc) http.HandlerFunc {
    return func(w http.ResponseWriter, r *http.Request) {
        defer func() {
            if err := recover(); err != nil {
                w.WriteHeader(http.StatusInternalServerError)
                json.NewEncoder(w).Encode(map[string]string{
                    "error": "Internal Server Error",
                })
            }
        }()
        next(w, r)
    }
}

func mainHandler(w http.ResponseWriter, r *http.Request) {
    w.Write([]byte("Request successful"))
}

func main() {
    token := "your-secure-token"
    
    http.HandleFunc("/", LoggingMiddleware(
        AuthMiddleware(token)(
            ErrorHandlingMiddleware(mainHandler),
        ),
    ))
    log.Fatal(http.ListenAndServe(":5500", nil))
}
