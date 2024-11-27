package utilhttp

import (
	"encoding/json"
	"fmt"
	"net/http"
	"strings"
	"time"
)

const (
	DATE_FORMAT  = "2006-01-02"
	TIME_FORMAT  = "15:04:05"
	SPECIAL_CHAR = `!@#$%^&*()_+\-=\[\]{};':\"\\|,.<>\/?`
)

func RequestUrlParam[T comparable](req *http.Request, key string) (T, error) {
	v, ok := req.Context().Value(key).(T)
	if !ok {
		return v, fmt.Errorf("invalid path parameter %s-%v", key, v)
	}
	return v, nil
}

func RequestUrlQuery(req *http.Request, query string) (string, error) {
	q := req.URL.Query().Get(query)
	if q == "" {
		return "", fmt.Errorf("query parameter '%s' not found", query)
	}
	return q, nil
}

func RequestBody[T any](req *http.Request) (T, error) {
	var body T
	if err := json.NewDecoder(req.Body).Decode(&body); err != nil {
		return body, fmt.Errorf("failed to decode request body: %w", err)
	}
	return body, nil
}

func RequestBearer(req *http.Request) (string, error) {
	beaarer := req.Header.Get("Authorization")
	if beaarer == "" {
		return "", fmt.Errorf("authorization header not found")
	}
	return beaarer, nil
}

func RequestParseDate(d string) (time.Time, error) {
	return time.Parse(DATE_FORMAT, d)
}

func RequestParseTime(t string) (time.Time, error) {
	return time.Parse(TIME_FORMAT, t)
}

func RequestValidateSpecialChar(s string) bool {
	return strings.ContainsAny(SPECIAL_CHAR, s)
}
