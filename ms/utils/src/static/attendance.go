package static

// reference all attendace services

type CompanyIdKey string
type StaffIdKey string
type GroupIdKey string
type WorkTypeIdKey string
type WageIdKey string

const (
	// domain id
	AUTH_DOMAIN = iota
	AUDIT_DOMAIN
	HOLIDAY_DOMAIN
	MASTER_DOMAIN
	SHIFT_DOMAIN
	STAMP_DOMAIN
	SUMMARY_DOMAIN
	// role
	STAFF_ROLE   = 4
	MANAGER_ROLE = 2
	ADMIN_ROLE   = 1
	// header
	APPLICATION_JSON = "application/json"
	CONTENT_TYPE     = "Content-Type"
	AUTHORIZATION    = "Authorization"
	BEARER           = "Bearer"
)
