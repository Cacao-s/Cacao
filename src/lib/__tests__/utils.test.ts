import { describe, it, expect } from "vitest";
import { cn } from "../utils";

describe("cn (class name utility)", () => {
  it("merges simple class names", () => {
    expect(cn("foo", "bar")).toBe("foo bar");
  });

  it("handles conditional classes via clsx", () => {
    const isHidden = false;
    expect(cn("base", isHidden && "hidden", "visible")).toBe("base visible");
  });

  it("resolves conflicting Tailwind classes (last wins)", () => {
    // tailwind-merge should keep only the last padding
    const result = cn("p-4", "p-8");
    expect(result).toBe("p-8");
  });

  it("resolves conflicting Tailwind text colors", () => {
    const result = cn("text-red-500", "text-blue-500");
    expect(result).toBe("text-blue-500");
  });

  it("preserves non-conflicting classes", () => {
    const result = cn("p-4", "m-2", "text-lg");
    expect(result).toContain("p-4");
    expect(result).toContain("m-2");
    expect(result).toContain("text-lg");
  });

  it("handles undefined and null values", () => {
    const result = cn("base", undefined, null, "end");
    expect(result).toBe("base end");
  });

  it("handles empty string", () => {
    const result = cn("");
    expect(result).toBe("");
  });

  it("handles no arguments", () => {
    const result = cn();
    expect(result).toBe("");
  });

  it("handles array inputs", () => {
    const result = cn(["foo", "bar"]);
    expect(result).toBe("foo bar");
  });

  it("handles object inputs (conditional)", () => {
    const result = cn({ "bg-red-500": true, "bg-blue-500": false });
    expect(result).toBe("bg-red-500");
  });
});
