import { Tabs } from 'expo-router';
import { MaterialCommunityIcons } from '@expo/vector-icons';

type TabBarIconProps = {
  color: string;
  size: number;
  focused: boolean;
};

const renderIcon = (name: keyof typeof MaterialCommunityIcons.glyphMap) => {
  const Icon = ({ color, size }: TabBarIconProps) => (
    <MaterialCommunityIcons name={name} color={color} size={size} />
  );
  Icon.displayName = `${name}-tab-icon`;
  return Icon;
};

export default function TabsLayout() {
  return (
    <Tabs screenOptions={{ headerShown: false }}>
      <Tabs.Screen name="index" options={{ title: 'Dashboard', tabBarIcon: renderIcon('view-dashboard-outline') }} />
      <Tabs.Screen name="requests" options={{ title: 'Requests', tabBarIcon: renderIcon('clipboard-text-outline') }} />
      <Tabs.Screen name="settings" options={{ title: 'Settings', tabBarIcon: renderIcon('cog-outline') }} />
    </Tabs>
  );
}
