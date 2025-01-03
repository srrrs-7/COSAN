package router

import (
	"auth/router/response"
	"context"
	"net/http"
	"strings"
	"utils/verificator"

	"github.com/go-chi/chi"
)

type Autheticator interface {
	ProtagonistLogin(ctx context.Context, lid, psswd, secretKey string) (*response.Login, error)
	SupporterLogin(ctx context.Context, lid, psswd, secretKey string) (*response.Login, error)
	Logout(ctx context.Context, aToken string) error
	Refresh(ctx context.Context, aToken string) error
}

type Certificator interface{}

type Router struct {
	AuthService Autheticator
	CertService Certificator
	SecretKey   string
}

func NewRouter(a Autheticator, c Certificator, secretKey string) Router {
	return Router{
		AuthService: a,
		CertService: c,
		SecretKey:   secretKey,
	}
}

func (rt Router) Serve() *chi.Mux {
	router := chi.NewMux()

	router.Use(router.Middlewares()...)

	// health
	router.Get("/health", func(w http.ResponseWriter, r *http.Request) {})

	// auth domain path
	router.Route("/auth/v1", func(r chi.Router) {
		// auth
		r.Route("/auth", func(r chi.Router) {
			r.Post("/protagonist/login", rt.protagonistLogin)
			r.Post("/supporter/login", rt.supporterLogin)
			r.Group(func(r chi.Router) {
				r.Use(rt.middlewares)
				r.Get("/logout", rt.logout)
				r.Put("/refresh", rt.refresh)
			})
		})
		// cert
		r.Route("/cert", func(r chi.Router) {
			r.Post("/scope", rt.createScope)
			r.Post("/role", rt.createRole)
			r.Group(func(r chi.Router) {
				r.Use(rt.middlewares)
				// scope
				r.Get("/scope", rt.getScope)
				r.Put("/scope", rt.updateScope)
				r.Delete("/scope", rt.deleteScope)
				// role
				r.Get("/role", rt.getRole)
				r.Put("/role", rt.updateRole)
				r.Delete("/role", rt.deleteRole)
			})
		})
	})

	return router
}

func (rt Router) middlewares(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		authHeader := r.Header.Get("Authorization")
		if authHeader == "" || !strings.HasPrefix(authHeader, "Bearer ") {
			http.Error(w, "Authorization header required", http.StatusUnauthorized)
			return
		}

		claim, err := verificator.ValidateToken(strings.TrimPrefix(authHeader, "Bearer "), rt.SecretKey)
		if err != nil {
			http.Error(w, "Invalid token", http.StatusUnauthorized)
			return
		}

		next.ServeHTTP(w, r.WithContext(context.WithValue(r.Context(), "token", claim)))
	})
}
