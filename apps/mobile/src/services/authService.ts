import * as Crypto from 'expo-crypto';
import { database } from '../database';
import User from '../models/User';
import { Q } from '@nozbe/watermelondb';

/**
 * Local authentication service (no API integration)
 * Handles user registration and login using local SQLite database
 * Uses expo-crypto for password hashing (SHA256)
 */

export interface RegisterInput {
  email: string;
  password: string;
  displayName: string;
  locale?: string;
  theme?: string;
}

export interface LoginInput {
  email: string;
  password: string;
}

export interface AuthResult {
  success: boolean;
  user?: User;
  error?: string;
}

/**
 * Hash password using SHA256
 */
async function hashPassword(password: string): Promise<string> {
  const hash = await Crypto.digestStringAsync(
    Crypto.CryptoDigestAlgorithm.SHA256,
    password
  );
  return hash;
}

/**
 * Register a new user with email and password
 */
export async function registerUser(input: RegisterInput): Promise<AuthResult> {
  try {
    const { email, password, displayName, locale = 'zh-TW', theme = 'default' } = input;

    // Validate input
    if (!email || !password || !displayName) {
      return { success: false, error: '請填寫所有必填欄位' };
    }

    if (password.length < 6) {
      return { success: false, error: '密碼至少需要 6 個字元' };
    }

    // Check if email already exists
    const usersCollection = database.get<User>('users');
    const existingUsers = await usersCollection
      .query(Q.where('email', email))
      .fetch();

    if (existingUsers.length > 0) {
      return { success: false, error: '此 Email 已被註冊' };
    }

    // Hash password
    const passwordHash = await hashPassword(password);

    // Create user
    const user = await database.write(async () => {
      return await usersCollection.create((newUser) => {
        newUser.email = email;
        newUser.passwordHash = passwordHash;
        newUser.displayName = displayName;
        newUser.locale = locale;
        newUser.theme = theme;
        newUser.role = 'baby'; // Default role
        newUser.status = 'active';
      });
    });

    console.log('✅ User registered successfully:', user.id);
    return { success: true, user };
  } catch (error) {
    console.error('❌ Registration failed:', error);
    return { success: false, error: '註冊失敗，請稍後再試' };
  }
}

/**
 * Login with email and password
 */
export async function loginUser(input: LoginInput): Promise<AuthResult> {
  try {
    const { email, password } = input;

    // Validate input
    if (!email || !password) {
      return { success: false, error: '請輸入 Email 和密碼' };
    }

    // Find user by email
    const usersCollection = database.get<User>('users');
    const users = await usersCollection
      .query(Q.where('email', email))
      .fetch();

    if (users.length === 0) {
      return { success: false, error: 'Email 或密碼錯誤' };
    }

    const user = users[0];

    // Check if user has password (not OAuth user)
    if (!user.passwordHash) {
      return { success: false, error: '此帳號使用 Google 登入，請使用 Google 登入' };
    }

    // Verify password
    const passwordHash = await hashPassword(password);
    if (passwordHash !== user.passwordHash) {
      return { success: false, error: 'Email 或密碼錯誤' };
    }

    // Check user status
    if (user.status !== 'active') {
      return { success: false, error: '此帳號已被停用' };
    }

    console.log('✅ User logged in successfully:', user.id);
    return { success: true, user };
  } catch (error) {
    console.error('❌ Login failed:', error);
    return { success: false, error: '登入失敗，請稍後再試' };
  }
}

/**
 * Get user by ID
 */
export async function getUserById(userId: string): Promise<User | null> {
  try {
    const usersCollection = database.get<User>('users');
    const user = await usersCollection.find(userId);
    return user;
  } catch (error) {
    console.error('❌ Get user failed:', error);
    return null;
  }
}

/**
 * Update user profile
 */
export async function updateUserProfile(
  userId: string,
  updates: Partial<{ displayName: string; locale: string; theme: string }>
): Promise<AuthResult> {
  try {
    const user = await getUserById(userId);
    if (!user) {
      return { success: false, error: '找不到使用者' };
    }

    await database.write(async () => {
      await user.update((u) => {
        if (updates.displayName) u.displayName = updates.displayName;
        if (updates.locale) u.locale = updates.locale;
        if (updates.theme) u.theme = updates.theme;
      });
    });

    console.log('✅ User profile updated:', userId);
    return { success: true, user };
  } catch (error) {
    console.error('❌ Update profile failed:', error);
    return { success: false, error: '更新失敗，請稍後再試' };
  }
}

/**
 * Change password
 */
export async function changePassword(
  userId: string,
  oldPassword: string,
  newPassword: string
): Promise<AuthResult> {
  try {
    const user = await getUserById(userId);
    if (!user) {
      return { success: false, error: '找不到使用者' };
    }

    if (!user.passwordHash) {
      return { success: false, error: '此帳號使用 Google 登入，無法變更密碼' };
    }

    // Verify old password
    const oldPasswordHash = await hashPassword(oldPassword);
    if (oldPasswordHash !== user.passwordHash) {
      return { success: false, error: '舊密碼錯誤' };
    }

    // Validate new password
    if (newPassword.length < 6) {
      return { success: false, error: '新密碼至少需要 6 個字元' };
    }

    // Hash new password
    const newPasswordHash = await hashPassword(newPassword);

    await database.write(async () => {
      await user.update((u) => {
        u.passwordHash = newPasswordHash;
      });
    });

    console.log('✅ Password changed:', userId);
    return { success: true, user };
  } catch (error) {
    console.error('❌ Change password failed:', error);
    return { success: false, error: '變更密碼失敗，請稍後再試' };
  }
}
