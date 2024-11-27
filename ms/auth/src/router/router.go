package router

import (
	"net/http"

	"github.com/go-chi/chi"
)

type Router struct {
	AuthService Autheticator
	CertService Certificator
}

func NewRouter(a Autheticator, c Certificator) Router {
	return Router{
		AuthService: a,
		CertService: c,
	}
}

func (rt Router) Serve() *chi.Mux {
	router := chi.NewMux()

	router.Use(router.Middlewares()...)

	// health
	router.Get("/health", func(w http.ResponseWriter, r *http.Request) {})

	// auth domain path
	router.Route("/auth/v1", func(r chi.Router) {
		rt.middlewares()
		// auth
		r.Route("/auth", func(r chi.Router) {
			r.Post("/login", rt.login)
			r.Get("/logout", rt.logout)
			r.Put("/refresh", rt.refresh)
		})
		// cert
		r.Route("/cert", func(r chi.Router) {
			r.Post("/scope", rt.updateScope)
			r.Get("/scope", rt.updateScope)
			r.Put("/scope", rt.updateScope)
			r.Delete("/scope", rt.updateScope)

			r.Post("/role", rt.updateRole)
			r.Get("/role", rt.updateRole)
			r.Put("/role", rt.updateRole)
			r.Delete("/role", rt.updateRole)
		})
	})
	return router
}

func (rt Router) middlewares() []func(http.Handler) http.Handler {
	return []func(http.Handler) http.Handler{}
}
