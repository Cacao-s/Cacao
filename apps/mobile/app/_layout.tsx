import { Stack, useRouter, useSegments } from 'expo-router';
import { useEffect } from 'react';
import { DatabaseProvider } from '../src/database/DatabaseProvider';
import { AuthProvider, useAuth } from '../src/contexts/AuthContext';

function RootLayoutNav() {
  const { isAuthenticated, isLoading } = useAuth();
  const segments = useSegments();
  const router = useRouter();

  useEffect(() => {
    if (isLoading) return;

    const inAuthGroup = segments[0] === '(tabs)';

    if (!isAuthenticated && inAuthGroup) {
      // Redirect to login if not authenticated
      router.replace('/login');
    } else if (isAuthenticated && !inAuthGroup) {
      // Redirect to home if authenticated
      router.replace('/(tabs)/home');
    }
  }, [isAuthenticated, isLoading, segments]);

  return (
    <Stack>
      <Stack.Screen name="index" options={{ headerShown: false }} />
      <Stack.Screen name="login" options={{ headerShown: false }} />
      <Stack.Screen name="register" options={{ headerShown: false }} />
      <Stack.Screen name="(tabs)" options={{ headerShown: false }} />
    </Stack>
  );
}

export default function RootLayout() {
  return (
    <DatabaseProvider>
      <AuthProvider>
        <RootLayoutNav />
      </AuthProvider>
    </DatabaseProvider>
  );
}
