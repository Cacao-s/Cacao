import { describe, it, expect } from "vitest";
import { REQUEST_CATEGORIES } from "../types";
import type { RequestCategory } from "../types";

describe("REQUEST_CATEGORIES", () => {
  it("has 7 categories", () => {
    expect(REQUEST_CATEGORIES.length).toBe(7);
  });

  it("contains all expected categories", () => {
    const values = REQUEST_CATEGORIES.map((c) => c.value);
    const expected: RequestCategory[] = [
      "food",
      "transport",
      "education",
      "entertainment",
      "clothing",
      "health",
      "other",
    ];
    for (const cat of expected) {
      expect(values).toContain(cat);
    }
  });

  it("has no duplicate values", () => {
    const values = REQUEST_CATEGORIES.map((c) => c.value);
    expect(new Set(values).size).toBe(values.length);
  });

  it("has no empty labels", () => {
    for (const cat of REQUEST_CATEGORIES) {
      expect(cat.label.length).toBeGreaterThan(0);
    }
  });

  it("each entry has value and label", () => {
    for (const cat of REQUEST_CATEGORIES) {
      expect(typeof cat.value).toBe("string");
      expect(typeof cat.label).toBe("string");
    }
  });
});

// ── R46: Type definitions validation ────────────────────────

describe("REQUEST_CATEGORIES ordering and labels", () => {
  it("first category is food", () => {
    expect(REQUEST_CATEGORIES[0].value).toBe("food");
  });

  it("last category is other", () => {
    expect(REQUEST_CATEGORIES[REQUEST_CATEGORIES.length - 1].value).toBe("other");
  });

  it("all labels are in Chinese", () => {
    // Chinese characters are in CJK range
    const cjkPattern = /[\u4e00-\u9fff]/;
    for (const cat of REQUEST_CATEGORIES) {
      expect(cjkPattern.test(cat.label)).toBe(true);
    }
  });

  it("labels have 1-4 characters", () => {
    for (const cat of REQUEST_CATEGORIES) {
      expect(cat.label.length).toBeGreaterThanOrEqual(1);
      expect(cat.label.length).toBeLessThanOrEqual(4);
    }
  });

  it("no duplicate labels", () => {
    const labels = REQUEST_CATEGORIES.map((c) => c.label);
    expect(new Set(labels).size).toBe(labels.length);
  });
});
