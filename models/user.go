package models

import "time"

type User struct {
	UserId      uint       `gorm:"primaryKey" json:"user_id"`
	UserName    string     `json:"user_name"`
	Email       string     `json:"email"`
	IsActive    bool       `gorm:"default:true" json:"is_active"`
	Theme       string     `json:"theme"`
	LanguageSet string     `json:"language_set"`
	UserType    int        `json:"user_type"` // 0 baby, 1 Giver
	LoginPwd    string     `json:"-"`
	AccessPwd   string     `json:"-"`
	CreateDate  time.Time  `gorm:"autoCreateTime" json:"create_date"`
	UpdateDate  *time.Time `gorm:"autoUpdateTime" json:"update_date"`
}

func (User) TableName() string {
	return "Users"
}
