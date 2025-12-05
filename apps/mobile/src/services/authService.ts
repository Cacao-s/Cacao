import * as Crypto from 'expo-crypto';
import { GoogleSignin } from '@react-native-google-signin/google-signin';
import { database } from '../database';
import User from '../models/User';
import { Q } from '@nozbe/watermelondb';

/**
 * Local authentication service (no API integration)
 * Handles user registration and login using local SQLite database
 * Uses expo-crypto for password hashing (SHA256)
 * Supports Google OAuth login
 */

/**
 * Configure Google Sign-In
 * Call this once when app starts
 */
export function configureGoogleSignIn() {
  GoogleSignin.configure({
    // Web Client ID from Google Cloud Console
    // You need to create OAuth 2.0 credentials in Google Cloud Console
    // and add both Android and iOS apps with their respective SHA-1 fingerprints
    webClientId: process.env.EXPO_PUBLIC_GOOGLE_WEB_CLIENT_ID || '',
    offlineAccess: false,
  });
}

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

/**
 * Login with Google OAuth
 */
export async function loginWithGoogle(): Promise<AuthResult> {
  try {
    // Check if Google Play Services are available
    await GoogleSignin.hasPlayServices();

    // Sign in with Google
    const userInfo = await GoogleSignin.signIn();
    
    if (!userInfo.data?.user) {
      return { success: false, error: 'Google 登入失敗：無法取得使用者資訊' };
    }

    const googleUser = userInfo.data.user;
    const email = googleUser.email;
    const displayName = googleUser.name || googleUser.givenName || 'Google User';
    const googleId = googleUser.id;
    const photoUrl = googleUser.photo || undefined;

    // Check if user already exists
    const usersCollection = database.get<User>('users');
    const existingUsers = await usersCollection
      .query(Q.where('email', email))
      .fetch();

    let user: User;

    if (existingUsers.length > 0) {
      // User exists, update OAuth info if needed
      user = existingUsers[0];
      await database.write(async () => {
        await user.update((u) => {
          if (!u.googleId) u.googleId = googleId;
          if (photoUrl && !u.photoUrl) u.photoUrl = photoUrl;
          if (u.status !== 'active') u.status = 'active';
        });
      });
      console.log('✅ Existing user logged in with Google:', user.id);
    } else {
      // Create new user
      user = await database.write(async () => {
        return await usersCollection.create((newUser) => {
          newUser.email = email;
          newUser.displayName = displayName;
          newUser.googleId = googleId;
          newUser.photoUrl = photoUrl;
          newUser.locale = 'zh-TW';
          newUser.theme = 'default';
          newUser.role = 'baby'; // Default role
          newUser.status = 'active';
          // No passwordHash for OAuth users
        });
      });
      console.log('✅ New user created via Google:', user.id);
    }

    return { success: true, user };
  } catch (error: any) {
    console.error('❌ Google login failed:', error);
    
    // Handle specific error codes
    if (error.code === 'SIGN_IN_CANCELLED') {
      return { success: false, error: '使用者取消登入' };
    } else if (error.code === 'IN_PROGRESS') {
      return { success: false, error: '登入進行中，請稍候' };
    } else if (error.code === 'PLAY_SERVICES_NOT_AVAILABLE') {
      return { success: false, error: 'Google Play Services 不可用' };
    }
    
    return { success: false, error: 'Google 登入失敗，請稍後再試' };
  }
}

/**
 * Sign out from Google
 */
export async function signOutGoogle(): Promise<void> {
  try {
    await GoogleSignin.signOut();
    console.log('✅ Signed out from Google');
  } catch (error) {
    console.error('❌ Google sign out failed:', error);
  }
}
