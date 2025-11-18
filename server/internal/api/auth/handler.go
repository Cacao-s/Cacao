package auth

import (
	"errors"
	"net/http"
	"time"

	domain "github.com/Cacao/Cacao/internal/domain/auth"
	"github.com/gin-gonic/gin"
)

type Handler struct {
	authenticator *domain.Authenticator
}

func NewHandler(authenticator *domain.Authenticator) *Handler {
	return &Handler{authenticator: authenticator}
}

type loginRequest struct {
	Username string `json:"username" binding:"required"`
	Password string `json:"password" binding:"required"`
}

type loginResponse struct {
	Token     string       `json:"token"`
	ExpiresAt string       `json:"expiresAt"`
	User      responseUser `json:"user"`
}

type responseUser struct {
	Username    string   `json:"username"`
	DisplayName string   `json:"displayName"`
	Roles       []string `json:"roles"`
}

func (h *Handler) Login(c *gin.Context) {
	var req loginRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "invalid_request"})
		return
	}

	session, err := h.authenticator.Login(c.Request.Context(), req.Username, req.Password)
	if err != nil {
		if errors.Is(err, domain.ErrInvalidCredentials) {
			c.JSON(http.StatusUnauthorized, gin.H{"error": "invalid_credentials"})
			return
		}

		c.JSON(http.StatusInternalServerError, gin.H{"error": "login_failed"})
		return
	}

	res := loginResponse{
		Token:     session.Token,
		ExpiresAt: session.ExpiresAt.Format(time.RFC3339),
		User: responseUser{
			Username:    session.User.Username,
			DisplayName: session.User.DisplayName,
			Roles:       session.User.Roles,
		},
	}

	c.JSON(http.StatusOK, res)
}
