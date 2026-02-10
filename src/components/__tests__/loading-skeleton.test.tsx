import { describe, it, expect } from "vitest";
import { render } from "@testing-library/react";
import { LoadingSkeleton } from "../loading-skeleton";

describe("LoadingSkeleton", () => {
  it("renders default 3 skeleton cards", () => {
    const { container } = render(<LoadingSkeleton />);
    const cards = container.querySelectorAll(".rounded-xl");
    expect(cards.length).toBe(3);
  });

  it("renders custom count of skeleton cards", () => {
    const { container } = render(<LoadingSkeleton count={5} />);
    const cards = container.querySelectorAll(".rounded-xl");
    expect(cards.length).toBe(5);
  });

  it("renders 1 skeleton card", () => {
    const { container } = render(<LoadingSkeleton count={1} />);
    const cards = container.querySelectorAll(".rounded-xl");
    expect(cards.length).toBe(1);
  });

  it("renders 0 skeleton cards when count is 0", () => {
    const { container } = render(<LoadingSkeleton count={0} />);
    const cards = container.querySelectorAll(".rounded-xl");
    expect(cards.length).toBe(0);
  });

  it("contains animate-pulse elements", () => {
    const { container } = render(<LoadingSkeleton count={1} />);
    const pulseElements = container.querySelectorAll(".animate-pulse");
    expect(pulseElements.length).toBeGreaterThan(0);
  });

  it("has grid layout wrapper", () => {
    const { container } = render(<LoadingSkeleton />);
    const grid = container.querySelector(".grid");
    expect(grid).toBeDefined();
    expect(grid).not.toBeNull();
  });
});
