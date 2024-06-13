package main

import (
    "encoding/json"
    "log"
    "net/http"
)

type AppHandler func(http.ResponseWriter, *http.Request) error

type AppError struct {
    Code    int
    Message string
    Err     error
}

func (fn AppHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
    err := fn(w, r)
    if err != nil {
        log.Printf("Handling error: %v", err)
        http.Error(w, err.Error(), http.StatusInternalServerError)
    }
}

func CustomErrorMiddleware(next AppHandler) http.Handler {
    return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        err := next(w, r)
        if err != nil {
            if appErr, ok := err.(AppError); ok {
                response, _ := json.Marshal(map[string]string{
                    "error":   appErr.Message,
                    "details": appErr.Err.Error(),
                })
                http.Error(w, string(response), appErr.Code)
            } else {
                http.Error(w, err.Error(), http.StatusInternalServerError)
            }
        }
    })
}

func mainHandler(w http.ResponseWriter, r *http.Request) error {
    return AppError{
        Code:    http.StatusBadRequest,
        Message: "Invalid request",
        Err:     nil,
    }
}

func main() {
    http.Handle("/", CustomErrorMiddleware(mainHandler))
    http.ListenAndServe(":5500", nil)
}
