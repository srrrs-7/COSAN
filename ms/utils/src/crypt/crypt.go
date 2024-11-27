package crypt

import (
	"golang.org/x/crypto/bcrypt"
)

func Hash(psswd []byte) ([]byte, error) {
	psswdHash, err := bcrypt.GenerateFromPassword(psswd, bcrypt.DefaultCost)
	if err != nil {
		return nil, err
	}
	return psswdHash, nil
}

func CompareHash(psswd, psswdHash []byte) error {
	if err := bcrypt.CompareHashAndPassword(psswdHash, psswd); err != nil {
		return err
	}
	return nil
}
