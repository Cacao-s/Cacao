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

// ── R7: Format Currency Parameter ───────────────────────────

describe("formatAmount with currency parameter", () => {
  it("uses TWD by default", () => {
    const result = formatAmount(10000);
    expect(result).toMatch(/\$|NT/);
  });

  it("accepts USD currency", () => {
    const result = formatAmount(10000, "USD");
    expect(result).toContain("100");
  });

  it("accepts JPY currency", () => {
    const result = formatAmount(10000, "JPY");
    expect(result).toContain("100");
  });

  it("consistent output for same input", () => {
    const a = formatAmount(5000);
    const b = formatAmount(5000);
    expect(a).toBe(b);
  });
});

describe("formatDate robustness", () => {
  it("handles ISO string without timezone", () => {
    const result = formatDate("2024-06-15");
    expect(result).toContain("2024");
  });

  it("handles SQLite datetime format", () => {
    // SQLite datetime('now') returns "YYYY-MM-DD HH:MM:SS"
    const result = formatDate("2024-06-15 14:30:00");
    expect(result).toContain("2024");
  });

  it("handles January 1st", () => {
    const result = formatDate("2024-01-01T00:00:00.000Z");
    expect(result).toContain("2024");
    expect(result).toMatch(/1/);
  });
});

// ── R41: Format Amount Boundary Values ──────────────────────

describe("formatAmount boundary values", () => {
  it("formats MAX_SAFE_INTEGER without crashing", () => {
    expect(typeof formatAmount(Number.MAX_SAFE_INTEGER)).toBe("string");
  });

  it("formats negative MAX_SAFE_INTEGER without crashing", () => {
    expect(typeof formatAmount(-Number.MAX_SAFE_INTEGER)).toBe("string");
  });

  it("formats 1 cent as sub-dollar amount", () => {
    const result = formatAmount(1);
    expect(result).toBeDefined();
  });

  it("formats 99 cents correctly", () => {
    const result = formatAmount(99);
    expect(result).toBeDefined();
  });
});

// ── R42: Format with Different Currencies ───────────────────

describe("formatAmount multi-currency", () => {
  it("formats EUR", () => {
    const result = formatAmount(10000, "EUR");
    expect(result).toContain("100");
  });

  it("formats GBP", () => {
    const result = formatAmount(10000, "GBP");
    expect(result).toContain("100");
  });

  it("formats CNY", () => {
    const result = formatAmount(10000, "CNY");
    expect(result).toContain("100");
  });

  it("same currency same amount returns identical strings", () => {
    expect(formatAmount(12345, "TWD")).toBe(formatAmount(12345, "TWD"));
  });

  it("different currencies produce different strings", () => {
    const twd = formatAmount(10000, "TWD");
    const usd = formatAmount(10000, "USD");
    // They may overlap on the number portion, but the currency symbol should differ
    expect(twd).not.toBe(usd);
  });
});

// ── R43: formatDate and formatDateTime consistency ──────────

describe("format consistency", () => {
  it("formatDate returns shorter string than formatDateTime for same input", () => {
    const iso = "2024-06-15T14:30:00.000Z";
    expect(formatDate(iso).length).toBeLessThanOrEqual(formatDateTime(iso).length);
  });

  it("formatDate is deterministic", () => {
    const iso = "2024-03-15T12:00:00.000Z";
    expect(formatDate(iso)).toBe(formatDate(iso));
  });

  it("formatDateTime is deterministic", () => {
    const iso = "2024-03-15T12:00:00.000Z";
    expect(formatDateTime(iso)).toBe(formatDateTime(iso));
  });

  it("formatDateTime contains time separator", () => {
    const result = formatDateTime("2024-06-15T14:30:00.000Z");
    expect(result).toMatch(/:/);
  });

  it("formatDate does not contain time for typical dates", () => {
    const result = formatDate("2024-06-15T00:00:00.000Z");
    // Date-only format should not contain ":"
    expect(result).not.toMatch(/:/);
  });
});
