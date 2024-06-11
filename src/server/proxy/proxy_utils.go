package main

import (
	"log"
	"net/http"
	"net/http/httputil"
	"net/url"

	"github.com/gorilla/mux"
)

func reverseProxy(target *url.URL) http.HandlerFunc {
	proxy := httputil.NewSingleHostReverseProxy(target)
	return func(w http.ResponseWriter, r *http.Request) {
		log.Printf("Reverse Proxy handling request for: %s\n", r.URL.Path)
		proxy.ServeHTTP(w, r)
	}
}

func main() {
	// Define the target URLs of the upstream servers
	upstreamURL1, _ := url.Parse("http://localhost:8080")
	upstreamURL2, _ := url.Parse("http://localhost:8081")

	r := mux.NewRouter()

	// Handler for /proxyutils path
	r.HandleFunc("/proxyutils", func(w http.ResponseWriter, r *http.Request) {
		w.Write([]byte("Proxy Utilities handling request"))
	})

	// Handler for /service1 path - Reverse proxy to upstreamURL1
	r.HandleFunc("/service1/{rest:.*}", reverseProxy(upstreamURL1))

	// Handler for /service2 path - Reverse proxy to upstreamURL2
	r.HandleFunc("/service2/{rest:.*}", reverseProxy(upstreamURL2))

	log.Fatal(http.ListenAndServe(":8082", r))
}
