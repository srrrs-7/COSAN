package service

import (
	"auth/driver/model"
	"auth/router/response"
	"context"
	"fmt"
	"net/url"
	"utils/utilhttp"
)

type AuthService struct {
	AuthRepo  Autheticator
	BaseUrl   string
	SecretKey string
}

func NewAuth(a Autheticator, u string, sk string) AuthService {
	return AuthService{
		AuthRepo:  a,
		BaseUrl:   u,
		SecretKey: sk,
	}
}

// protagonist login service
func (a AuthService) ProtagonistLogin(ctx context.Context, lid, psswd string) (*response.Login, error) {
	// get http params
	baseUrl, params, err := a.protagonistRequestParams(a.BaseUrl, lid, psswd)
	if err != nil {
		return nil, err
	}
	// get master data
	master, err := utilhttp.HttpGetRequest[model.Protagonist](baseUrl, params, "")
	if err != nil {
		return nil, err
	}
	// login
	token, err := a.AuthRepo.Login(ctx, master.Pid, a.SecretKey)
	if err != nil {
		return nil, err
	}

	return response.NewLoginResponse(token.AccessToken, token.RefreshToken, token.ExpiredAt, token.IssuedAt), err
}

// supporter logout services
func (a AuthService) SupporterLogin(ctx context.Context, lid, psswd string) (*response.Login, error) {
	// get http params
	baseUrl, params, err := a.supporterRequestParams(a.BaseUrl, lid, psswd)
	if err != nil {
		return nil, err
	}
	// get master data
	master, err := utilhttp.HttpGetRequest[model.Supporter](baseUrl, params, "")
	if err != nil {
		return nil, err
	}
	// login
	token, err := a.AuthRepo.Login(ctx, master.Sid, a.SecretKey)
	if err != nil {
		return nil, err
	}

	return response.NewLoginResponse(token.AccessToken, token.RefreshToken, token.ExpiredAt, token.IssuedAt), err
}

// logout services
func (a AuthService) Logout(ctx context.Context, aToken string) error {
	return nil
}

// refresh service
func (a AuthService) Refresh(ctx context.Context, aToken string) error {
	return nil
}

func (a AuthService) protagonistRequestParams(baseUrl, lid, psswd string) (*url.URL, url.Values, error) {
	params := url.Values{}
	params.Add("login_id", lid)
	params.Add("password", psswd)

	u, err := url.Parse(fmt.Sprintf("%s/protagonist/login/%s/password/%s", baseUrl, lid, psswd))
	if err != nil {
		return nil, nil, err
	}
	u.RawQuery = params.Encode()

	return u, params, nil
}

func (a AuthService) supporterRequestParams(baseUrl, lid, psswd string) (*url.URL, url.Values, error) {
	params := url.Values{}
	params.Add("login_id", lid)
	params.Add("password", psswd)

	u, err := url.Parse(fmt.Sprintf("%s/supporter/login/%s/password/%s", baseUrl, lid, psswd))
	if err != nil {
		return nil, nil, err
	}
	u.RawQuery = params.Encode()

	return u, params, nil
}
