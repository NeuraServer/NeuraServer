package main

import (
    "net/http"
    "github.com/gorilla/mux"
    "log"
    "time"
    "golang.org/x/time/rate"
    "sync"
)

type RateLimiter struct {
    visitors map[string]*Visitor
    mu       sync.Mutex
}

type Visitor struct {
    limiter  *rate.Limiter
    lastSeen time.Time
}

func NewRateLimiter() *RateLimiter {
    return &RateLimiter{
        visitors: make(map[string]*Visitor),
    }
}

func (rl *RateLimiter) getVisitor(ip string) *rate.Limiter {
    rl.mu.Lock()
    defer rl.mu.Unlock()

    v, exists := rl.visitors[ip]
    if !exists {
        limiter := rate.NewLimiter(1, 5)
        rl.visitors[ip] = &Visitor{limiter, time.Now()}
        return limiter
    }

    v.lastSeen = time.Now()
    return v.limiter
}

func (rl *RateLimiter) cleanupVisitors() {
    for {
        time.Sleep(time.Minute)
        rl.mu.Lock()
        for ip, v := range rl.visitors {
            if time.Since(v.lastSeen) > 3*time.Minute {
                delete(rl.visitors, ip)
            }
        }
        rl.mu.Unlock()
    }
}

var rl = NewRateLimiter()

func rateLimitMiddleware(next http.Handler) http.Handler {
    return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        ip := r.RemoteAddr
        limiter := rl.getVisitor(ip)
        if !limiter.Allow() {
            http.Error(w, http.StatusText(http.StatusTooManyRequests), http.StatusTooManyRequests)
            return
        }
        next.ServeHTTP(w, r)
    })
}

func main() {
    go rl.cleanupVisitors()

    r := mux.NewRouter()
    r.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
        w.Write([]byte("Welcome to NeuraServer with Advanced Rate Limiting!"))
    })
    r.Use(rateLimitMiddleware)

    log.Fatal(http.ListenAndServe(":8080", r))
}
