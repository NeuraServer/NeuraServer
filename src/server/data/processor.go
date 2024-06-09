package main

import (
    "github.com/gin-gonic/gin"
    "gorm.io/driver/postgres"
    "gorm.io/gorm"
    "log"
    "net/http"
    "os"
)

type Data struct {
    gorm.Model
    Key   string
    Value string
}

var db *gorm.DB

func init() {
    dsn := "host=localhost user=postgres password=mysecretpassword dbname=mydb port=5432 sslmode=disable"
    var err error
    db, err = gorm.Open(postgres.Open(dsn), &gorm.Config{})
    if err != nil {
        log.Fatal("failed to connect database")
    }

    db.AutoMigrate(&Data{})
}

func createData(c *gin.Context) {
    var input Data
    if err := c.ShouldBindJSON(&input); err != nil {
        c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
        return
    }

    db.Create(&input)
    c.JSON(http.StatusOK, input)
}

func getData(c *gin.Context) {
    var data []Data
    db.Find(&data)
    c.JSON(http.StatusOK, data)
}

func main() {
    r := gin.Default()
    r.POST("/data", createData)
    r.GET("/data", getData)

    port := os.Getenv("PORT")
    if port == "" {
        port = "8080"
    }
    r.Run(":" + port)
}
