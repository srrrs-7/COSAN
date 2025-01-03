package model

import "auth/domain/entity"

type GetScope struct {
	Uid  int64  `gorm:"column:user_id"`
	Cid  int64  `gorm:"column:certificate_domain_id"`
	Auth string `gorm:"column:authority"`
}

func (s GetScope) Entity() *entity.Scope {
	return &entity.Scope{
		Uid:  s.Uid,
		Cid:  s.Cid,
		Auth: s.Auth,
	}
}

type GetRole struct {
	Uid  int64  `gorm:"column:user_id"`
	Role string `gorm:"column:user_role"`
}

func (r GetRole) Entity() *entity.Role {
	return &entity.Role{
		Uid:  r.Uid,
		Role: r.Role,
	}
}
