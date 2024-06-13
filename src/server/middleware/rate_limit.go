package main

import (
    "net/http"
    "sync"
    "time"
)

type RateLimiter struct {
    mu        sync.Mutex
    requests  map[string]int
    limit     int
    interval  time.Duration
    lastReset time.Time
}

func NewRateLimiter(limit int, interval time.Duration) *RateLimiter {
    return &RateLimiter{
        requests:  make(map[string]int),
        limit:     limit,
        interval:  interval,
        lastReset: time.Now(),
    }
}

func (rl *RateLimiter) Reset() {
    rl.mu.Lock()
    defer rl.mu.Unlock()
    rl.requests = make(map[string]int)
    rl.lastReset = time.Now()
}

func (rl *RateLimiter) Allow(ip string) bool {
    rl.mu.Lock()
    defer rl.mu.Unlock()

    if time.Since(rl.lastReset) > rl.interval {
        rl.Reset()
    }

    rl.requests[ip]++
    return rl.requests[ip] <= rl.limit
}

func RateLimitMiddleware(rl *RateLimiter) func(http.Handler) http.Handler {
    return func(next http.Handler) http.Handler {
        return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
            ip := r.RemoteAddr
            if !rl.Allow(ip) {
                http.Error(w, "Too many requests", http.StatusTooManyRequests)
                return
            }
            next.ServeHTTP(w, r)
        })
    }
}

func mainHandler(w http.ResponseWriter, r *http.Request) {
    w.Write([]byte("Request successful"))
}

func main() {
    rateLimiter := NewRateLimiter(5, time.Minute)
    http.Handle("/", RateLimitMiddleware(rateLimiter)(http.HandlerFunc(mainHandler)))
    http.ListenAndServe(":5500", nil)
}
