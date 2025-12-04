package main

import (
	"context"
	"log/slog"
	"os/signal"
	"syscall"

	"github.com/Cacao/Cacao/internal/jobs"
	"github.com/Cacao/Cacao/internal/platform/config"
)

func main() {
	ctx, stop := signal.NotifyContext(context.Background(), syscall.SIGINT, syscall.SIGTERM)
	defer stop()

	cfg := config.Load()

	if err := jobs.Run(ctx, cfg); err != nil {
		slog.Error("jobs runner exited", "error", err)
	}
}
