package response

type CreateScopeResponse struct {
	Uid  int64  `json:"user_id"`
	Cid  string `json:"certificate_domain_id"`
	Auth string `json:"authority"`
}

type GetScopeResponse struct {
	Uid  int64  `json:"user_id"`
	Cid  string `json:"certificate_domain_id"`
	Auth string `json:"authority"`
}

type UpdateScopeResponse struct {
	Uid  int64  `json:"user_id"`
	Cid  string `json:"certificate_domain_id"`
	Auth string `json:"authority"`
}

type DeleteScopeResponse struct {
	Uid int64 `json:"user_id"`
	Cid int64 `json:"certificate_domain_id"`
}

type CreateRoleResponse struct {
	Uid  int64  `json:"user_id"`
	Role string `json:"role"`
}

type GetRoleResponse struct {
	Uid  int64  `json:"user_id"`
	Role string `json:"role"`
}

type UpdateRoleResponse struct {
	Uid  int64  `json:"user_id"`
	Role string `json:"role"`
}

type DeleteRoleResponse struct {
	Uid int64 `json:"user_id"`
}
