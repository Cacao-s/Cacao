import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import AsyncStorage from '@react-native-async-storage/async-storage';
import { loginUser, getUserById } from '../services/authService';
import { User } from '../models/User';

interface AuthContextType {
  user: User | null;
  isLoading: boolean;
  isAuthenticated: boolean;
  login: (email: string, password: string) => Promise<{ success: boolean; message?: string }>;
  logout: () => Promise<void>;
  setUser: (user: User | null) => void;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

const AUTH_USER_ID_KEY = '@cacao_auth_user_id';

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  // Load saved user on mount
  useEffect(() => {
    loadSavedUser();
  }, []);

  const loadSavedUser = async () => {
    try {
      const savedUserId = await AsyncStorage.getItem(AUTH_USER_ID_KEY);
      if (savedUserId) {
        const savedUser = await getUserById(savedUserId);
        if (savedUser) {
          setUser(savedUser);
        } else {
          // User not found, clear saved ID
          await AsyncStorage.removeItem(AUTH_USER_ID_KEY);
        }
      }
    } catch (error) {
      console.error('Error loading saved user:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const login = async (email: string, password: string) => {
    try {
      const result = await loginUser(email, password);
      
      if (result.success && result.user) {
        setUser(result.user);
        // Save user ID for auto-login
        await AsyncStorage.setItem(AUTH_USER_ID_KEY, result.user.id);
        return { success: true };
      }
      
      return { success: false, message: result.message || '登入失敗' };
    } catch (error) {
      console.error('Login error:', error);
      return { success: false, message: '登入過程發生錯誤' };
    }
  };

  const logout = async () => {
    try {
      setUser(null);
      await AsyncStorage.removeItem(AUTH_USER_ID_KEY);
    } catch (error) {
      console.error('Logout error:', error);
    }
  };

  const updateUser = async (newUser: User | null) => {
    setUser(newUser);
    if (newUser) {
      await AsyncStorage.setItem(AUTH_USER_ID_KEY, newUser.id);
    } else {
      await AsyncStorage.removeItem(AUTH_USER_ID_KEY);
    }
  };

  return (
    <AuthContext.Provider
      value={{
        user,
        isLoading,
        isAuthenticated: !!user,
        login,
        logout,
        setUser: updateUser,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
}
