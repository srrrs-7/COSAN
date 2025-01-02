package model

import "auth/domain/entity"

type Protagonist struct {
	Pid        int64  `json:"protagonist_id"`
	PLastName  string `json:"protagonist_last_name"`
	PFirstName string `json:"protagonist_first_name"`
	PEmail     string `json:"protagonist_email"`
	PCountry   string `json:"protagonist_country"`
}

type Supporter struct {
	Sid        int64  `json:"supporter_id"`
	SLastName  string `json:"supporter_last_name"`
	SFirstName string `json:"supporter_first_name"`
	SEmail     string `json:"supporter_email"`
	SCountry   string `json:"supporter_country"`
}

type UserRole struct {
	UserRoleId int64  `gorm:"column:user_role_id"`
	UserId     int64  `gorm:"column:user_id"`
	UserRole   string `gorm:"column:user_role"`
}

type UserScope struct {
	DomainName string `gorm:"column:domain_name"`
}

func TokenEntity(uid int64, a UserRole, us []UserScope, secretKey string) *entity.Token {
	var scopes []string
	for _, s := range us {
		scopes = append(scopes, s.DomainName)
	}

	return entity.NewToken(uid, a.UserRole, scopes, secretKey)
}
