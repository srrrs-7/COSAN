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
	CacheRepo Cacher
	BaseUrl   string
	SecretKey string
}

func NewAuth(a Autheticator, c Cacher, u string, sk string) AuthService {
	return AuthService{
		AuthRepo:  a,
		CacheRepo: c,
		BaseUrl:   u,
		SecretKey: sk,
	}
}

// login service
func (a AuthService) Login(ctx context.Context, lid string, psswd string) (*response.Login, error) {
	// get http params
	baseUrl, params, err := a.requestParams(a.BaseUrl, lid, psswd)
	if err != nil {
		return nil, err
	}
	// get master data
	master, err := utilhttp.HttpGetRequest[model.MasterModel](baseUrl, params, "")
	if err != nil {
		return nil, err
	}
	// login
	token, err := a.AuthRepo.Login(ctx, master.Cid, master.Sid, a.SecretKey)
	if err != nil {
		return nil, err
	}

	return response.NewLoginResponse(token.AccessToken, token.RefreshToken, token.ExpiredAt, token.IssuedAt), err
}

// logout services
func (a AuthService) Logout(ctx context.Context, aToken string) error {
	t, err := a.CacheRepo.Get(ctx, aToken)
	if err != nil {
		return err
	}
	err = a.CacheRepo.Delete(ctx, t.AccessToken)
	if err != nil {
		return err
	}
	err = a.CacheRepo.Delete(ctx, t.RefreshToken)
	if err != nil {
		return err
	}
	return nil
}

// refresh service
func (a AuthService) Refresh(ctx context.Context, aToken string) error {
	return nil
}

func (a AuthService) requestParams(baseUrl, sid, psswd string) (*url.URL, url.Values, error) {
	params := url.Values{}
	params.Add("login_id", sid)
	params.Add("password", psswd)

	u, err := url.Parse(fmt.Sprintf("%s/protagonist/login/%s/password/%s", baseUrl, sid, psswd))
	if err != nil {
		return nil, nil, err
	}
	u.RawQuery = params.Encode()

	return u, params, nil
}
