package server

import (
    "database/sql"
    "log"
    "time"
    _ "github.com/lib/pq"
)

type ConnectionPool struct {
    db *sql.DB
}

func NewConnectionPool(dataSourceName string) *ConnectionPool {
    db, err := sql.Open("postgres", dataSourceName)
    if err != nil {
        log.Fatalf("Error opening database: %q", err)
    }

    db.SetMaxOpenConns(25)
    db.SetMaxIdleConns(25)
    db.SetConnMaxLifetime(5 * time.Minute)

    return &ConnectionPool{db: db}
}

func (p *ConnectionPool) GetDB() *sql.DB {
    return p.db
}
