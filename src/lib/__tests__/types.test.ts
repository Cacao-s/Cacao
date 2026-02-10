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
