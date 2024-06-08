package server

import (
    "net/http"
    "net/http/httputil"
    "net/url"
)

type LoadBalancer struct {
    targets []*httputil.ReverseProxy
}

func NewLoadBalancer(targets []string) *LoadBalancer {
    proxies := make([]*httputil.ReverseProxy, len(targets))
    for i, target := range targets {
        url, _ := url.Parse(target)
        proxies[i] = httputil.NewSingleHostReverseProxy(url)
    }
    return &LoadBalancer{targets: proxies}
}

func (lb *LoadBalancer) ServeHTTP(w http.ResponseWriter, r *http.Request) {
    target := lb.targets[len(r.URL.Path)%len(lb.targets)]
    target.ServeHTTP(w, r)
}
