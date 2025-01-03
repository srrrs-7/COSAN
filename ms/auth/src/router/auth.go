package router

import (
	"auth/router/request"
	"auth/router/response"
	"net/http"
	"utils/utilhttp"
	"utils/utillog"

	"github.com/google/uuid"
)

func (rt Router) protagonistLogin(w http.ResponseWriter, req *http.Request) {
	traceId := uuid.New().String()
	req.Header.Set("x-trace-id", traceId)
	utillog.ApiAccessLog(req, traceId)

	r, err := utilhttp.RequestBody[request.LoginRequest](req)
	if err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusBadRequest, err)
		utilhttp.ResponseBadRequest(w, response.Err{Error: err.Error()})
		return
	}
	if err := r.Validate(); err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusBadRequest, err)
		utilhttp.ResponseBadRequest(w, response.Err{Error: err.Error()})
		return
	}

	res, err := rt.AuthService.ProtagonistLogin(req.Context(), r.LoginId, r.Password, rt.SecretKey)
	if err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusInternalServerError, err)
		utilhttp.ResponseInternalServerError(w, response.Err{Error: err.Error()})
		return
	}

	utillog.ApiSuccessLog(req, traceId, http.StatusOK)
	utilhttp.ResponseOk(w, res)
}

func (rt Router) supporterLogin(w http.ResponseWriter, req *http.Request) {
	traceId := uuid.New().String()
	req.Header.Set("x-trace-id", traceId)
	utillog.ApiAccessLog(req, traceId)

	r, err := utilhttp.RequestBody[request.LoginRequest](req)
	if err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusBadRequest, err)
		utilhttp.ResponseBadRequest(w, response.Err{Error: err.Error()})
		return
	}
	if err := r.Validate(); err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusBadRequest, err)
		utilhttp.ResponseBadRequest(w, response.Err{Error: err.Error()})
		return
	}

	res, err := rt.AuthService.SupporterLogin(req.Context(), r.LoginId, r.Password, rt.SecretKey)
	if err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusInternalServerError, err)
		utilhttp.ResponseInternalServerError(w, response.Err{Error: err.Error()})
		return
	}

	utillog.ApiSuccessLog(req, traceId, http.StatusOK)
	utilhttp.ResponseOk(w, res)
}

func (rt Router) logout(w http.ResponseWriter, req *http.Request) {
	traceId := uuid.New().String()
	req.Header.Set("x-trace-id", traceId)
	utillog.ApiAccessLog(req, traceId)

	aToken, err := utilhttp.RequestBearer(req)
	if err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusBadRequest, err)
		utilhttp.ResponseBadRequest(w, response.Err{Error: err.Error()})
		return
	}

	err = rt.AuthService.Logout(req.Context(), aToken)
	if err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusInternalServerError, err)
		utilhttp.ResponseBadRequest(w, response.Err{Error: err.Error()})
		return
	}

	utillog.ApiSuccessLog(req, traceId, http.StatusOK)
	utilhttp.ResponseOk(w, "")
}

func (rt Router) refresh(w http.ResponseWriter, req *http.Request) {
	traceId := uuid.New().String()
	req.Header.Set("x-trace-id", traceId)
	utillog.ApiAccessLog(req, traceId)

	r, err := utilhttp.RequestBody[request.RefreshRequest](req)
	if err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusBadRequest, err)
		utilhttp.ResponseBadRequest(w, response.Err{Error: err.Error()})
		return
	}

	if err := rt.AuthService.Refresh(req.Context(), r.RefreshToken); err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusInternalServerError, err)
		utilhttp.ResponseInternalServerError(w, response.Err{Error: err.Error()})
		return
	}

	utillog.ApiSuccessLog(req, traceId, http.StatusOK)
	utilhttp.ResponseOk(w, "")
}
