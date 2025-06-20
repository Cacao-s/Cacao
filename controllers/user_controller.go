package controllers

import (
    "net/http"

    "github.com/gin-gonic/gin"
    "cacaoapi/models"
    "cacaoapi/services"
)

func GetUsers(c *gin.Context) {
    users, err := services.ListUsers()
    if err != nil {
        c.JSON(http.StatusInternalServerError, gin.H{"error": "failed to fetch users"})
        return
    }
    c.JSON(http.StatusOK, users)
}

func PostUser(c *gin.Context) {
    var user models.User
    if err := c.ShouldBindJSON(&user); err != nil {
        c.JSON(http.StatusBadRequest, gin.H{"error": "invalid input"})
        return
    }
    if err := services.RegisterUser(&user); err != nil {
        c.JSON(http.StatusInternalServerError, gin.H{"error": "failed to create user"})
        return
    }
    c.JSON(http.StatusCreated, user)
}
