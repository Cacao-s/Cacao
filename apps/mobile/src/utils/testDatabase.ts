import { database } from '../database';
import { User, Family, Wallet, Transaction } from '../models';

/**
 * Test WatermelonDB connection and basic operations
 * Call this function to verify database setup is working
 */
export async function testDatabaseConnection() {
  try {
    console.log('🔍 Testing WatermelonDB connection...');

    // Test 1: Get users collection
    const usersCollection = database.get<User>('users');
    console.log('✅ Users collection retrieved');

    // Test 2: Query existing users
    const existingUsers = await usersCollection.query().fetch();
    console.log(`✅ Found ${existingUsers.length} existing users`);

    // Test 3: Create a test user
    await database.write(async () => {
      const newUser = await usersCollection.create((user) => {
        user.email = `test_${Date.now()}@example.com`;
        user.displayName = 'Test User';
        user.locale = 'zh-TW';
        user.theme = 'default';
        user.role = 'baby';
        user.status = 'active';
      });
      console.log('✅ Created test user:', newUser.id);
    });

    // Test 4: Query all users again
    const allUsers = await usersCollection.query().fetch();
    console.log(`✅ Total users after insert: ${allUsers.length}`);
    allUsers.forEach((user) => {
      console.log(`  - ${user.displayName} (${user.email})`);
    });

    console.log('🎉 Database connection test completed successfully!');
    return true;
  } catch (error) {
    console.error('❌ Database connection test failed:', error);
    return false;
  }
}

/**
 * Test all models with CRUD operations
 */
export async function testAllModels() {
  try {
    console.log('🧪 Testing all models...');

    // Test families
    const familiesCollection = database.get<Family>('families');
    const families = await familiesCollection.query().fetch();
    console.log(`✅ Families: ${families.length} records`);

    // Test wallets
    const walletsCollection = database.get<Wallet>('wallets');
    const wallets = await walletsCollection.query().fetch();
    console.log(`✅ Wallets: ${wallets.length} records`);
    wallets.forEach((wallet) => {
      const balance = wallet.balanceCents / 100;
      console.log(`  - ${wallet.name}: ${wallet.currency} $${balance}`);
    });

    // Test transactions
    const transactionsCollection = database.get<Transaction>('transactions');
    const transactions = await transactionsCollection.query().fetch();
    console.log(`✅ Transactions: ${transactions.length} records`);

    console.log('🎉 All models test completed!');
    return true;
  } catch (error) {
    console.error('❌ Models test failed:', error);
    return false;
  }
}

/**
 * Clean up test data
 */
export async function cleanupTestData() {
  try {
    const usersCollection = database.get<User>('users');
    const testUsers = await usersCollection.query().fetch();

    await database.write(async () => {
      for (const user of testUsers) {
        if (user.email.startsWith('test_')) {
          await user.destroyPermanently();
        }
      }
    });

    console.log('🧹 Test data cleaned up');
  } catch (error) {
    console.error('❌ Cleanup failed:', error);
  }
}
