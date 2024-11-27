package service

type CertService struct {
	certRepo Certificater
}

func NewCert(c Certificater) CertService {
	return CertService{
		certRepo: c,
	}
}

func (c CertService) Certificate() {
	// check cache by token uuid
	// cache exist -> return true response and token

	// check refresh token
	// refresh token not exist -> return false response and redirect
	// refresh token exist -> reissue token and update refresh token -> return true and new token

	// return false response and redirect
}
