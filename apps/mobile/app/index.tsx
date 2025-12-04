import { View, Text, StyleSheet, Button, Alert } from 'react-native';
import { testDatabaseConnection, cleanupTestData } from '../src/utils/testDatabase';

export default function Index() {
  const handleTestDatabase = async () => {
    const success = await testDatabaseConnection();
    if (success) {
      Alert.alert('成功', '資料庫測試通過!檢查 console 查看詳細資訊');
    } else {
      Alert.alert('失敗', '資料庫測試失敗,請檢查 console 錯誤訊息');
    }
  };

  const handleCleanup = async () => {
    await cleanupTestData();
    Alert.alert('完成', '測試資料已清理');
  };

  return (
    <View style={styles.container}>
      <Text style={styles.title}>Welcome to Cacao</Text>
      <Text style={styles.subtitle}>Family Allowance Management Platform</Text>
      
      <View style={styles.buttonContainer}>
        <Button title="測試資料庫連接" onPress={handleTestDatabase} />
        <View style={styles.buttonSpacer} />
        <Button title="清理測試資料" onPress={handleCleanup} color="#888" />
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: '#fff',
  },
  title: {
    fontSize: 24,
    fontWeight: 'bold',
    marginBottom: 8,
  },
  subtitle: {
    fontSize: 16,
    color: '#666',
    marginBottom: 32,
  },
  buttonContainer: {
    marginTop: 16,
    width: 200,
  },
  buttonSpacer: {
    height: 12,
  },
});
