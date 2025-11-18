package auth

import (
	"context"
	"crypto/rand"
	"crypto/sha256"
	"encoding/hex"
	"errors"
	"fmt"
	"strings"
	"time"
)

var ErrInvalidCredentials = errors.New("invalid credentials")

type Authenticator struct {
	adminUsername string
	adminPassword string
	sessionSecret string
	displayName   string
}

type User struct {
	ID          string
	Username    string
	DisplayName string
	Roles       []string
}

type Session struct {
	Token     string
	ExpiresAt time.Time
	User      User
}

func NewAuthenticator(username, password, secret, displayName string) *Authenticator {
	username = strings.TrimSpace(username)
	password = strings.TrimSpace(password)
	displayName = strings.TrimSpace(displayName)

	if username == "" {
		username = "amanda"
	}
	if password == "" {
		password = "1234"
	}
	if secret == "" {
		secret = "cacao-dev-secret"
	}
	if displayName == "" {
		displayName = "Amanda"
	}

	return &Authenticator{
		adminUsername: username,
		adminPassword: password,
		sessionSecret: secret,
		displayName:   displayName,
	}
}

func (a *Authenticator) Login(ctx context.Context, username, password string) (Session, error) {
	if username != a.adminUsername || password != a.adminPassword {
		return Session{}, ErrInvalidCredentials
	}

	token := a.issueToken(username)
	user := User{
		ID:          "admin",
		Username:    a.adminUsername,
		DisplayName: a.displayName,
		Roles:       []string{"admin"},
	}

	return Session{
		Token:     token,
		ExpiresAt: time.Now().Add(24 * time.Hour),
		User:      user,
	}, nil
}

func (a *Authenticator) issueToken(username string) string {
	nonce := make([]byte, 32)
	if _, err := rand.Read(nonce); err != nil {
		return fmt.Sprintf("%s-%d", username, time.Now().UnixNano())
	}

	h := sha256.New()
	h.Write(nonce)
	h.Write([]byte(a.sessionSecret))
	h.Write([]byte(username))
	h.Write([]byte(time.Now().String()))

	return hex.EncodeToString(h.Sum(nil))
}
