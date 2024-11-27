package utillog

import (
	"log/slog"
	"net/http"
	"os"
	"time"
)

// NewLogger creates and sets a new slog.Logger as the default logger.
func NewLogger() {
	logger := slog.New(
		slog.NewJSONHandler(
			os.Stdout,
			&slog.HandlerOptions{Level: slog.LevelDebug},
		),
	)
	slog.SetDefault(logger)
}

func ApiAccessLog(req *http.Request, traceId string) {
	slog.Info(req.URL.Path,
		"timestamp", time.Now().Format(time.RFC3339),
		"id", traceId,
		"trace_id", req.Header.Get("x-trace-id"),
		"method", req.Method,
		"url", req.URL.String(),
		"header", req.Header,
		"body", req.Body,
		"remote", req.RemoteAddr,
		"host", req.Host,
		"content-length", req.ContentLength,
		"content-type", req.Header.Get("Content-Type"),
		"user-agent", req.UserAgent(),
		"referer", req.Referer(),
	)
}

func ApiErrorLog(req *http.Request, traceId string, status int, err error) {
	slog.Error(req.URL.Path,
		"timestamp", time.Now().Format(time.RFC3339),
		"id", traceId,
		"http_status", status,
		"error", err.Error(),
		"trace_id", req.Header.Get("x-trace-id"),
		"method", req.Method,
		"url", req.URL.String(),
		"header", req.Header,
		"body", req.Body,
	)
}

func ApiSuccessLog(req *http.Request, traceId string, status int) {
	slog.Info(req.URL.Path,
		"timestamp", time.Now().Format(time.RFC3339),
		"id", traceId,
		"http_status", status,
		"trace_id", req.Header.Get("x-trace-id"),
		"method", req.Method,
		"url", req.URL.String(),
		"header", req.Header,
		"body", req.Body,
	)
}
