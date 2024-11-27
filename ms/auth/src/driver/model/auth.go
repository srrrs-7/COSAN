package model

import "auth/domain/entity"

type MasterModel struct {
	Cid int64 `json:"company_id"`
	Sid int64 `json:"staff_id"`
}

type Auth struct {
	StaffRoleId int64 `gorm:"column:staff_role_id"`
	CompanyId   int64 `gorm:"column:company_id"`
	StaffId     int64 `gorm:"column:staff_id"`
	StaffRole   int8  `gorm:"column:staff_role"`
}

type Scope struct {
	CertificateDomainId int8 `gorm:"column:certificate_domain_id"`
}

func TokenEntity(a Auth, ss []Scope, secretKey string) *entity.Token {
	var scopes []int8
	for _, s := range ss {
		scopes = append(scopes, s.CertificateDomainId)
	}
	return entity.NewToken(a.CompanyId, a.StaffId, a.StaffRole, scopes, secretKey)
}
