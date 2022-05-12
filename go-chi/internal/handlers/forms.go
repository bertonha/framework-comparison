package handlers

import (
	"net/http"
	"strconv"

	"github.com/bertonha/go-chi/internal/crud"
	"github.com/bertonha/go-chi/internal/models"
	"github.com/go-chi/chi/v5"
	"github.com/go-chi/render"
)

func CreateForms(w http.ResponseWriter, r *http.Request) {
	formInput := &models.FormInput{}
	if err := render.Bind(r, formInput); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	form, err := crud.CreateForm(*formInput)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	render.JSON(w, r, form)
}

func ListForms(w http.ResponseWriter, r *http.Request) {
	forms, err := crud.ListForms()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	render.JSON(w, r, forms)
}

func GetForm(w http.ResponseWriter, r *http.Request) {
	formId, _ := strconv.Atoi(chi.URLParam(r, "formId"))
	form, err := crud.GetForm(formId)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	render.JSON(w, r, form)
}
