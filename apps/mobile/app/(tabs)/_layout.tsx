import { Tabs } from 'expo-router';
import { useAuth } from '../../src/contexts/AuthContext';

export default function TabsLayout() {
  const { user } = useAuth();

  return (
    <Tabs screenOptions={{ headerShown: true }}>
      <Tabs.Screen
        name="home"
        options={{
          title: '首頁',
          headerTitle: `歡迎, ${user?.name || '用戶'}`,
        }}
      />
      <Tabs.Screen
        name="profile"
        options={{
          title: '個人資料',
        }}
      />
    </Tabs>
  );
}
