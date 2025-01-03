package router

import (
	"auth/router/request"
	"auth/router/response"
	"net/http"
	"utils/utilhttp"
	"utils/utillog"

	"github.com/google/uuid"
)

func (rt Router) getScope(w http.ResponseWriter, req *http.Request) {
	traceId := uuid.New().String()
	req.Header.Set("x-trace-id", traceId)
	utillog.ApiAccessLog(req, traceId)

	uid, err := utilhttp.RequestUrlParam[int64](req, "user_id")
	if err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusBadRequest, err)
		utilhttp.ResponseBadRequest(w, response.Err{Error: err.Error()})
		return
	}

	cid, err := utilhttp.RequestUrlParam[int64](req, "certificate_id")
	if err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusBadRequest, err)
		utilhttp.ResponseBadRequest(w, response.Err{Error: err.Error()})
		return
	}

	r := request.GetScopeRequest{
		Uid: uid,
		Cid: cid,
	}
	if err := r.Validate(); err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusBadRequest, err)
		utilhttp.ResponseBadRequest(w, response.Err{Error: err.Error()})

	}

	scope, err := rt.certService.GetScope(req.Context(), r)
	if err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusInternalServerError, err)
		utilhttp.ResponseInternalServerError(w, response.Err{Error: err.Error()})
		return
	}

	utillog.ApiSuccessLog(req, traceId, http.StatusOK)
	utilhttp.ResponseOk(w, scope)
}

func (rt Router) createScope(w http.ResponseWriter, req *http.Request) {
	traceId := uuid.New().String()
	req.Header.Set("x-trace-id", traceId)
	utillog.ApiAccessLog(req, traceId)

	r, err := utilhttp.RequestBody[request.CreateScopeRequest](req)
	if err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusBadRequest, err)
		utilhttp.ResponseBadRequest(w, response.Err{Error: err.Error()})
		return
	}

	if err = r.Validate(); err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusBadRequest, err)
		utilhttp.ResponseBadRequest(w, response.Err{Error: err.Error()})
		return
	}

	if err = rt.certService.CreateScope(req.Context(), r); err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusInternalServerError, err)
		utilhttp.ResponseInternalServerError(w, response.Err{Error: err.Error()})
		return
	}

	utillog.ApiSuccessLog(req, traceId, http.StatusOK)
	utilhttp.ResponseCreated(w, "")
}

func (rt Router) updateScope(w http.ResponseWriter, req *http.Request) {
	traceId := uuid.New().String()
	req.Header.Set("x-trace-id", traceId)
	utillog.ApiAccessLog(req, traceId)

	r, err := utilhttp.RequestBody[request.UpdateScopeRequest](req)
	if err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusBadRequest, err)
		utilhttp.ResponseBadRequest(w, response.Err{Error: err.Error()})
		return
	}

	if err = r.Validate(); err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusBadRequest, err)
		utilhttp.ResponseBadRequest(w, response.Err{Error: err.Error()})
		return
	}

	if err = rt.certService.UpdateScope(req.Context(), r); err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusInternalServerError, err)
		utilhttp.ResponseInternalServerError(w, response.Err{Error: err.Error()})
		return
	}

	utillog.ApiSuccessLog(req, traceId, http.StatusOK)
	utilhttp.ResponseOk(w, "")
}

func (rt Router) deleteScope(w http.ResponseWriter, req *http.Request) {
	traceId := uuid.New().String()
	req.Header.Set("x-trace-id", traceId)
	utillog.ApiAccessLog(req, traceId)

	uid, err := utilhttp.RequestUrlParam[int64](req, "user_id")
	if err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusBadRequest, err)
		utilhttp.ResponseBadRequest(w, response.Err{Error: err.Error()})
		return
	}

	cid, err := utilhttp.RequestUrlParam[int64](req, "certificate_id")
	if err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusBadRequest, err)
		utilhttp.ResponseBadRequest(w, response.Err{Error: err.Error()})
		return
	}

	r := request.DeleteScopeRequest{
		Uid: uid,
		Cid: cid,
	}
	if err := r.Validate(); err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusBadRequest, err)
		utilhttp.ResponseBadRequest(w, response.Err{Error: err.Error()})
		return
	}

	if err = rt.certService.DeleteScope(req.Context(), r); err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusInternalServerError, err)
		utilhttp.ResponseInternalServerError(w, response.Err{Error: err.Error()})
		return
	}

	utillog.ApiSuccessLog(req, traceId, http.StatusOK)
	utilhttp.ResponseOk(w, "")
}

