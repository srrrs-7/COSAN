package entity

import (
	"time"

	"github.com/golang-jwt/jwt"
)

const (
	ACCESSS_TOKEN_EXPIRED_AT = 15 * time.Minute
	REFRESH_TOKEN_EXPIRED_AT = 30 * 24 * time.Hour
)

type Token struct {
	AccessToken  string
	RefreshToken string
	IssuedAt     time.Time
	ExpiredAt    time.Time
}

type AccessToken struct {
	Uid     int64
	Expired time.Duration
	Issued  time.Time
	Scopes  []string
	Role    int8
}

type RefreshToken struct {
	Uid     int64
	Expired time.Duration
	Issued  time.Time
	Scopes  []string
	Role    int8
}

func NewToken(uid int64, role int8, scope []string, secretKey string) *Token {
	at := AccessToken{
		Uid:     uid,
		Expired: ACCESSS_TOKEN_EXPIRED_AT,
		Issued:  time.Now(),
		Role:    role,
		Scopes:  scope,
	}
	rt := RefreshToken{
		Uid:     uid,
		Expired: REFRESH_TOKEN_EXPIRED_AT,
		Issued:  time.Now(),
		Role:    role,
		Scopes:  scope,
	}

	return &Token{
		AccessToken:  at.JWT(secretKey),
		RefreshToken: rt.JWT(secretKey),
		IssuedAt:     time.Now(),
		ExpiredAt:    time.Now().Add(ACCESSS_TOKEN_EXPIRED_AT),
	}
}

func (t AccessToken) JWT(secretKey string) string {
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, jwt.MapClaims{
		"Uid":     t.Uid,
		"Expired": t.Expired,
		"Issued":  t.Issued,
		"Scopes":  t.Scopes,
		"Role":    t.Role,
	})

	return generateJWT(secretKey, token)
}

func (t RefreshToken) JWT(secretKey string) string {
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, jwt.MapClaims{
		"Uid":     t.Uid,
		"Expired": t.Expired,
		"Issued":  t.Issued,
		"Scopes":  t.Scopes,
		"Role":    t.Role,
	})

	return generateJWT(secretKey, token)
}

func generateJWT(secretKey string, token *jwt.Token) string {
	signedToken, err := token.SignedString([]byte(secretKey))
	if err != nil {
		panic(err)
	}

	return signedToken
}
