import { describe, it, expect } from "vitest";
import zhTW from "../zh-TW.json";
import en from "../en.json";

/**
 * Recursively collect all leaf keys from a nested object.
 * e.g. { a: { b: "x" } } => ["a.b"]
 */
function collectKeys(obj: Record<string, unknown>, prefix = ""): string[] {
  const keys: string[] = [];
  for (const [key, value] of Object.entries(obj)) {
    const fullKey = prefix ? `${prefix}.${key}` : key;
    if (typeof value === "object" && value !== null && !Array.isArray(value)) {
      keys.push(...collectKeys(value as Record<string, unknown>, fullKey));
    } else {
      keys.push(fullKey);
    }
  }
  return keys.sort();
}

describe("i18n key completeness", () => {
  const zhKeys = collectKeys(zhTW);
  const enKeys = collectKeys(en);

  it("zh-TW and en have the same number of keys", () => {
    expect(zhKeys.length).toBe(enKeys.length);
  });

  it("every zh-TW key exists in en", () => {
    const missingInEn = zhKeys.filter((k) => !enKeys.includes(k));
    expect(missingInEn).toEqual([]);
  });

  it("every en key exists in zh-TW", () => {
    const missingInZh = enKeys.filter((k) => !zhKeys.includes(k));
    expect(missingInZh).toEqual([]);
  });

  it("no zh-TW values are empty strings", () => {
    const emptyKeys = zhKeys.filter((key) => {
      const value = key.split(".").reduce((obj: unknown, k) => {
        return (obj as Record<string, unknown>)?.[k];
      }, zhTW);
      return value === "";
    });
    expect(emptyKeys).toEqual([]);
  });

  it("no en values are empty strings", () => {
    const emptyKeys = enKeys.filter((key) => {
      const value = key.split(".").reduce((obj: unknown, k) => {
        return (obj as Record<string, unknown>)?.[k];
      }, en);
      return value === "";
    });
    expect(emptyKeys).toEqual([]);
  });

  it("has expected top-level namespaces", () => {
    const zhTopLevel = Object.keys(zhTW).sort();
    const expectedNamespaces = [
      "allowance",
      "audit",
      "common",
      "dashboard",
      "family",
      "lock",
      "notification",
      "request",
      "settings",
      "setup",
      "sync",
      "transaction",
      "wallet",
    ];

    for (const ns of expectedNamespaces) {
      expect(zhTopLevel).toContain(ns);
    }
  });
});

// ── R7: i18n Value Quality ──────────────────────────────────

describe("i18n value quality", () => {
  const zhKeys = collectKeys(zhTW);
  const enKeys = collectKeys(en);

  function getValue(obj: Record<string, unknown>, path: string): unknown {
    return path.split(".").reduce((o: unknown, k) => {
      return (o as Record<string, unknown>)?.[k];
    }, obj);
  }

  it("all zh-TW values are strings", () => {
    for (const key of zhKeys) {
      const value = getValue(zhTW, key);
      expect(typeof value).toBe("string");
    }
  });

  it("all en values are strings", () => {
    for (const key of enKeys) {
      const value = getValue(en, key);
      expect(typeof value).toBe("string");
    }
  });

  it("zh-TW values contain CJK characters where expected", () => {
    // At least most zh-TW values should contain CJK characters
    const cjkRegex = /[\u4e00-\u9fff]/;
    let cjkCount = 0;
    for (const key of zhKeys) {
      const value = getValue(zhTW, key) as string;
      if (cjkRegex.test(value)) cjkCount++;
    }
    // Most values should be in Chinese
    expect(cjkCount).toBeGreaterThan(zhKeys.length * 0.5);
  });

  it("en values do not contain CJK characters", () => {
    const cjkRegex = /[\u4e00-\u9fff]/;
    for (const key of enKeys) {
      const value = getValue(en, key) as string;
      expect(cjkRegex.test(value)).toBe(false);
    }
  });

  it("no values contain only whitespace", () => {
    for (const key of zhKeys) {
      const value = getValue(zhTW, key) as string;
      if (value.length > 0) {
        expect(value.trim().length).toBeGreaterThan(0);
      }
    }
    for (const key of enKeys) {
      const value = getValue(en, key) as string;
      if (value.length > 0) {
        expect(value.trim().length).toBeGreaterThan(0);
      }
    }
  });
});
