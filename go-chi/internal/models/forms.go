package models

import "net/http"

type Form struct {
	ID    int64  `db:"id" json:"id"`
	Title string `db:"title" json:"title"`
}

type FormInput struct {
	Title string `json:"title"`
}

func (f *FormInput) Bind(r *http.Request) error {
	return nil
}
