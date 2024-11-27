package router

import (
	"net/http"
	"utils/utilhttp"
	"utils/utillog"

	"github.com/google/uuid"
)

func (rt Router) updateScope(w http.ResponseWriter, req *http.Request) {
	traceId := uuid.New().String()
	req.Header.Set("x-trace-id", traceId)
	utillog.ApiAccessLog(req, traceId)
	// request validate

	utillog.ApiSuccessLog(req, traceId, http.StatusOK)
	utilhttp.ResponseOk(w, "")
}

func (rt Router) updateRole(w http.ResponseWriter, req *http.Request) {
	traceId := uuid.New().String()
	req.Header.Set("x-trace-id", traceId)
	utillog.ApiAccessLog(req, traceId)
	// request validate

	utillog.ApiSuccessLog(req, traceId, http.StatusOK)
	utilhttp.ResponseOk(w, "")
}
