import { database } from '../database';
import { User, Family, FamilyMember, Wallet } from '../models';
import * as Crypto from 'expo-crypto';

/**
 * Hash password using SHA256
 */
async function hashPassword(password: string): Promise<string> {
  const hash = await Crypto.digestStringAsync(
    Crypto.CryptoDigestAlgorithm.SHA256,
    password
  );
  return hash;
}

/**
 * Seed database with test data for development
 * Creates test users, families, members, and wallets
 */
export async function seedDatabase() {
  try {
    console.log('🌱 Starting database seeding...');

    // Check if data already exists
    const usersCollection = database.get<User>('users');
    const existingUsers = await usersCollection.query().fetch();

    if (existingUsers.length > 0) {
      console.log('⚠️ Database already has data. Skipping seed.');
      return { success: false, message: '資料庫已有資料' };
    }

    // Create test users
    const passwordHash = await hashPassword('password123');

    const giver = await database.write(async () => {
      return await usersCollection.create((user) => {
        user.email = 'giver@example.com';
        user.passwordHash = passwordHash;
        user.displayName = 'Primary Giver';
        user.locale = 'zh-TW';
        user.theme = 'default';
        user.role = 'giver';
        user.status = 'active';
      });
    });
    console.log('✅ Created giver user:', giver.id);

    const baby = await database.write(async () => {
      return await usersCollection.create((user) => {
        user.email = 'baby@example.com';
        user.passwordHash = passwordHash;
        user.displayName = 'Little Baby';
        user.locale = 'zh-TW';
        user.theme = 'default';
        user.role = 'baby';
        user.status = 'active';
      });
    });
    console.log('✅ Created baby user:', baby.id);

    const parent = await database.write(async () => {
      return await usersCollection.create((user) => {
        user.email = 'parent@example.com';
        user.passwordHash = passwordHash;
        user.displayName = 'Caring Parent';
        user.locale = 'zh-TW';
        user.theme = 'default';
        user.role = 'giver';
        user.status = 'active';
      });
    });
    console.log('✅ Created parent user:', parent.id);

    // Create a family
    const familiesCollection = database.get<Family>('families');
    const demoFamily = await database.write(async () => {
      return await familiesCollection.create((family) => {
        family.name = 'Demo Family';
        family.currency = 'TWD';
        family.timezone = 'Asia/Taipei';
        family.createdBy = giver.id;
      });
    });
    console.log('✅ Created demo family:', demoFamily.id);

    // Add family members
    const familyMembersCollection = database.get<FamilyMember>('family_members');

    const giverMember = await database.write(async () => {
      return await familyMembersCollection.create((member) => {
        member.familyId = demoFamily.id;
        member.userId = giver.id;
        member.familyRole = 'giver';
        member.status = 'active';
        member.joinedAt = new Date();
      });
    });
    console.log('✅ Added giver as family member');

    const babyMember = await database.write(async () => {
      return await familyMembersCollection.create((member) => {
        member.familyId = demoFamily.id;
        member.userId = baby.id;
        member.familyRole = 'baby';
        member.status = 'active';
        member.joinedAt = new Date();
        member.invitedBy = giver.id;
      });
    });
    console.log('✅ Added baby as family member');

    const parentMember = await database.write(async () => {
      return await familyMembersCollection.create((member) => {
        member.familyId = demoFamily.id;
        member.userId = parent.id;
        member.familyRole = 'giver';
        member.status = 'active';
        member.joinedAt = new Date();
        member.invitedBy = giver.id;
      });
    });
    console.log('✅ Added parent as family member');

    // Create wallets for the baby
    const walletsCollection = database.get<Wallet>('wallets');

    const cashWallet = await database.write(async () => {
      return await walletsCollection.create((wallet) => {
        wallet.familyId = demoFamily.id;
        wallet.name = "Baby's Cash";
        wallet.type = 'cash';
        wallet.balanceCents = 50000; // $500
        wallet.currency = 'TWD';
        wallet.warningThresholdCents = 10000; // $100
        wallet.status = 'active';
      });
    });
    console.log('✅ Created cash wallet:', cashWallet.id);

    const bankWallet = await database.write(async () => {
      return await walletsCollection.create((wallet) => {
        wallet.familyId = demoFamily.id;
        wallet.name = "Baby's Bank Account";
        wallet.type = 'bank';
        wallet.balanceCents = 1000000; // $10,000
        wallet.currency = 'TWD';
        wallet.warningThresholdCents = 50000; // $500
        wallet.status = 'active';
      });
    });
    console.log('✅ Created bank wallet:', bankWallet.id);

    console.log('🎉 Database seeding completed successfully!');
    console.log('');
    console.log('📋 Test accounts:');
    console.log('  Giver: giver@example.com / password123');
    console.log('  Baby: baby@example.com / password123');
    console.log('  Parent: parent@example.com / password123');

    return {
      success: true,
      users: { giver, baby, parent },
      family: demoFamily,
      wallets: { cash: cashWallet, bank: bankWallet },
    };
  } catch (error) {
    console.error('❌ Database seeding failed:', error);
    return { success: false, error };
  }
}

/**
 * Clear all data from the database
 */
export async function clearDatabase() {
  try {
    console.log('🧹 Clearing database...');

    const collections = [
      'users',
      'families',
      'family_members',
      'wallets',
      'allowances',
      'requests',
      'transactions',
      'notifications',
      'sync_queue',
      'audit_logs',
    ];

    for (const collectionName of collections) {
      const collection = database.get(collectionName);
      const records = await collection.query().fetch();

      await database.write(async () => {
        for (const record of records) {
          await record.destroyPermanently();
        }
      });

      console.log(`✅ Cleared ${records.length} records from ${collectionName}`);
    }

    console.log('🎉 Database cleared successfully!');
    return { success: true };
  } catch (error) {
    console.error('❌ Clear database failed:', error);
    return { success: false, error };
  }
}
