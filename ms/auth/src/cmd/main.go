package main

import (
	"auth/domain/service"
	"auth/driver"
	"auth/driver/query"
	"auth/driver/repository"
	"auth/router"
	"context"
	"fmt"
	"log/slog"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"
	"utils/utillog"
)

const (
	MODE        = "MODE"
	AUTH_PG_URL = "AUTH_PG_URL"
	REDIS_URL   = "REDIS_URL"
	SECRET_KEY  = "SECRET_KEY"
)

const (
	DEBUG   = "debug"
	RELEASE = "release"
)

type Env struct {
	Mode      string
	AuthPgUrl string
	RedisUrl  string
	SecretKey string
}

func newEnv() Env {
	e := Env{
		Mode:      os.Getenv(MODE),
		AuthPgUrl: os.Getenv(AUTH_PG_URL),
		RedisUrl:  os.Getenv(REDIS_URL),
		SecretKey: os.Getenv(SECRET_KEY),
	}

	if e.Mode == "" || e.AuthPgUrl == "" || e.RedisUrl == "" || e.SecretKey == "" {
		panic(fmt.Sprintf("either of env is an empty string.: %v", e))
	} else if e.Mode != DEBUG && e.Mode != RELEASE {
		panic(fmt.Sprintf("invalid mode: debug or release only but got %v", e.Mode))
	}

	return e
}

func main() {
	utillog.NewLogger()
	env := newEnv()
	slog.Info("env loaded", "env", env)

	gormDb, db := driver.NewDb(env.AuthPgUrl)
	defer db.Close()

	r := router.NewRouter(
		service.NewAuth(
			repository.NewAuthRepo(gormDb, query.NewAuthQuery()),
			repository.NewCacheRepo(env.RedisUrl),
			env.SecretKey,
		),
		service.NewCert(
			repository.NewCertRepo(gormDb, query.NewCertQuery()),
		),
	)

	slog.Info("starting server 8080", "mode", env.Mode)
	gracefulShutdown(&http.Server{
		Addr:    ":8080",
		Handler: r.Serve(),
	})
}

func gracefulShutdown(server *http.Server) {
	go func() {
		if err := server.ListenAndServe(); err != http.ErrServerClosed {
			panic(err.Error())
		}
	}()
	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, syscall.SIGINT, syscall.SIGTERM)
	<-sigChan
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	if err := server.Shutdown(ctx); err != nil {
		panic(err.Error())
	}
	slog.Info("server graceful shutdown")
}
