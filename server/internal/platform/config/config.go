package config

import (
	"os"
)

type AppConfig struct {
	Env       string
	APIPort   string
	AllowCORS bool
	Auth      AuthConfig
}

type AuthConfig struct {
	AdminUsername string
	AdminPassword string
	SessionSecret string
	DisplayName   string
}

func Load() AppConfig {
	return AppConfig{
		Env:       getEnv("CACAO_ENV", "development"),
		APIPort:   getEnv("CACAO_API_PORT", "8080"),
		AllowCORS: getEnv("CACAO_ALLOW_CORS", "true") == "true",
		Auth: AuthConfig{
			AdminUsername: getEnv("CACAO_ADMIN_USERNAME", "amanda"),
			AdminPassword: getEnv("CACAO_ADMIN_PASSWORD", "1234"),
			SessionSecret: getEnv("CACAO_SESSION_SECRET", "cacao-dev-secret"),
			DisplayName:   getEnv("CACAO_ADMIN_DISPLAY_NAME", "Amanda"),
		},
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
