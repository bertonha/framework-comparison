package utils

import (
	"os"
	"sync"

	_ "github.com/joho/godotenv/autoload"
)

var environment = sync.Map{}

func GetEnv(key string) string {
	if val, ok := environment.Load(key); ok {
		return val.(string)
	}

	val := os.Getenv(key)
	if val == "" || len(val) == 0 {
		return ""
	}

	environment.Store(key, val)

	return val
}
