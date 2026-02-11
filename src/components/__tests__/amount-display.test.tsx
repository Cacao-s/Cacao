import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { AmountDisplay } from "../amount-display";

describe("AmountDisplay", () => {
  it("renders formatted amount", () => {
    render(<AmountDisplay cents={10000} />);
    expect(screen.getByText(/100/)).toBeDefined();
  });

  it("renders zero amount", () => {
    render(<AmountDisplay cents={0} />);
    expect(screen.getByText(/0/)).toBeDefined();
  });

  it("shows + sign for positive when showSign is true", () => {
    const { container } = render(<AmountDisplay cents={5000} showSign />);
    expect(container.textContent).toContain("+");
  });

  it("shows - sign for negative when showSign is true", () => {
    const { container } = render(<AmountDisplay cents={-5000} showSign />);
    expect(container.textContent).toContain("-");
  });

  it("does not show sign when showSign is false", () => {
    const { container } = render(<AmountDisplay cents={5000} />);
    expect(container.textContent).not.toContain("+");
  });

  it("does not show sign for zero even when showSign is true", () => {
    const { container } = render(<AmountDisplay cents={0} showSign />);
    expect(container.textContent).not.toContain("+");
    expect(container.textContent).not.toMatch(/-/);
  });

  it("applies text-destructive class for negative cents", () => {
    const { container } = render(<AmountDisplay cents={-100} />);
    const span = container.querySelector("span");
    expect(span?.className).toContain("text-destructive");
  });

  it("does not apply text-destructive for positive cents", () => {
    const { container } = render(<AmountDisplay cents={100} />);
    const span = container.querySelector("span");
    expect(span?.className).not.toContain("text-destructive");
  });

  it("applies custom className", () => {
    const { container } = render(<AmountDisplay cents={100} className="text-xl font-bold" />);
    const span = container.querySelector("span");
    expect(span?.className).toContain("text-xl");
    expect(span?.className).toContain("font-bold");
  });

  it("defaults to TWD currency", () => {
    const { container } = render(<AmountDisplay cents={5000} />);
    // TWD format should contain $ or NT$
    expect(container.textContent).toMatch(/\$|NT/);
  });
});

// ── R48: AmountDisplay Extended ─────────────────────────────

describe("AmountDisplay extended", () => {
  it("renders USD currency symbol", () => {
    const { container } = render(<AmountDisplay cents={10000} currency="USD" />);
    expect(container.textContent).toMatch(/\$|US/);
  });

  it("renders JPY currency", () => {
    const { container } = render(<AmountDisplay cents={10000} currency="JPY" />);
    expect(container.textContent).toContain("100");
  });

  it("does not apply destructive class for zero cents", () => {
    const { container } = render(<AmountDisplay cents={0} />);
    const span = container.querySelector("span");
    expect(span?.className).not.toContain("text-destructive");
  });

  it("formats large negative value with sign and destructive", () => {
    const { container } = render(<AmountDisplay cents={-9999999} showSign />);
    expect(container.textContent).toContain("-");
    const span = container.querySelector("span");
    expect(span?.className).toContain("text-destructive");
  });

  it("formats 1 cent amount", () => {
    const { container } = render(<AmountDisplay cents={1} />);
    expect(container.textContent).toBeDefined();
  });

  it("renders without sign for negative when showSign false", () => {
    const { container } = render(<AmountDisplay cents={-5000} showSign={false} />);
    // Uses Math.abs for the formatted value, so no explicit sign added
    expect(container.textContent).not.toMatch(/^-/);
  });
});
