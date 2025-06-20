CREATE DATABASE Cacao
CHARACTER SET utf8mb4
COLLATE utf8mb4_0900_ai_ci;

USE Cacao;

-- Users
CREATE TABLE Users (
    UserId INT AUTO_INCREMENT PRIMARY KEY, 
    UserName VARCHAR(255),
    Email VARCHAR(255),
    IsActive BOOLEAN NOT NULL DEFAULT TRUE,
    Theme VARCHAR(50),
    LanguageSet VARCHAR(50),
    UserType INT, -- 0 baby, 1 Giver, 
    LoginPwd VARCHAR(255),
    AccessPwd VARCHAR(255),
    CreateDate DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UpdateDate DATETIME
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- RevenueTypes
CREATE TABLE RevenueTypes (
    RevenueTypeId INT AUTO_INCREMENT PRIMARY KEY,
    RevenueTypeName VARCHAR(255) NOT NULL UNIQUE,
    IsActive BOOLEAN NOT NULL DEFAULT TRUE,
    CreateDate DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UpdateDate DATETIME
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- Revenues
CREATE TABLE Revenues (
    RevenueId INT AUTO_INCREMENT PRIMARY KEY,
    RevenueName VARCHAR(255) NOT NULL,
    Amount DECIMAL(18,2) DEFAULT 0,
    RevenueTypeId INT,
    RevenueFrom INT,
    UserId INT,
    IsActive BOOLEAN NOT NULL DEFAULT TRUE,
    CreateDate DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UpdateDate DATETIME,
    FOREIGN KEY (RevenueTypeId) REFERENCES RevenueTypes(RevenueTypeId),
    FOREIGN KEY (RevenueFrom) REFERENCES Users(UserId),
    FOREIGN KEY (UserId) REFERENCES Users(UserId) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE INDEX idx_Revenues_UserId ON Revenues(UserId);

-- Accounts
CREATE TABLE Accounts (
    AccountId INT AUTO_INCREMENT PRIMARY KEY,
    Amount DECIMAL(18,2) DEFAULT 0,
    IsActive BOOLEAN NOT NULL DEFAULT TRUE,
    UserId INT,
    CreateDate DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UpdateDate DATETIME,
    FOREIGN KEY (UserId) REFERENCES Users(UserId) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- Goals
CREATE TABLE Goals (
    GoalId INT AUTO_INCREMENT PRIMARY KEY,
    IsActive BOOLEAN NOT NULL DEFAULT TRUE,
    GoalName VARCHAR(255) NOT NULL,
    TargetAmount DECIMAL(18,2) NOT NULL DEFAULT 0,
    CurrentAmount DECIMAL(18,2) NOT NULL DEFAULT 0,
    DueDate DATETIME,
    CreateDate DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UpdateDate DATETIME,
    IsSuccess BOOLEAN,
    UserId INT,
    RevenueTypeId INT,
    FOREIGN KEY (UserId) REFERENCES Users(UserId) ON DELETE SET NULL,
    FOREIGN KEY (RevenueTypeId) REFERENCES RevenueTypes(RevenueTypeId) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- Withdrawals
CREATE TABLE Withdrawals (
    WithdrawalId INT AUTO_INCREMENT PRIMARY KEY,
    Amount DECIMAL(18,2) DEFAULT 0,
    UserId INT,
    IsActive BOOLEAN NOT NULL DEFAULT TRUE,
    WithdrawalDate DATETIME NOT NULL,
    CreateDate DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UpdateDate DATETIME,
    FOREIGN KEY (UserId) REFERENCES Users(UserId) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE INDEX idx_Withdrawals_UserId ON Withdrawals(UserId);

-- Tasks
CREATE TABLE Tasks (
    TaskId INT AUTO_INCREMENT PRIMARY KEY,
    Revenue DECIMAL(18,2) DEFAULT 0,
    TaskFrom INT,
    UserId INT,
    IsActive BOOLEAN NOT NULL DEFAULT TRUE,
    IsSuccess BOOLEAN,
    CompleteDate DATETIME,
    CreateDate DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UpdateDate DATETIME,
    FOREIGN KEY (TaskFrom) REFERENCES Users(UserId) ON DELETE SET NULL,
    FOREIGN KEY (UserId) REFERENCES Users(UserId) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE INDEX idx_Tasks_UserId ON Tasks(UserId);
CREATE INDEX idx_Tasks_TaskFrom ON Tasks(TaskFrom);

-- Notifications
CREATE TABLE Notifications (
    NotificationId INT AUTO_INCREMENT PRIMARY KEY,
    UserId INT NOT NULL,
    Message TEXT NOT NULL,
    IsRead BOOLEAN NOT NULL DEFAULT FALSE,
    NotificationDate DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (UserId) REFERENCES Users(UserId) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- Sample insert
INSERT INTO Users (
    UserName, Email, IsActive, Theme, LanguageSet, UserType, LoginPwd, AccessPwd, UpdateDate
) VALUES (
    'cacao', 'dummy_user@example.com', TRUE, 'dark', 'en-US', 0, 'cacao', 'cacao', CURRENT_TIMESTAMP
);

