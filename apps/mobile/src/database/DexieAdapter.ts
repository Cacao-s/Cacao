import Dexie, { Table } from 'dexie';
import type { DatabaseAdapter } from '@nozbe/watermelondb/adapters/type';
import type { TableName, AppSchema, RecordId } from '@nozbe/watermelondb';
import type { SchemaMigrations } from '@nozbe/watermelondb/Schema/migrations';
import type { SerializedQuery } from '@nozbe/watermelondb/Query';

interface DexieRecord {
  id: string;
  _status?: string;
  _changed?: string;
  [key: string]: any;
}

class CacaoDatabase extends Dexie {
  users!: Table<DexieRecord, string>;
  families!: Table<DexieRecord, string>;
  family_members!: Table<DexieRecord, string>;
  wallets!: Table<DexieRecord, string>;
  allowances!: Table<DexieRecord, string>;
  requests!: Table<DexieRecord, string>;
  transactions!: Table<DexieRecord, string>;
  notifications!: Table<DexieRecord, string>;
  sync_queue!: Table<DexieRecord, string>;
  audit_logs!: Table<DexieRecord, string>;

  constructor() {
    super('cacao');

    this.version(1).stores({
      users: 'id, email, google_sub, created_at, updated_at',
      families: 'id, created_by, created_at, updated_at',
      family_members: 'id, family_id, user_id, created_at, updated_at',
      wallets: 'id, family_id, user_id, created_at, updated_at',
      allowances: 'id, family_id, giver_id, baby_id, created_at, updated_at',
      requests: 'id, family_id, baby_id, giver_id, wallet_id, created_at, updated_at',
      transactions: 'id, wallet_id, request_id, created_by, created_at',
      notifications: 'id, user_id, family_id, created_at, is_read',
      sync_queue: 'id, created_at',
      audit_logs: 'id, user_id, family_id, created_at',
    });
  }
}

export class DexieAdapter implements DatabaseAdapter {
  schema: AppSchema;
  migrations?: SchemaMigrations;
  dbName = 'cacao';
  private db: CacaoDatabase;

  constructor(options: { schema: AppSchema; migrations?: SchemaMigrations }) {
    this.schema = options.schema;
    this.migrations = options.migrations;
    this.db = new CacaoDatabase();
  }

  async batch(operations: any[]): Promise<void> {
    await this.db.transaction('rw', this.getAllTables(), async () => {
      for (const operation of operations) {
        const [type, table, ...args] = operation;
        
        switch (type) {
          case 'create':
            await this.create(table, args[0]);
            break;
          case 'update':
            await this.update(table, args[0]);
            break;
          case 'markAsDeleted':
            await this.markAsDeleted(table, args[0]);
            break;
          case 'destroyPermanently':
            await this.destroyPermanently(table, args[0]);
            break;
        }
      }
    });
  }

  async find(table: TableName<any>, id: string): Promise<any> {
    const record = await (this.db as any)[table].get(id);
    return record || null;
  }

  async query(query: SerializedQuery): Promise<any[]> {
    const { table, description } = query;
    let collection = (this.db as any)[table].toCollection();
    
    // Simple where clause support
    if (description.where && description.where.length > 0) {
      collection = collection.filter((record: any) => {
        return description.where.every((clause: any) => {
          if (clause.type === 'eq') {
            return record[clause.left.column] === clause.right.value;
          }
          return true;
        });
      });
    }
    
    return await collection.toArray();
  }

  async count(query: SerializedQuery): Promise<number> {
    const records = await this.query(query);
    return records.length;
  }

  async unsafeQueryRaw(query: SerializedQuery): Promise<any[]> {
    // Web doesn't support raw SQL, fallback to query
    console.warn('Raw SQL not supported on Web, using query instead');
    return this.query(query);
  }

  getLocal(key: string): Promise<string | null> {
    return Promise.resolve(localStorage.getItem(key));
  }

  setLocal(key: string, value: string): Promise<void> {
    localStorage.setItem(key, value);
    return Promise.resolve();
  }

  removeLocal(key: string): Promise<void> {
    localStorage.removeItem(key);
    return Promise.resolve();
  }

  async getDeletedRecords(table: TableName<any>): Promise<string[]> {
    const records = await (this.db as any)[table]
      .where('_status')
      .equals('deleted')
      .toArray();
    return records.map((r: any) => r.id);
  }

  async destroyDeletedRecords(
    table: TableName<any>,
    recordIds: string[]
  ): Promise<void> {
    await (this.db as any)[table].bulkDelete(recordIds);
  }

  private async create(table: TableName<any>, record: any): Promise<void> {
    await (this.db as any)[table].add(record);
  }

  private async update(table: TableName<any>, record: any): Promise<void> {
    await (this.db as any)[table].put(record);
  }

  private async markAsDeleted(table: TableName<any>, id: string): Promise<void> {
    await (this.db as any)[table].update(id, { _status: 'deleted' });
  }

  private async destroyPermanently(table: TableName<any>, id: string): Promise<void> {
    await (this.db as any)[table].delete(id);
  }

  // Additional required methods for DatabaseAdapter
  async queryIds(query: SerializedQuery): Promise<string[]> {
    const records = await this.query(query);
    return records.map((r: any) => r.id);
  }

  async unsafeLoadFromSync(jsonId: number): Promise<any> {
    // Not used in local-only mode
    return null;
  }

  async provideSyncJson(id: number, json: any): Promise<void> {
    // Not used in local-only mode
  }

  async unsafeResetDatabase(): Promise<void> {
    await this.db.delete();
    this.db = new CacaoDatabase();
  }

  async unsafeExecute(work: any): Promise<void> {
    // Execute work in transaction
    await work(this);
  }

  private getAllTables(): Table[] {
    return [
      this.db.users,
      this.db.families,
      this.db.family_members,
      this.db.wallets,
      this.db.allowances,
      this.db.requests,
      this.db.transactions,
      this.db.notifications,
      this.db.sync_queue,
      this.db.audit_logs,
    ];
  }
}
