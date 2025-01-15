package router

import (
	"auth/router/request"
	"auth/router/response"
	"net/http"
	"utils/utilhttp"
	"utils/utillog"

	"github.com/google/uuid"
)

func (rt Router) userLogin(w http.ResponseWriter, req *http.Request) {
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

	res, err := rt.userService.UserLogin(req.Context(), r.LoginId, r.Password, rt.secretKey)
	if err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusInternalServerError, err)
		utilhttp.ResponseInternalServerError(w, response.Err{Error: err.Error()})
		return
	}

	utillog.ApiSuccessLog(req, traceId, http.StatusOK)
	utilhttp.ResponseOk(w, res)
}
