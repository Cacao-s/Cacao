import { useEffect, useState } from 'react';
import { ActivityIndicator, Button, StyleSheet, Text, TextInput, View } from 'react-native';
import { useRouter } from 'expo-router';
import { useAuth } from '../../features/auth/auth-provider';

export default function LoginScreen() {
  const router = useRouter();
  const { login, status, error, token } = useAuth();

  const [username, setUsername] = useState('amanda');
  const [password, setPassword] = useState('1234');

  useEffect(() => {
    if (token) {
      router.replace('/(tabs)');
    }
  }, [router, token]);

  const handleSubmit = async () => {
    try {
      await login(username.trim(), password);
      router.replace('/(tabs)');
    } catch {
      // error state already handled in provider
    }
  };

  return (
    <View style={styles.container}>
      <Text style={styles.title}>Cacao</Text>
      <Text style={styles.subtitle}>Sign in to manage family allowances</Text>

      <TextInput
        value={username}
        onChangeText={setUsername}
        placeholder="Username"
        autoCapitalize="none"
        style={styles.input}
        editable={status !== 'signingIn'}
      />
      <TextInput
        value={password}
        onChangeText={setPassword}
        placeholder="Password"
        secureTextEntry
        style={styles.input}
        editable={status !== 'signingIn'}
      />

      {error ? <Text style={styles.error}>{error}</Text> : null}

      <View style={styles.buttonWrapper}>
        {status === 'signingIn' ? (
          <ActivityIndicator />
        ) : (
          <Button title="Sign in" onPress={handleSubmit} />
        )}
      </View>

      <Text style={styles.hint}>Use amanda / 1234 for now.</Text>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    padding: 24,
    backgroundColor: '#fff',
  },
  title: {
    fontSize: 32,
    fontWeight: '700',
  },
  subtitle: {
    marginTop: 8,
    marginBottom: 24,
    color: '#555',
  },
  input: {
    borderWidth: 1,
    borderColor: '#ccc',
    borderRadius: 8,
    paddingHorizontal: 16,
    paddingVertical: 12,
    marginTop: 12,
  },
  buttonWrapper: {
    marginTop: 24,
  },
  error: {
    marginTop: 12,
    color: '#b42318',
  },
  hint: {
    marginTop: 16,
    color: '#666',
    fontSize: 12,
  },
});
