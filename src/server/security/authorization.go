package security

import (
	"errors"
	"sync"
	"time"
)

var (
	usersMap  map[string]User
	usersLock sync.RWMutex
)

func init() {
	// Initialize users map
	usersMap = make(map[string]User)
	// Populate users map with your data
	for _, user := range users {
		usersMap[user.APIKey] = user
	}
}

func authorize(apiKey, token string) (User, error) {
	usersLock.RLock()
	defer usersLock.RUnlock()

	user, ok := usersMap[apiKey]
	if !ok {
		return User{}, errors.New("invalid API key")
	}

	if user.Token != token || user.TokenExpiry.Before(time.Now()) {
		return User{}, errors.New("invalid or expired token")
	}

	return user, nil
}
