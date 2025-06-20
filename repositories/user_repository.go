package repositories

import (
	cacaomysql "cacaoapi/infra/db"
	"cacaoapi/models"
)

func GetAllUsers() ([]models.User, error) {
	var users []models.User
	err := cacaomysql.DB.Find(&users).Error
	return users, err
}

func CreateUser(user *models.User) error {
	return cacaomysql.DB.Create(user).Error
}
