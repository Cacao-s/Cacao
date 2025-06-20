package cacaomysql

import (
	"fmt"
	"log"

	"gorm.io/driver/mysql"
	"gorm.io/gorm"
)

var DB *gorm.DB

// InitDB reads connection parameters from environment variables:
//
//	DB_USER, DB_PASS, DB_HOST, DB_PORT, DB_NAME
//
// The function establishes a GORM connection and stores it in the package‑level DB variable.
func InitDB() {
	user := "root"
	pass := "mysettingpassword"
	host := "127.0.0.1"
	port := "3307"
	name := "Cacao"
	// user := os.Getenv("DB_USER")
	// pass := os.Getenv("DB_PASS")
	// host := os.Getenv("DB_HOST")
	// port := os.Getenv("DB_PORT")
	// name := os.Getenv("DB_NAME")

	if user == "" || pass == "" || host == "" || port == "" || name == "" {
		log.Fatalf("database environment variables are incomplete")
	}

	dsn := fmt.Sprintf("%s:%s@tcp(%s:%s)/%s?parseTime=true&charset=utf8mb4&loc=Local",
		user, pass, host, port, name)

	db, err := gorm.Open(mysql.Open(dsn), &gorm.Config{})
	if err != nil {
		log.Fatalf("database connection failed: %v", err)
	}

	DB = db
}
