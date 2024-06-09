package main

import (
    "encoding/json"
    "fmt"
    "log"
    "net/http"
    "time"

    "github.com/dgrijalva/jwt-go"
    "github.com/gorilla/mux"
    "github.com/joho/godotenv"
    "golang.org/x/oauth2"
)

var oauthConfig *oauth2.Config

func main() {
    err := godotenv.Load()
    if err != nil {
        log.Fatalf("Error loading .env file")
    }

    oauthConfig = &oauth2.Config{
        ClientID:     "your-client-id",
        ClientSecret: "your-client-secret",
        RedirectURL:  "http://localhost:8080/callback",
        Scopes:       []string{"read", "write"},
        Endpoint: oauth2.Endpoint{
            AuthURL:  "https://example.com/oauth/authorize",
            TokenURL: "https://example.com/oauth/token",
        },
    }

    r := mux.NewRouter()
    r.HandleFunc("/generate-api-key", generateAPIKeyHandler).Methods("POST")
    r.HandleFunc("/oauth2", startOAuth2).Methods("GET")
    r.HandleFunc("/callback", handleOAuth2Callback).Methods("GET")
    http.Handle("/", r)

    log.Println("Server starting on :8080")
    log.Fatal(http.ListenAndServe(":8080", r))
}

func generateAPIKeyHandler(w http.ResponseWriter, r *http.Request) {
    var ip struct {
        Address string `json:"address"`
    }
    if err := json.NewDecoder(r.Body).Decode(&ip); err != nil {
        http.Error(w, err.Error(), http.StatusBadRequest)
        return
    }

    apiKey := generateAPIKey(ip.Address)
    json.NewEncoder(w).Encode(map[string]string{"apiKey": apiKey})
}

func generateAPIKey(address string) string {
    token := jwt.NewWithClaims(jwt.SigningMethodHS256, jwt.MapClaims{
        "address": address,
        "exp":     time.Now().Add(time.Hour * 24).Unix(),
    })
    tokenString, _ := token.SignedString([]byte("your-secret-key"))
    return tokenString
}

func startOAuth2(w http.ResponseWriter, r *http.Request) {
    url := oauthConfig.AuthCodeURL("state", oauth2.AccessTypeOffline)
    http.Redirect(w, r, url, http.StatusTemporaryRedirect)
}

func handleOAuth2Callback(w http.ResponseWriter, r *http.Request) {
    code := r.URL.Query().Get("code")
    token, err := oauthConfig.Exchange(oauth2.NoContext, code)
    if err != nil {
        http.Error(w, err.Error(), http.StatusInternalServerError)
        return
    }

    json.NewEncoder(w).Encode(token)
}
