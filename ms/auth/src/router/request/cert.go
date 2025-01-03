package request

import (
	"fmt"
	"utils/static"
)

type CreateScopeRequest struct {
	Uid  int64  `json:"user_id"`
	Cid  string `json:"certificate_domain_id"`
	Auth string `json:"authority"`
}

func (r CreateScopeRequest) Validate() error {
	if r.Uid < 0 {
		return fmt.Errorf("staff id is less than 0")
	}

	switch r.Auth {
	case static.READ, static.WRITE:
	default:
		return fmt.Errorf("authority %s is invalid", r.Auth)
	}

	switch r.Cid {
	case static.SUPPORT_DOMAIN:
		return nil
	default:
		return fmt.Errorf("scope %s is invalid ", r.Cid)
	}
}

type GetScopeRequest struct {
	Uid int64  `json:"user_id"`
	Cid string `json:"certificate_domain_id"`
}

func (r GetScopeRequest) Validate() error {
	if r.Uid < 0 {
		return fmt.Errorf("staff id is less than 0")
	}

	if r.Cid != "" {
		return fmt.Errorf("certificate domain id is less than 0")
	}

	return nil
}

type UpdateScopeRequest struct {
	Uid  int64  `json:"user_id"`
	Cid  string `json:"certificate_domain_id"`
	Auth string `json:"authority"`
}

func (r UpdateScopeRequest) Validate() error {
	if r.Uid < 0 {
		return fmt.Errorf("staff id is less than 0")
	}

	switch r.Auth {
	case static.READ, static.WRITE:
	default:
		return fmt.Errorf("authority %s is invalid", r.Auth)
	}

	switch r.Cid {
	case static.SUPPORT_DOMAIN:
		return nil
	default:
		return fmt.Errorf("scope %s is invalid ", r.Cid)
	}
}

type DeleteScopeRequest struct {
	Uid int64  `json:"user_id"`
	Cid string `json:"certificate_domain_id"`
}

func (r DeleteScopeRequest) Validate() error {
	if r.Uid < 0 {
		return fmt.Errorf("staff id is less than 0")
	}
	if r.Cid == "" {
		return fmt.Errorf("certificate domain id is less than 0")
	}

	return nil
}

type CreateRoleRequest struct {
	Uid  int64  `json:"user_id"`
	Role string `json:"user_role"`
}

func (r CreateRoleRequest) Validate() error {
	if r.Uid < 0 {
		return fmt.Errorf("staff id is less than 0")
	}

	if r.Role < "" {
		return fmt.Errorf("role is less than 0")
	}

	switch r.Role {
	case static.PROTAGONIST, static.SUPPORTER:
		return nil
	default:
		return fmt.Errorf("role %s is invalid", r.Role)
	}
}

type GetRoleRequest struct {
	Uid  int64  `json:"user_id"`
	Role string `json:"user_role"`
}

func (r GetRoleRequest) Validate() error {
	if r.Uid < 0 {
		return fmt.Errorf("staff id is less than 0")
	}

	switch r.Role {
	case static.PROTAGONIST, static.SUPPORTER:
	default:
		return fmt.Errorf("role %s is invalid", r.Role)
	}

	return nil
}

type UpdateRoleRequest struct {
	Uid  int64  `json:"user_id"`
	Role string `json:"user_role"`
}

func (r UpdateRoleRequest) Validate() error {
	if r.Uid < 0 {
		return fmt.Errorf("staff id is less than 0")
	}

	switch r.Role {
	case static.PROTAGONIST, static.SUPPORTER:
	default:
		return fmt.Errorf("role %s is invalid", r.Role)
	}

	return nil
}

type DeleteRoleRequest struct {
	Uid  int64  `json:"user_id"`
	Role string `json:"user_role"`
}

func (r DeleteRoleRequest) Validate() error {
	if r.Uid < 0 {
		return fmt.Errorf("staff id is less than 0")
	}

	switch r.Role {
	case static.PROTAGONIST, static.SUPPORTER:
	default:
		return fmt.Errorf("role %s is invalid", r.Role)
	}

	return nil
}
