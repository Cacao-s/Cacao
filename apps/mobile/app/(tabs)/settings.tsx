import { View, Text, Switch, StyleSheet, Button, Pressable } from 'react-native';
import { useMemo, useState } from 'react';
import { useRouter } from 'expo-router';
import { MaterialCommunityIcons } from '@expo/vector-icons';
import { useAuth } from '../../features/auth/auth-provider';
import { useTheme, type ThemeName } from '../../theme';

export default function SettingsScreen() {
  const [notificationsEnabled, setNotificationsEnabled] = useState(true);
  const { logout } = useAuth();
  const router = useRouter();
  const { name: currentTheme, setTheme, theme } = useTheme();

  const themeOptions = useMemo(
    () => [
      { label: 'Light', description: 'Default、亮色背景', value: 'light' as ThemeName },
      { label: 'Dark', description: '低亮度、深色背景', value: 'dark' as ThemeName },
      { label: 'High Contrast', description: '高對比度、輔助可讀', value: 'highContrast' as ThemeName },
    ],
    [],
  );

  const handleSignOut = () => {
    logout();
    router.replace('/auth/login');
  };

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <View style={styles.row}>
        <Text style={[styles.label, { color: theme.text }]}>Push notifications</Text>
        <Switch value={notificationsEnabled} onValueChange={setNotificationsEnabled} />
      </View>
      <View style={styles.sectionHeader}>
        <Text style={[styles.sectionTitle, { color: theme.mutedText }]}>主題</Text>
        <Text style={[styles.sectionCaption, { color: theme.mutedText }]}>預設為 Light，可隨時切換。</Text>
      </View>
      <View style={styles.themeList}>
        {themeOptions.map((option) => {
          const selected = option.value === currentTheme;
          return (
            <Pressable
              key={option.value}
              style={[
                styles.themeOption,
                { borderColor: selected ? theme.accent : theme.border, backgroundColor: theme.card },
              ]}
              onPress={() => setTheme(option.value)}
            >
              <View>
                <Text style={[styles.themeLabel, { color: theme.text }]}>{option.label}</Text>
                <Text style={[styles.themeDescription, { color: theme.mutedText }]}>{option.description}</Text>
              </View>
              {selected ? <MaterialCommunityIcons name="check-circle" color={theme.accent} size={22} /> : null}
            </Pressable>
          );
        })}
      </View>
      <View style={styles.buttonRow}>
        <Button title="Sign out" onPress={handleSignOut} />
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    padding: 24,
  },
  row: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    paddingVertical: 16,
    borderBottomWidth: StyleSheet.hairlineWidth,
    borderBottomColor: '#ccc',
  },
  label: {
    fontSize: 16,
  },
  sectionHeader: {
    marginTop: 32,
    marginBottom: 12,
  },
  sectionTitle: {
    fontSize: 14,
    fontWeight: '600',
    textTransform: 'uppercase',
  },
  sectionCaption: {
    marginTop: 4,
    fontSize: 12,
  },
  themeList: {
    gap: 12,
  },
  themeOption: {
    borderWidth: 1,
    borderRadius: 12,
    padding: 16,
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
  },
  themeLabel: {
    fontSize: 16,
    fontWeight: '600',
  },
  themeDescription: {
    marginTop: 4,
    fontSize: 13,
  },
  buttonRow: {
    marginTop: 32,
  },
});
