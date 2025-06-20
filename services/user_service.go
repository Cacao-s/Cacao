package services

import (
    "cacaoapi/models"
    "cacaoapi/repositories"
)

func ListUsers() ([]models.User, error) {
    return repositories.GetAllUsers()
}

func RegisterUser(u *models.User) error {
    return repositories.CreateUser(u)
}
