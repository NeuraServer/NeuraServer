  package main

    import (
        "database/sql"
        "net/http"
        "github.com/gorilla/mux"
        _ "github.com/denisenkom/go-mssqldb"
        "log"
        "fmt"
        "encoding/json"
    )

    var db *sql.DB

    type AppState struct {
        allowedTables map[string]bool
    }

    var appState AppState

    func initDB() {
        var err error
        connString := "server=localhost;user id=SA;password=your_password;port=1433;database=your_database"
        db, err = sql.Open("sqlserver", connString)
        if err != nil {
            log.Fatal("Error creating connection pool: ", err)
        }
    }

    func getUser(w http.ResponseWriter, r *http.Request) {
        id := mux.Vars(r)["id"]

        if !appState.allowedTables["users"] {
            http.Error(w, "Access denied", http.StatusForbidden)
            return
        }

        var name string
        err := db.QueryRow("SELECT name FROM users WHERE id = @p1", id).Scan(&name)
        if err != nil {
            http.Error(w, "User not found", http.StatusNotFound)
            return
        }
        w.Write([]byte(fmt.Sprintf("User: %s", name)))
    }

    func setUser(w http.ResponseWriter, r *http.Request) {
        var data map[string]interface{}
        json.NewDecoder(r.Body).Decode(&data)

        id := int(data["id"].(float64))
        name := data["name"].(string)

        if !appState.allowedTables["users"] {
            http.Error(w, "Access denied", http.StatusForbidden)
            return
        }

        _, err := db.Exec("INSERT INTO users (id, name) VALUES (@p1, @p2)", id, name)
        if err != nil {
            http.Error(w, "Error inserting into the database", http.StatusInternalServerError)
            return
        }
        w.Write([]byte("User added"))
    }

    func main() {
        initDB()
        appState = AppState{
            allowedTables: map[string]bool{
                "users": true,
            },
        }

        r := mux.NewRouter()
        r.HandleFunc("/user/{id}", getUser).Methods("GET")
        r.HandleFunc("/user", setUser).Methods("POST")

        log.Fatal(http.ListenAndServe("127.0.0.1:5500", r))
    }
