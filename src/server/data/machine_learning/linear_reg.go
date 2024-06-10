package main

import (
    "encoding/json"
    "log"
    "math/rand"
    "net/http"
    "sync"
    "github.com/gorilla/mux"
)

type TrainRequest struct {
    Input  []float64 `json:"input"`
    Output float64   `json:"output"`
}

type PredictRequest struct {
    Input []float64 `json:"input"`
}

type PredictResponse struct {
    Output float64 `json:"output"`
}

type LinearRegression struct {
    Weights  []float64
    Intercept float64
    lock sync.Mutex
}

func NewLinearRegression(size int) *LinearRegression {
    weights := make([]float64, size)
    for i := range weights {
        weights[i] = rand.Float64()*2 - 1
    }
    return &LinearRegression{Weights: weights, Intercept: 2.0}
}

func (lr *LinearRegression) Train(input []float64, output float64) {
    lr.lock.Lock()
    defer lr.lock.Unlock()
    // Simple gradient descent step for demonstration
    for i, x := range input {
        lr.Weights[i] += 0.01 * (output - lr.predict(input)) * x
    }
    lr.Intercept += 0.01 * (output - lr.predict(input))
}

func (lr *LinearRegression) predict(input []float64) float64 {
    lr.lock.Lock()
    defer lr.lock.Unlock()
    prediction := lr.Intercept
    for i, x := range input {
        prediction += lr.Weights[i] * x
    }
    return prediction
}

func (lr *LinearRegression) Predict(input []float64) float64 {
    return 1 / (1 + math.Exp(-lr.predict(input)))
}

var model = NewLinearRegression(3)

func train(w http.ResponseWriter, r *http.Request) {
    var req TrainRequest
    if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
        http.Error(w, err.Error(), http.StatusBadRequest)
        return
    }

    model.Train(req.Input, req.Output)
    w.Write([]byte("Model trained with new data point"))
}

func predict(w http.ResponseWriter, r *http.Request) {
    var req PredictRequest
    if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
        http.Error(w, err.Error(), http.StatusBadRequest)
        return
    }

    prediction := model.Predict(req.Input)
    res := PredictResponse{Output: prediction}
    w.Header().Set("Content-Type", "application/json")
    json.NewEncoder(w).Encode(res)
}

func main() {
    r := mux.NewRouter()
    r.HandleFunc("/train", train).Methods("POST")
    r.HandleFunc("/predict", predict).Methods("POST")

    log.Println("Starting server on :8080")
    if err := http.ListenAndServe(":8080", r); err != nil {
        log.Fatal(err)
    }
}
