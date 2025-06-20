package routes

import (
    "github.com/gin-gonic/gin"
    "cacaoapi/controllers"
)

func SetupRouter() *gin.Engine {
    r := gin.Default()
    userGroup := r.Group("/api/users")
    {
        userGroup.GET("/", controllers.GetUsers)
        userGroup.POST("/", controllers.PostUser)
    }
    return r
}
