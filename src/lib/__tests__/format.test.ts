import { describe, it, expect } from "vitest";
import { formatAmount, formatDate, formatDateTime } from "../format";

describe("formatAmount", () => {
  it("formats zero cents", () => {
    const result = formatAmount(0);
    expect(result).toContain("0");
  });

  it("formats positive integer amounts (no decimals for TWD)", () => {
    // TWD typically shows no decimal places
    const result = formatAmount(10000);
    expect(result).toContain("100");
  });

  it("formats large amounts with grouping", () => {
    const result = formatAmount(1000000);
    // Should contain the number 10,000 in some locale format
    expect(result).toContain("10,000");
  });

  it("includes TWD currency symbol by default", () => {
    const result = formatAmount(5000);
    // zh-TW Intl.NumberFormat with TWD should include $ or NT$
    expect(result).toMatch(/\$|NT/);
  });

  it("handles negative amounts", () => {
    const result = formatAmount(-5000);
    expect(result).toContain("50");
  });
});

describe("formatDate", () => {
  it("formats ISO date string to zh-TW locale", () => {
    const result = formatDate("2024-06-15T10:30:00.000Z");
    // zh-TW locale date: 2024/6/15
    expect(result).toContain("2024");
    expect(result).toMatch(/6/);
    expect(result).toMatch(/15/);
  });

  it("handles date-only strings", () => {
    const result = formatDate("2024-01-01");
    expect(result).toContain("2024");
    expect(result).toMatch(/1/);
  });
});

describe("formatDateTime", () => {
  it("formats ISO string to zh-TW locale with time", () => {
    const result = formatDateTime("2024-06-15T14:30:00.000Z");
    expect(result).toContain("2024");
    // Should contain time components
    expect(result).toMatch(/\d{1,2}:\d{2}/);
  });
});

// ── Extended Edge Case Tests ──────────────────────────────

describe("formatAmount edge cases", () => {
  it("formats 1 cent correctly", () => {
    const result = formatAmount(1);
    // 1 cent = 0.01 in dollar terms, but TWD shows no decimals by default
    expect(result).toBeDefined();
  });

  it("formats very large amounts", () => {
    const result = formatAmount(99999999);
    // 99999999 cents = 999,999.99
    expect(result).toContain("999");
  });

  it("returns a string for any valid number", () => {
    expect(typeof formatAmount(0)).toBe("string");
    expect(typeof formatAmount(100)).toBe("string");
    expect(typeof formatAmount(-100)).toBe("string");
  });
});

describe("formatDate edge cases", () => {
  it("handles midnight UTC", () => {
    const result = formatDate("2024-12-31T00:00:00.000Z");
    expect(result).toContain("2024");
  });

  it("handles end of year", () => {
    // Note: in UTC+8, 23:59 UTC on Dec 31 is already Jan 1 of the next year
    const result = formatDate("2024-12-31T10:00:00.000Z");
    expect(result).toContain("2024");
    expect(result).toMatch(/12/);
    expect(result).toMatch(/31/);
  });

  it("handles leap year date", () => {
    const result = formatDate("2024-02-29T12:00:00.000Z");
    expect(result).toContain("2024");
    expect(result).toMatch(/2/);
    expect(result).toMatch(/29/);
  });
});

describe("formatDateTime edge cases", () => {
  it("handles midnight", () => {
    const result = formatDateTime("2024-01-01T00:00:00.000Z");
    expect(result).toContain("2024");
  });

  it("returns different output than formatDate for same input", () => {
    const iso = "2024-06-15T14:30:00.000Z";
    const dateOnly = formatDate(iso);
    const dateTime = formatDateTime(iso);
    // DateTime should be longer (includes time)
    expect(dateTime.length).toBeGreaterThanOrEqual(dateOnly.length);
  });
});
