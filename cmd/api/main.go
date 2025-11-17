package main

import (
	"context"
	"log/slog"

	"github.com/Cacao/Cacao/internal/platform/config"
	"github.com/Cacao/Cacao/internal/platform/router"
	"github.com/Cacao/Cacao/internal/platform/server"
)

func main() {
	cfg := config.Load()
	eng := router.New()
	srv := server.New(cfg, eng)

	if err := srv.Run(context.Background()); err != nil {
		slog.Error("api server exited", "error", err)
	}
}
