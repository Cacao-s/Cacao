package main

import (
	cacaomysql "cacaoapi/infra/db"
	"cacaoapi/routes"
	"log"
	"os"

	"github.com/joho/godotenv"
)

func main() {
	// Load .env file if present (ignores error if missing)
	_ = godotenv.Load("../.env")

	cacaomysql.InitDB()

	r := routes.SetupRouter()

	port := os.Getenv("PORT")
	if port == "" {
		port = "8080"
	}

	if err := r.Run(":" + port); err != nil {
		log.Fatalf("server failed: %v", err)
	}
}
