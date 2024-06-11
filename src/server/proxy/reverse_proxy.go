package main

import (
    "log"
    "net/http"
    "net/http/httputil"
    "net/url"

    "github.com/gorilla/mux"
)

func reverseProxy(target string) http.HandlerFunc {
    targetURL, err := url.Parse(target)
    if err != nil {
        log.Fatalf("Error parsing target URL: %v", err)
    }

    proxy := httputil.NewSingleHostReverseProxy(targetURL)
    return func(w http.ResponseWriter, r *http.Request) {
        log.Printf("Reverse Proxy handling request for: %s\n", r.URL.Path)
        proxy.ServeHTTP(w, r)
    }
}

func main() {
    // Define the target URL of the upstream server
    upstreamURL := "http://localhost:8080"

    r := mux.NewRouter()
    r.HandleFunc("/reverseproxy", reverseProxy(upstreamURL))

    log.Fatal(http.ListenAndServe(":8081", r))
}
