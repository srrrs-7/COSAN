package response

type Scope struct {
	Cid    int64  `json:"company_id"`
	Sid    int64  `json:"staff_id"`
	Scopes []int8 `json:"scopes"`
}

type Role struct {
	Cid  int64 `json:"company_id"`
	Sid  int64 `json:"staff_id"`
	Role int8  `json:"role"`
}
