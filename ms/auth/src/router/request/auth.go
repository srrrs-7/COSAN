package request

import (
	"fmt"
	"strings"
)

type LoginRequest struct {
	LoginId  string `json:"login_id"`
	Password string `json:"password"`
}

func (r LoginRequest) Validate() error {
	if r.LoginId == "" || r.Password == "" {
		return fmt.Errorf("staff_id or password is empty")
	}
	if strings.ContainsAny(r.LoginId, "!@#$%^&*()") {
		return fmt.Errorf("staff_id contains invalid characters")
	}
	if strings.ContainsAny(r.Password, "!@#$%^&*()") {
		return fmt.Errorf("password contains invalid characters")
	}
	return nil
}

type RefreshRequest struct {
	RefreshToken string `json:"refresh_token"`
}

func (r RefreshRequest) Validate() error {
	if r.RefreshToken == "" {
		return fmt.Errorf("refresh token is empty")
	}
	return nil
}
