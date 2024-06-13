package main

import (
    "encoding/json"
    "fmt"
    "log"
    "net/http"
    "sync"
    "time"
    "github.com/dgrijalva/jwt-go"
    "github.com/go-chi/chi"
    "github.com/go-chi/chi/middleware"
)

var mySigningKey = []byte("supersecretkey")

type User struct {
    ID    string `json:"id"`
    Name  string `json:"name"`
    Email string `json:"email"`
}

var (
    users   = make(map[string]User)
    usersMu sync.Mutex
)

func generateJWT() (string, error) {
    token := jwt.New(jwt.SigningMethodHS256)
    claims := token.Claims.(jwt.MapClaims)
    claims["authorized"] = true
    claims["user"] = "user123"
    claims["exp"] = time.Now().Add(time.Minute * 30).Unix()
    tokenString, err := token.SignedString(mySigningKey)
    if err != nil {
        return "", err
    }
    return tokenString, nil
}

func validateJWT(tokenString string) (*jwt.Token, error) {
    token, err := jwt.Parse(tokenString, func(token *jwt.Token) (interface{}, error) {
        return mySigningKey, nil
    })
    if err != nil {
        return nil, err
    }
    if !token.Valid {
        return nil, fmt.Errorf("invalid token")
    }
    return token, nil
}

func authMiddleware(next http.Handler) http.Handler {
    return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        tokenString := r.Header.Get("Authorization")
        if tokenString == "" {
            http.Error(w, "Authorization token required", http.StatusUnauthorized)
            return
        }
        _, err := validateJWT(tokenString)
        if err != nil {
            http.Error(w, err.Error(), http.StatusUnauthorized)
            return
        }
        next.ServeHTTP(w, r)
    })
}

func createUser(w http.ResponseWriter, r *http.Request) {
    usersMu.Lock()
    defer usersMu.Unlock()

    var user User
    if err := json.NewDecoder(r.Body).Decode(&user); err != nil {
        http.Error(w, err.Error(), http.StatusBadRequest)
        return
    }
    users[user.ID] = user
    w.WriteHeader(http.StatusCreated)
}

func getUser(w http.ResponseWriter, r *http.Request) {
    usersMu.Lock()
    defer usersMu.Unlock()

    id := chi.URLParam(r, "id")
    user, ok := users[id]
    if !ok {
        http.Error(w, "User not found", http.StatusNotFound)
        return
    }
    json.NewEncoder(w).Encode(user)
}

func updateUser(w http.ResponseWriter, r *http.Request) {
    usersMu.Lock()
    defer usersMu.Unlock()

    id := chi.URLParam(r, "id")
    var user User
    if err := json.NewDecoder(r.Body).Decode(&user); err != nil {
        http.Error(w, err.Error(), http.StatusBadRequest)
        return
    }
    users[id] = user
    w.WriteHeader(http.StatusOK)
}

func deleteUser(w http.ResponseWriter, r *http.Request) {
    usersMu.Lock()
    defer usersMu.Unlock()

    id := chi.URLParam(r, "id")
    delete(users, id)
    w.WriteHeader(http.StatusOK)
}

func main() {
    r := chi.NewRouter()
    r.Use(middleware.Logger)
    r.Use(middleware.Recoverer)
    r.Use(authMiddleware)

    r.Post("/users", createUser)
    r.Get("/users/{id}", getUser)
    r.Put("/users/{id}", updateUser)
    r.Delete("/users/{id}", deleteUser)

    token, err := generateJWT()
    if err != nil {
        log.Fatal(err)
    }
    fmt.Printf("Generated Token: %s\n", token)

    log.Println("API server is listening on port 8080")
    http.ListenAndServe(":8080", r)
}
