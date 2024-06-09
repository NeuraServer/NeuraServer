package main

import (
    "log"
    "net/http"
    "time"

    "github.com/gorilla/mux"
    "github.com/rs/cors"
)

// LoggerMiddleware logs the details of each incoming HTTP request
func LoggerMiddleware(next http.Handler) http.Handler {
    return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        start := time.Now()
        next.ServeHTTP(w, r)
        log.Printf(
            "%s %s %s %s",
            r.Method,
            r.RequestURI,
            r.RemoteAddr,
            time.Since(start),
        )
    })
}

// RecoverMiddleware recovers from panics and logs the error
func RecoverMiddleware(next http.Handler) http.Handler {
    return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        defer func() {
            if err := recover(); err != nil {
                http.Error(w, "Internal Server Error", http.StatusInternalServerError)
                log.Printf("panic: %v", err)
            }
        }()
        next.ServeHTTP(w, r)
    })
}

// HomeHandler handles the root route
func HomeHandler(w http.ResponseWriter, r *http.Request) {
    w.Write([]byte("Welcome to the API Gateway"))
}

func main() {
    r := mux.NewRouter()

    // Set up routes
    r.HandleFunc("/", HomeHandler).Methods(http.MethodGet)

    // Apply middlewares
    r.Use(LoggerMiddleware)
    r.Use(RecoverMiddleware)

    // Set up CORS
    c := cors.New(cors.Options{
        AllowedOrigins:   []string{"*"},
        AllowedMethods:   []string{"GET", "POST", "PUT", "DELETE", "OPTIONS"},
        AllowedHeaders:   []string{"Content-Type", "Authorization"},
        AllowCredentials: true,
        MaxAge:           86400,
    })

    // Start the server
    srv := &http.Server{
        Handler:      c.Handler(r),
        Addr:         ":8080",
        WriteTimeout: 15 * time.Second,
        ReadTimeout:  15 * time.Second,
        IdleTimeout:  60 * time.Second,
    }

    log.Printf("Starting server on %s", srv.Addr)
    if err := srv.ListenAndServe(); err != nil {
        log.Fatalf("Server failed to start: %v", err)
    }
}
