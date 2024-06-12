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

    func readData(w http.ResponseWriter, r *http.Request) {
        id := mux.Vars(r)["id"]

        if !appState.allowedTables["data"] {
            http.Error(w, "Access denied", http.StatusForbidden)
            return
        }

        var value string
        err := db.QueryRow("SELECT value FROM data WHERE id = @p1", id).Scan(&value)
        if err != nil {
            http.Error(w, "Data not found", http.StatusNotFound)
            return
        }
        w.Write([]byte(fmt.Sprintf("Data: %s", value)))
    }

    func writeData(w http.ResponseWriter, r *http.Request) {
        var data map[string]interface{}
        json.NewDecoder(r.Body).Decode(&data)

        id := int(data["id"].(float64))
        value := data["value"].(string)

        if !appState.allowedTables["data"] {
            http.Error(w, "Access denied", http.StatusForbidden)
            return
        }

        _, err := db.Exec("INSERT INTO data (id, value) VALUES (@p1, @p2)", id, value)
        if err != nil {
            http.Error(w, "Error inserting into the database", http.StatusInternalServerError)
            return
        }
        w.Write([]byte("Data added"))
    }

    func main() {
        initDB()
        appState = AppState{
            allowedTables: map[string]bool{
                "data": true,
            },
        }

        r := mux.NewRouter()
        r.HandleFunc("/data/{id}", readData).Methods("GET")
        r.HandleFunc("/data", writeData).Methods("POST")

        log.Fatal(http.ListenAndServe("127.0.0.1:5500", r))
    }
