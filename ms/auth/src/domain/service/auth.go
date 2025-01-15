package service

import (
	"auth/domain/entity"
	"auth/driver/model"
	"auth/driver/repository"
	"auth/router/response"
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"net/url"
	"time"
	"utils/utilhttp"
)

type AuthService struct {
	authRepo repository.Autheticator
	baseUrl  string
}

func NewAuth(a repository.Autheticator, u string) AuthService {
	return AuthService{
		authRepo: a,
		baseUrl:  u,
	}
}

// protagonist login service
func (a AuthService) ProtagonistLogin(ctx context.Context, lid, psswd, secretKey string) (*response.Login, error) {
	// get http params
	baseUrl, params, err := a.requestParams(a.baseUrl, "protagonist", lid, psswd)
	if err != nil {
		return nil, err
	}
	// get master data
	protagonist, err := utilhttp.HttpGetRequest[model.Protagonist](baseUrl, params, "")
	if err != nil {
		return nil, err
	}
	// login
	token, err := a.authRepo.Login(ctx, protagonist.Pid, secretKey)
	if err != nil {
		return nil, err
	}

	return response.NewLoginResponse(token.AccessToken, token.RefreshToken, token.ExpiredAt, token.IssuedAt), err
}

// supporter logout services
func (a AuthService) SupporterLogin(ctx context.Context, lid, psswd, secretKey string) (*response.Login, error) {
	// get http params
	baseUrl, params, err := a.requestParams(a.baseUrl, "supporter", lid, psswd)
	if err != nil {
		return nil, err
	}
	// get master data
	supporter, err := utilhttp.HttpGetRequest[model.Supporter](baseUrl, params, "")
	if err != nil {
		return nil, err
	}
	// login
	token, err := a.authRepo.Login(ctx, supporter.Sid, secretKey)
	if err != nil {
		return nil, err
	}

	return response.NewLoginResponse(token.AccessToken, token.RefreshToken, token.ExpiredAt, token.IssuedAt), err
}

// logout services
func (a AuthService) Logout(ctx context.Context, w http.ResponseWriter) error {
	http.SetCookie(w, &http.Cookie{
		Name:     "access_token",
		Value:    "",
		Path:     "/",
		Expires:  time.Unix(0, 0),
		HttpOnly: true,
	})

	http.SetCookie(w, &http.Cookie{
		Name:     "refresh_token",
		Value:    "",
		Path:     "/",
		Expires:  time.Unix(0, 0),
		HttpOnly: true,
	})

	return nil
}

func (a AuthService) Refresh(ctx context.Context, rToken, secretKey string) (*response.Login, error) {
	var claim entity.Claim
	if err := json.Unmarshal([]byte(rToken), &claim); err != nil {
		return nil, err
	}

	token := entity.NewToken(claim.Uid, claim.Role, claim.Scopes, secretKey)

	return response.NewLoginResponse(token.AccessToken, token.RefreshToken, token.ExpiredAt, token.IssuedAt), nil
}

func (a AuthService) requestParams(baseUrl, userType, lid, psswd string) (*url.URL, url.Values, error) {
	params := url.Values{}
	params.Add("login_id", lid)
	params.Add("password", psswd)

	u, err := url.Parse(fmt.Sprintf("%s/%s/login/%s/password/%s", baseUrl, userType, lid, psswd))
	if err != nil {
		return nil, nil, err
	}
	u.RawQuery = params.Encode()

	return u, params, nil
}
