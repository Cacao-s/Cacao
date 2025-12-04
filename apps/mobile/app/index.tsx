import { View, Text, StyleSheet, Button, Alert, ScrollView } from 'react-native';
import { testDatabaseConnection, testAllModels, cleanupTestData } from '../src/utils/testDatabase';
import { seedDatabase, clearDatabase } from '../src/utils/seedDatabase';
import { registerUser, loginUser } from '../src/services/authService';

export default function Index() {
  const handleTestDatabase = async () => {
    const success = await testDatabaseConnection();
    if (success) {
      Alert.alert('成功', '資料庫測試通過!檢查 console 查看詳細資訊');
    } else {
      Alert.alert('失敗', '資料庫測試失敗,請檢查 console 錯誤訊息');
    }
  };

  const handleTestAllModels = async () => {
    const success = await testAllModels();
    if (success) {
      Alert.alert('成功', '所有 models 測試通過!');
    } else {
      Alert.alert('失敗', '測試失敗,請檢查 console');
    }
  };

  const handleSeedDatabase = async () => {
    const result = await seedDatabase();
    if (result.success) {
      Alert.alert(
        '成功',
        '測試資料已建立!\n\n測試帳號:\n' +
          'giver@example.com\n' +
          'baby@example.com\n' +
          'parent@example.com\n\n' +
          '密碼: password123'
      );
    } else {
      Alert.alert('提示', result.message || '操作失敗');
    }
  };

  const handleClearDatabase = async () => {
    Alert.alert('確認', '確定要清空所有資料嗎?此操作無法復原', [
      { text: '取消', style: 'cancel' },
      {
        text: '確定',
        style: 'destructive',
        onPress: async () => {
          const result = await clearDatabase();
          if (result.success) {
            Alert.alert('完成', '資料庫已清空');
          } else {
            Alert.alert('失敗', '清空失敗');
          }
        },
      },
    ]);
  };

  const handleTestRegister = async () => {
    const result = await registerUser({
      email: `user_${Date.now()}@test.com`,
      password: 'test123456',
      displayName: 'Test Register User',
    });

    if (result.success) {
      Alert.alert('成功', `註冊成功!\nUser ID: ${result.user?.id}`);
    } else {
      Alert.alert('失敗', result.error || '註冊失敗');
    }
  };

  const handleTestLogin = async () => {
    console.log('🔍 開始測試登入...');
    const result = await loginUser({
      email: 'giver@example.com',
      password: 'password123',
    });

    console.log('登入結果:', result);
    if (result.success) {
      Alert.alert('成功', `登入成功!\n歡迎 ${result.user?.displayName}`);
    } else {
      Alert.alert('失敗', result.error || '登入失敗');
      console.error('登入失敗原因:', result.error);
    }
  };

  const handleCleanup = async () => {
    await cleanupTestData();
    Alert.alert('完成', '測試資料已清理');
  };

  return (
    <ScrollView contentContainerStyle={styles.container}>
      <Text style={styles.title}>Welcome to Cacao</Text>
      <Text style={styles.subtitle}>Family Allowance Management Platform</Text>

      <View style={styles.section}>
        <Text style={styles.sectionTitle}>資料庫測試</Text>
        <View style={styles.buttonContainer}>
          <Button title="測試資料庫連接" onPress={handleTestDatabase} />
          <View style={styles.buttonSpacer} />
          <Button title="測試所有 Models" onPress={handleTestAllModels} />
        </View>
      </View>

      <View style={styles.section}>
        <Text style={styles.sectionTitle}>測試資料管理</Text>
        <View style={styles.buttonContainer}>
          <Button title="建立測試資料" onPress={handleSeedDatabase} color="#4CAF50" />
          <View style={styles.buttonSpacer} />
          <Button title="清空資料庫" onPress={handleClearDatabase} color="#f44336" />
        </View>
      </View>

      <View style={styles.section}>
        <Text style={styles.sectionTitle}>認證功能測試</Text>
        <View style={styles.buttonContainer}>
          <Button title="測試註冊" onPress={handleTestRegister} color="#2196F3" />
          <View style={styles.buttonSpacer} />
          <Button title="測試登入 (Giver)" onPress={handleTestLogin} color="#2196F3" />
        </View>
      </View>

      <View style={styles.section}>
        <View style={styles.buttonContainer}>
          <Button title="清理測試資料" onPress={handleCleanup} color="#888" />
        </View>
      </View>
    </ScrollView>
  );
}

const styles = StyleSheet.create({
  container: {
    flexGrow: 1,
    padding: 20,
    paddingTop: 60,
    backgroundColor: '#fff',
  },
  title: {
    fontSize: 24,
    fontWeight: 'bold',
    marginBottom: 8,
    textAlign: 'center',
  },
  subtitle: {
    fontSize: 16,
    color: '#666',
    marginBottom: 32,
    textAlign: 'center',
  },
  section: {
    marginBottom: 24,
  },
  sectionTitle: {
    fontSize: 18,
    fontWeight: '600',
    marginBottom: 12,
    color: '#333',
  },
  buttonContainer: {
    width: '100%',
  },
  buttonSpacer: {
    height: 12,
  },
});
