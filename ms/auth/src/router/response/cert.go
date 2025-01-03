package response

type GetScopeResponse struct {
	Uid  int64  `json:"user_id"`
	Cid  int64  `json:"certificate_domain_id"`
	Auth string `json:"authority"`
}

type GetRoleResponse struct {
	Uid  int64  `json:"user_id"`
	Role string `json:"user_role"`
}
