package verificator

import (
	"auth/domain/entity"
	"errors"
	"time"

	"github.com/golang-jwt/jwt"
)

func ValidateAccessToken(tokenString string) (*entity.AccessToken, error) {
	claims, err := getClaimsFromToken(tokenString)
	if err != nil {
		return nil, err
	}

	at := &entity.AccessToken{
		Cid:     int64(claims["Cid"].(float64)),
		Sid:     int64(claims["Sid"].(float64)),
		Issued:  time.Unix(int64(claims["Issued"].(float64)), 0),
		Expired: time.Duration(int64(claims["Expired"].(float64))) * time.Second,
		Scopes:  convertScopes(claims["Scopes"].([]interface{})),
		Role:    int8(claims["Role"].(float64)),
	}

	if expiredToken(at.Expired) {
		return nil, errors.New("refresh token expired")
	}

	return at, nil
}

func ValidateRefreshToken(tokenString string) (*entity.RefreshToken, error) {
	claims, err := getClaimsFromToken(tokenString)
	if err != nil {
		return nil, err
	}

	rt := &entity.RefreshToken{
		Cid:     int64(claims["Cid"].(float64)),
		Sid:     int64(claims["Sid"].(float64)),
		Issued:  time.Unix(int64(claims["Issued"].(float64)), 0),
		Expired: time.Duration(int64(claims["Expired"].(float64))) * time.Second,
		Scopes:  convertScopes(claims["Scopes"].([]interface{})),
		Role:    int8(claims["Role"].(float64)),
	}

	if expiredToken(rt.Expired) {
		return nil, errors.New("refresh token expired")
	}

	return rt, nil
}

func getClaimsFromToken(tokenString string) (jwt.MapClaims, error) {
	token, err := jwt.Parse(tokenString, func(t *jwt.Token) (interface{}, error) {
		return []byte("secret"), nil
	})
	if err != nil || !token.Valid {
		return nil, err
	}

	claims, ok := token.Claims.(jwt.MapClaims)
	if !ok {
		return nil, errors.New("invalid token claims")
	}

	return claims, nil
}

func convertScopes(scopesInterface []interface{}) []int8 {
	scopes := make([]int8, len(scopesInterface))
	for i, v := range scopesInterface {
		scopes[i] = int8(v.(float64))
	}
	return scopes
}

func expiredToken(expired time.Duration) bool {
	return time.Now().After(time.Now().Add(expired))
}
