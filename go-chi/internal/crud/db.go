package crud

import (
	"github.com/bertonha/go-chi/internal/models"
	"github.com/bertonha/go-chi/internal/utils"
	"github.com/jmoiron/sqlx"
)

var db *sqlx.DB

func ConnectDB() error {
	var err error
	var databaseUrl = utils.GetEnv("DATABASE_URL")
	db, err = sqlx.Connect("postgres", databaseUrl)
	return err
}

func CreateForm(inputForm models.FormInput) (*models.Form, error) {
	rows, err := db.NamedQuery(`INSERT INTO forms (title) VALUES (:title) RETURNING id`, inputForm)
	if err != nil {
		return nil, err
	}
	var id int64
	if rows.Next() {
		_ = rows.Scan(&id)
	}

	form := models.Form{ID: id, Title: inputForm.Title}
	return &form, err
}

func ListForms() ([]models.Form, error) {
	var forms []models.Form

	err := db.Select(&forms, "SELECT * FROM forms")
	return forms, err
}

func GetForm(id int) (models.Form, error) {
	var form models.Form
	err := db.Get(&form, "SELECT * FROM forms WHERE id = $1", id)
	return form, err
}
