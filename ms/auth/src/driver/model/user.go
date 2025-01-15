package model

type User struct {
	Uid        int64  `json:"user_id"`
	ULastName  string `json:"user_last_name"`
	UFirstName string `json:"user_first_name"`
	UEmail     string `json:"user_email"`
	UCountry   string `json:"user_country"`
}
