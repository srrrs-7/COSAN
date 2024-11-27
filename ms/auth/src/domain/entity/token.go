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
	Cid     int64
	Sid     int64
	Expired time.Duration
	Issued  time.Time
	Scopes  []int8
	Role    int8
}

type RefreshToken struct {
	Cid     int64
	Sid     int64
	Expired time.Duration
	Issued  time.Time
	Scopes  []int8
	Role    int8
}

func NewToken(cid, sid int64, role int8, scope []int8, secretKey string) *Token {
	at := AccessToken{
		Cid:     cid,
		Sid:     sid,
		Expired: ACCESSS_TOKEN_EXPIRED_AT,
		Issued:  time.Now(),
		Role:    role,
		Scopes:  scope,
	}
	rt := RefreshToken{
		Cid:     cid,
		Sid:     sid,
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
		"Cid":     t.Cid,
		"Sid":     t.Sid,
		"Expired": t.Expired,
		"Issued":  t.Issued,
		"Scopes":  t.Scopes,
		"Role":    t.Role,
	})
	return generateJWT(secretKey, token)
}

func (t RefreshToken) JWT(secretKey string) string {
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, jwt.MapClaims{
		"Cid":     t.Cid,
		"Sid":     t.Sid,
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
