package request

import (
	"fmt"
	"utils/static"
)

type ScopeRequest struct {
	Cid    int64  `json:"company_id"`
	Sid    int64  `json:"staff_id"`
	Scopes []int8 `json:"scopes"`
}

func (s ScopeRequest) Validate() error {
	if s.Cid < 0 {
		return fmt.Errorf("company id is less than 0")
	}
	if s.Sid < 0 {
		return fmt.Errorf("staff id is less than 0")
	}
	for _, scope := range s.Scopes {
		switch scope {
		case static.AUDIT_DOMAIN, static.AUTH_DOMAIN, static.HOLIDAY_DOMAIN,
			static.MASTER_DOMAIN, static.SHIFT_DOMAIN, static.STAMP_DOMAIN,
			static.SUMMARY_DOMAIN:
			continue
		default:
			return fmt.Errorf("scope %d is invalid ", scope)
		}
	}
	return nil
}

type RoleRequest struct {
	Cid  int64 `json:"company_id"`
	Sid  int64 `json:"staff_id"`
	Role int8  `json:"role"`
}

func (r RoleRequest) Validate() error {
	if r.Cid < 0 {
		return fmt.Errorf("company id is less than 0")
	}
	if r.Sid < 0 {
		return fmt.Errorf("staff id is less than 0")
	}
	if r.Role < 0 {
		return fmt.Errorf("role is less than 0")
	}
	switch r.Role {
	case 1, 2, 4:
		return nil
	default:
		return fmt.Errorf("role %d is invalid", r.Role)
	}
}
