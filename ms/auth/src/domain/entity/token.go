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
type Claims struct {
	Uid     int64     `json:"uid"`
	Issued  time.Time `json:"issued"`
	Expired int64     `json:"expired"`
	Scopes  []string  `json:"scopes"`
	Role    int8      `json:"role"`
	jwt.StandardClaims
}

// NewToken は、新しいアクセストークンとリフレッシュトークンを生成します。
func NewToken(uid int64, role int8, scopes []string, secretKey string) *Token {
	now := time.Now()
	accessTokenClaims := Claims{
		Uid:     uid,
		Issued:  now,
		Expired: now.Add(accessTokenExpiration).Unix(),
		Scopes:  scopes,
		Role:    role,
	}
	refreshTokenClaims := Claims{
		Uid:     uid,
		Issued:  now,
		Expired: now.Add(refreshTokenExpiration).Unix(),
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
func generateJWT(claims Claims, secretKey string) string {
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	signedToken, err := token.SignedString([]byte(secretKey))
	if err != nil {
		panic(err)
	}
	return signedToken
}