func (rt Router) getRole(w http.ResponseWriter, req *http.Request) {
	traceId := uuid.New().String()
	req.Header.Set("x-trace-id", traceId)
	utillog.ApiAccessLog(req, traceId)

	uid, err := utilhttp.RequestUrlParam[int64](req, "user_id")
	if err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusBadRequest, err)
		utilhttp.ResponseBadRequest(w, response.Err{Error: err.Error()})
		return
	}
	rl, err := utilhttp.RequestUrlParam[string](req, "user_role")
	if err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusBadRequest, err)
		utilhttp.ResponseBadRequest(w, response.Err{Error: err.Error()})
		return
	}

	r := request.GetRoleRequest{
		Uid:  uid,
		Role: rl,
	}
	if err := r.Validate(); err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusBadRequest, err)
		utilhttp.ResponseBadRequest(w, response.Err{Error: err.Error()})
		return
	}

	role, err := rt.certService.GetRole(req.Context(), r)
	if err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusInternalServerError, err)
		utilhttp.ResponseInternalServerError(w, response.Err{Error: err.Error()})
		return
	}

	utillog.ApiSuccessLog(req, traceId, http.StatusOK)
	utilhttp.ResponseOk(w, role)
}

func (rt Router) createRole(w http.ResponseWriter, req *http.Request) {
	traceId := uuid.New().String()
	req.Header.Set("x-trace-id", traceId)
	utillog.ApiAccessLog(req, traceId)

	r, err := utilhttp.RequestBody[request.CreateRoleRequest](req)
	if err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusBadRequest, err)
		utilhttp.ResponseBadRequest(w, response.Err{Error: err.Error()})
		return
	}
	if err = r.Validate(); err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusBadRequest, err)
		utilhttp.ResponseBadRequest(w, response.Err{Error: err.Error()})
		return
	}

	if err := rt.certService.CreateRole(req.Context(), r); err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusInternalServerError, err)
		utilhttp.ResponseInternalServerError(w, response.Err{Error: err.Error()})
		return
	}

	utillog.ApiSuccessLog(req, traceId, http.StatusOK)
	utilhttp.ResponseCreated(w, "")
}

func (rt Router) updateRole(w http.ResponseWriter, req *http.Request) {
	traceId := uuid.New().String()
	req.Header.Set("x-trace-id", traceId)
	utillog.ApiAccessLog(req, traceId)

	r, err := utilhttp.RequestBody[request.UpdateRoleRequest](req)
	if err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusBadRequest, err)
		utilhttp.ResponseBadRequest(w, response.Err{Error: err.Error()})
		return
	}
	if err = r.Validate(); err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusBadRequest, err)
		utilhttp.ResponseBadRequest(w, response.Err{Error: err.Error()})
		return
	}

	if err := rt.certService.UpdateRole(req.Context(), r); err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusInternalServerError, err)
		utilhttp.ResponseInternalServerError(w, response.Err{Error: err.Error()})
		return
	}

	utillog.ApiSuccessLog(req, traceId, http.StatusOK)
	utilhttp.ResponseOk(w, "")
}

func (rt Router) deleteRole(w http.ResponseWriter, req *http.Request) {
	traceId := uuid.New().String()
	req.Header.Set("x-trace-id", traceId)
	utillog.ApiAccessLog(req, traceId)

	uid, err := utilhttp.RequestUrlParam[int64](req, "user_id")
	if err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusBadRequest, err)
		utilhttp.ResponseBadRequest(w, response.Err{Error: err.Error()})
		return
	}
	rl, err := utilhttp.RequestUrlParam[string](req, "user_role")
	if err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusBadRequest, err)
		utilhttp.ResponseBadRequest(w, response.Err{Error: err.Error()})
		return
	}

	r := request.DeleteRoleRequest{
		Uid:  uid,
		Role: rl,
	}
	if err := r.Validate(); err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusBadRequest, err)
		utilhttp.ResponseBadRequest(w, response.Err{Error: err.Error()})
		return
	}

	if err := rt.certService.DeleteRole(req.Context(), r); err != nil {
		utillog.ApiErrorLog(req, traceId, http.StatusInternalServerError, err)
		utilhttp.ResponseInternalServerError(w, response.Err{Error: err.Error()})
		return
	}

	utillog.ApiSuccessLog(req, traceId, http.StatusOK)
	utilhttp.ResponseOk(w, "")
}
