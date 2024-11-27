package response

import (
	"time"
)

type Login struct {
	AccessToken  string    `json:"access_token"`
	ExpiresAt    time.Time `json:"expires_at"`
	IssuedAt     time.Time `json:"issued_at"`
	RefreshToken string    `json:"refresh_token"`
}

func NewLoginResponse(ak, rt string, expire, issued time.Time) *Login {
	return &Login{
		AccessToken:  ak,
		ExpiresAt:    expire,
		IssuedAt:     issued,
		RefreshToken: rt,
	}
}

type Refresh struct {
	RefreshToken string `json:"refresh_token"`
	ExpiresAt    string `json:"expires_at"`
}
