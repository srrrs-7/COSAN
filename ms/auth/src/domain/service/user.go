package service

import (
	"auth/driver/model"
	"auth/driver/repository"
	"auth/router/response"
	"context"
	"fmt"
	"net/url"
	"utils/utilhttp"
)

type UserService struct {
	userRepo repository.User
	baseUrl  string
}

func NewUser(ur repository.User, u string) UserService {
	return UserService{
		userRepo: ur,
		baseUrl:  u,
	}
}

// user login service
func (a UserService) UserLogin(ctx context.Context, lid, psswd, secretKey string) (*response.Login, error) {
	// get http params
	baseUrl, params, err := a.requestParams(a.baseUrl, "user", lid, psswd)
	if err != nil {
		return nil, err
	}
	// get master data
	user, err := utilhttp.HttpGetRequest[model.User](baseUrl, params, "")
	if err != nil {
		return nil, err
	}
	// login
	token, err := a.userRepo.Login(ctx, user.Uid, secretKey)
	if err != nil {
		return nil, err
	}

	return response.NewLoginResponse(token.AccessToken, token.RefreshToken, token.ExpiredAt, token.IssuedAt), err
}

func (a UserService) requestParams(baseUrl, userType, lid, psswd string) (*url.URL, url.Values, error) {
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
