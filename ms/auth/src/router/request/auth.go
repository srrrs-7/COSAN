package request

import (
	"fmt"
	"regexp"
)

type LoginRequest struct {
	LoginId  string `json:"login_id"`
	Password string `json:"password"`
}

func (r LoginRequest) Validate() error {
	if r.LoginId == "" || r.Password == "" {
		return fmt.Errorf("login_id or password is empty")
	}

	invalidChars := regexp.MustCompile(`[^a-zA-Z0-9]`)
	if invalidChars.MatchString(r.LoginId) {
		return fmt.Errorf("login_id contains invalid characters")
	}
	if invalidChars.MatchString(r.Password) {
		return fmt.Errorf("password contains invalid characters")
	}

	return nil
}
