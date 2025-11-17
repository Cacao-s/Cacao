import { View, Text, StyleSheet, FlatList, Button } from 'react-native';
import { useState } from 'react';

const initialRequests = [
  { id: 'req_1', title: 'STEM workbook', amount: 850, status: 'pending' },
  { id: 'req_2', title: 'Field trip fee', amount: 1500, status: 'approved' },
];

export default function RequestsScreen() {
  const [requests, setRequests] = useState(initialRequests);

  const onApprove = (id: string) => {
    setRequests((prev) => prev.map((req) => (req.id === id ? { ...req, status: 'approved' } : req)));
  };

  return (
    <View style={styles.container}>
      <FlatList
        data={requests}
        keyExtractor={(item) => item.id}
        renderItem={({ item }) => (
          <View style={styles.card}>
            <Text style={styles.title}>{item.title}</Text>
            <Text style={styles.amount}>${(item.amount / 100).toFixed(2)}</Text>
            <Text style={styles.status}>Status: {item.status}</Text>
            {item.status === 'pending' && <Button title="Approve" onPress={() => onApprove(item.id)} />}
          </View>
        )}
      />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    padding: 16,
    gap: 12,
  },
  card: {
    backgroundColor: '#fff',
    borderRadius: 12,
    padding: 16,
    shadowColor: '#000',
    shadowOpacity: 0.1,
    shadowRadius: 8,
    elevation: 2,
    marginBottom: 12,
  },
  title: {
    fontSize: 16,
    fontWeight: '600',
  },
  amount: {
    marginTop: 4,
    fontSize: 15,
  },
  status: {
    marginTop: 6,
    color: '#666',
  },
});
