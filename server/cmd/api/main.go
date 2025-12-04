package main

import (
	"context"
	"log/slog"

	authapi "github.com/Cacao/Cacao/internal/api/auth"
	domainauth "github.com/Cacao/Cacao/internal/domain/auth"
	"github.com/Cacao/Cacao/internal/platform/config"
	"github.com/Cacao/Cacao/internal/platform/router"
	"github.com/Cacao/Cacao/internal/platform/server"
)

func main() {
	cfg := config.Load()
	authenticator := domainauth.NewAuthenticator(
		cfg.Auth.AdminUsername,
		cfg.Auth.AdminPassword,
		cfg.Auth.SessionSecret,
		cfg.Auth.DisplayName,
	)
	authHandler := authapi.NewHandler(authenticator)
	eng := router.New(cfg, authHandler)
	srv := server.New(cfg, eng)

	if err := srv.Run(context.Background()); err != nil {
		slog.Error("api server exited", "error", err)
	}
}
