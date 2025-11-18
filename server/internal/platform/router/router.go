package router

import (
	"net/http"

	authhandler "github.com/Cacao/Cacao/internal/api/auth"
	"github.com/Cacao/Cacao/internal/platform/config"
	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
)

func New(cfg config.AppConfig, authHandler *authhandler.Handler) *gin.Engine {
	r := gin.New()
	r.Use(gin.Logger(), gin.Recovery())

	if cfg.AllowCORS {
		r.Use(cors.New(cors.Config{
			AllowOrigins: []string{"*"},
			AllowMethods: []string{"GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"},
			AllowHeaders: []string{"Origin", "Content-Type", "Authorization"},
		}))
	}

	r.GET("/health", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"status":  "ok",
			"service": "cacao-api",
		})
	})

	api := r.Group("/api/v1")
	if authHandler != nil {
		api.POST("/auth/login", authHandler.Login)
	}

	return r
}
