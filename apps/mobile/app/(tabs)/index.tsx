import { useQuery } from '@tanstack/react-query';
import { View, Text, StyleSheet, FlatList } from 'react-native';

const mockDashboard = async () => {
  return {
    balance: 12500,
    pendingRequests: 2,
    upcomingAllowance: '2025-11-20',
    recentActivity: [
      { id: 'txn_1', label: 'Allowance', amount: 3000 },
      { id: 'txn_2', label: 'Book reimbursement', amount: -520 },
      { id: 'txn_3', label: 'Manual top-up', amount: 1200 },
    ],
  };
};

export default function DashboardScreen() {
  const { data } = useQuery({ queryKey: ['dashboard'], queryFn: mockDashboard });

  if (!data) {
    return null;
  }

  return (
    <View style={styles.container}>
      <Text style={styles.title}>Family Wallet</Text>
      <Text style={styles.balance}>${(data.balance / 100).toFixed(2)}</Text>
      <Text style={styles.caption}>Pending requests: {data.pendingRequests}</Text>
      <Text style={styles.caption}>Next allowance run: {data.upcomingAllowance}</Text>

      <Text style={styles.section}>Recent activity</Text>
      <FlatList
        data={data.recentActivity}
        keyExtractor={(item) => item.id}
        renderItem={({ item }) => (
          <View style={styles.row}>
            <Text style={styles.rowLabel}>{item.label}</Text>
            <Text style={[styles.rowAmount, item.amount < 0 && styles.negative]}>
              {(item.amount / 100).toFixed(2)}
            </Text>
          </View>
        )}
      />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    padding: 24,
    backgroundColor: '#fff',
  },
  title: {
    fontSize: 18,
    fontWeight: '600',
    marginBottom: 4,
  },
  balance: {
    fontSize: 40,
    fontWeight: 'bold',
  },
  caption: {
    color: '#555',
    marginTop: 4,
  },
  section: {
    marginTop: 24,
    fontSize: 16,
    fontWeight: '600',
  },
  row: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    paddingVertical: 12,
    borderBottomWidth: StyleSheet.hairlineWidth,
    borderBottomColor: '#ddd',
  },
  rowLabel: {
    fontSize: 15,
  },
  rowAmount: {
    fontSize: 15,
    fontWeight: '600',
  },
  negative: {
    color: '#c1121f',
  },
});
