package config

import (
	"os"
)

type AppConfig struct {
	Env       string
	APIPort   string
	AllowCORS bool
}

func Load() AppConfig {
	return AppConfig{
		Env:       getEnv("CACAO_ENV", "development"),
		APIPort:   getEnv("CACAO_API_PORT", "8080"),
		AllowCORS: getEnv("CACAO_ALLOW_CORS", "true") == "true",
	}
}

func (c AppConfig) Addr() string {
	return ":" + c.APIPort
}

func getEnv(key, fallback string) string {
	if val, ok := os.LookupEnv(key); ok && val != "" {
		return val
	}
	return fallback
}
