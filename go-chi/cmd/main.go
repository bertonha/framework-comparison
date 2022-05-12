package main

import (
	"log"
	"net/http"

	"github.com/bertonha/go-chi/internal/crud"
	"github.com/bertonha/go-chi/internal/handlers"
	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	_ "github.com/lib/pq"
)

func main() {
	if err := crud.ConnectDB(); err != nil {
		log.Fatal(err)
	}

	r := chi.NewRouter()
	r.Use(middleware.Logger)
	r.Post("/forms", handlers.CreateForms)
	r.Get("/forms", handlers.ListForms)
	r.Get("/forms/{formId}", handlers.GetForm)
	http.ListenAndServe(":8000", r)
}
