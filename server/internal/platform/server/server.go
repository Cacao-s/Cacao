package server

import (
	"context"
	"log/slog"

	"github.com/Cacao/Cacao/internal/platform/config"
	"github.com/gin-gonic/gin"
)

type Server struct {
	cfg    config.AppConfig
	engine *gin.Engine
}

func New(cfg config.AppConfig, engine *gin.Engine) *Server {
	return &Server{cfg: cfg, engine: engine}
}

func (s *Server) Run(ctx context.Context) error {
	slog.Info("starting cacao api server", "env", s.cfg.Env, "addr", s.cfg.Addr())

	// gin.Run is blocking; we ignore ctx for now but will wire graceful shutdown later
	return s.engine.Run(s.cfg.Addr())
}
