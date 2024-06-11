package security

import (
	"crypto/rand"
	"encoding/hex"
	"errors"
	"time"

	"golang.org/x/crypto/bcrypt"
)

type User struct {
	Username     string
	PasswordHash string
	APIKey       string
	Token        string
	TokenExpiry  time.Time
}

var users = make(map[string]User)

func createUser(username, password string) (User, error) {
	if _, exists := users[username]; exists {
		return User{}, errors.New("user already exists")
	}

	passwordHash, err := bcrypt.GenerateFromPassword([]byte(password), bcrypt.DefaultCost)
	if err != nil {
		return User{}, err
	}

	apiKey := generateAPIKey()
	token, tokenExpiry := generateToken()

	user := User{
		Username:     username,
		PasswordHash: string(passwordHash),
		APIKey:       apiKey,
		Token:        token,
		TokenExpiry:  tokenExpiry,
	}
	users[username] = user
	return user, nil
}

func authenticate(username, password string) (User, error) {
	user, exists := users[username]
	if !exists {
		return User{}, errors.New("user not found")
	}

	err := bcrypt.CompareHashAndPassword([]byte(user.PasswordHash), []byte(password))
	if err != nil {
		return User{}, errors.New("incorrect password")
	}

	return user, nil
}

func generateAPIKey() string {
	key := make([]byte, 16)
	rand.Read(key)
	return hex.EncodeToString(key)
}

func generateToken() (string, time.Time) {
	token := make([]byte, 16)
	rand.Read(token)
	return hex.EncodeToString(token), time.Now().Add(24 * time.Hour)
}
