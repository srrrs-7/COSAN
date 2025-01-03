package model

import "auth/domain/entity"

type InsertScope struct {
	Uid  int64  `gorm:"column:user_id"`
	Cid  string `gorm:"column:certificate_domain_id"`
	Auth string `gorm:"column:authority"`
}

type SelectScope struct {
	Uid  int64  `gorm:"column:user_id"`
	Cid  string `gorm:"column:certificate_domain_id"`
	Auth string `gorm:"column:authority"`
}

func (s SelectScope) Entity() *entity.Scope {
	return &entity.Scope{
		Uid:  s.Uid,
		Cid:  s.Cid,
		Auth: s.Auth,
	}
}

type UpdateScope struct {
	Uid  int64  `gorm:"column:user_id"`
	Cid  string `gorm:"column:certificate_domain_id"`
	Auth string `gorm:"column:authority"`
}

type DeleteScope struct {
	Uid int64  `gorm:"column:user_id"`
	Cid string `gorm:"column:certificate_domain_id"`
}

type InsertRole struct {
	Uid  int64  `gorm:"column:user_id"`
	Role string `gorm:"column:user_role"`
}

type SelectRole struct {
	Uid  int64  `gorm:"column:user_id"`
	Role string `gorm:"column:user_role"`
}

func (r SelectRole) Entity() *entity.Role {
	return &entity.Role{
		Uid:  r.Uid,
		Role: r.Role,
	}
}

type UpdateRole struct {
	Uid  int64  `gorm:"column:user_id"`
	Role string `gorm:"column:user_role"`
}

type DeleteRole struct {
	Uid  int64  `gorm:"column:user_id"`
	Role string `gorm:"column:user_role"`
}
