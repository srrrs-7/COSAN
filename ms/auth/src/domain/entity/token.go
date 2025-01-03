package entity

import (
	"time"

	"github.com/golang-jwt/jwt"
)

const (
	accessTokenExpiration  = 15 * time.Minute
	refreshTokenExpiration = 30 * 24 * time.Hour
)

// Token は、アクセストークンとリフレッシュトークンを保持する構造体です。
type Token struct {
	AccessToken  string    `json:"access_token"`
	RefreshToken string    `json:"refresh_token"`
	IssuedAt     time.Time `json:"issued_at"`
	ExpiredAt    time.Time `json:"expired_at"`
}

// Claims は、JWTのペイロードに含めるクレームです。
type Claim struct {
	Uid     int64     `json:"uid"`
	Expired int64     `json:"exp"`
	Issued  time.Time `json:"iat"`
	Scopes  []string  `json:"scopes"`
	Role    string    `json:"role"`
	jwt.StandardClaims
}

// NewToken は、新しいアクセストークンとリフレッシュトークンを生成します。
func NewToken(uid int64, role string, scopes []string, secretKey string) *Token {
	now := time.Now()
	accessTokenClaims := Claim{
		Uid:     uid,
		Expired: now.Add(accessTokenExpiration).Unix(),
		Issued:  now,
		Scopes:  scopes,
		Role:    role,
	}
	refreshTokenClaims := Claim{
		Uid:     uid,
		Expired: now.Add(refreshTokenExpiration).Unix(),
		Issued:  now,
		Scopes:  scopes,
		Role:    role,
	}

	accessToken := generateJWT(accessTokenClaims, secretKey)
	refreshToken := generateJWT(refreshTokenClaims, secretKey)

	return &Token{
		AccessToken:  accessToken,
		RefreshToken: refreshToken,
		IssuedAt:     now,
		ExpiredAt:    now.Add(accessTokenExpiration),
	}
}

// generateJWT は、JWTを生成します。
func generateJWT(claims Claim, secretKey string) string {
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	signedToken, err := token.SignedString([]byte(secretKey))
	if err != nil {
		panic(err)
	}
	return signedToken
}
