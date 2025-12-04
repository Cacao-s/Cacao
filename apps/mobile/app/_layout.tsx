import { Stack } from 'expo-router';
import { DatabaseProvider } from '../src/database/DatabaseProvider';

export default function RootLayout() {
  return (
    <DatabaseProvider>
      <Stack>
        <Stack.Screen name="index" options={{ headerShown: false }} />
      </Stack>
    </DatabaseProvider>
  );
}
