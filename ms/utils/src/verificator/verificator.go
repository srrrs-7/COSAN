package verificator

import (
	"auth/domain/entity"
	"errors"
	"fmt"
	"time"

	"github.com/golang-jwt/jwt"
)

// ValidateToken は、JWTを検証し、クレームを返します。
func ValidateToken(tokenString, secretKey string) (*entity.Claims, error) {
	token, err := jwt.ParseWithClaims(tokenString, &entity.Claims{}, func(token *jwt.Token) (interface{}, error) {
		if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, fmt.Errorf("unexpected signing method: %v", token.Header["alg"])
		}
		return secretKey, nil
	})
	if err != nil {
		return nil, err
	}

	if claims, ok := token.Claims.(*entity.Claims); ok && token.Valid {
		if claims.Expired < time.Now().Unix() {
			return nil, errors.New("token expired")
		}
		return claims, nil
	}

	return nil, errors.New("invalid token")
}
