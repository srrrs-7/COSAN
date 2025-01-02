package model

import "auth/domain/entity"

type Protagonist struct {
	Pid int64 `json:"protagonist_id"`
}

type Supporter struct {
	Sid int64 `json:"supporter_id"`
}

type UserRole struct {
	UserRoleId int64 `gorm:"column:user_role_id"`
	UserId     int64 `gorm:"column:user_id"`
	UserRole   int8  `gorm:"column:user_role"`
}

type UserScope struct {
	DomainName string `gorm:"column:domain_name"`
}

func TokenEntity(a UserRole, us []UserScope, secretKey string) *entity.Token {
	var scopes []string
	for _, s := range us {
		scopes = append(scopes, s.DomainName)
	}

	return entity.NewToken(a.UserId, a.UserRole, scopes, secretKey)
}
