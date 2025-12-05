import { Database } from '@nozbe/watermelondb';

import { schema } from './schema';
import { DexieAdapter } from './DexieAdapter';

// Web: DexieAdapter with empty model classes (IndexedDB storage)
const adapter = new DexieAdapter({
  schema,
});

export const database = new Database({
  adapter,
  modelClasses: [],
});
