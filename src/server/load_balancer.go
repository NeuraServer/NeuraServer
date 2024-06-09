package main
package server

import (
    "net/http"
    "net/http/httputil"
    "net/url"
    "sync/atomic"
    "github.com/gorilla/mux"
    "log"
)

type ServerPool struct {
    servers []*httputil.ReverseProxy
    current uint64
}

func (sp *ServerPool) getNextServer() *httputil.ReverseProxy {
    server := sp.servers[atomic.AddUint64(&sp.current, 1)%uint64(len(sp.servers))]
    return server
}

func (sp *ServerPool) loadBalance(w http.ResponseWriter, r *http.Request) {
    server := sp.getNextServer()
    server.ServeHTTP(w, r)
}

func newServerPool(urls []string) *ServerPool {
    servers := make([]*httputil.ReverseProxy, len(urls))
    for i, u := range urls {
        url, _ := url.Parse(u)
        servers[i] = httputil.NewSingleHostReverseProxy(url)
    }
    return &ServerPool{servers: servers}
}

func main() {
    serverURLs := []string{
        "http://localhost:8081",
        "http://localhost:8082",
    }

    serverPool := newServerPool(serverURLs)

    r := mux.NewRouter()
    r.HandleFunc("/", serverPool.loadBalance)

    log.Fatal(http.ListenAndServe(":8080", r))
}
