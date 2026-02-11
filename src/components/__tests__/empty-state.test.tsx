import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { EmptyState } from "../empty-state";

describe("EmptyState", () => {
  it("renders title", () => {
    render(<EmptyState title="No items" />);
    expect(screen.getByText("No items")).toBeDefined();
  });

  it("renders description when provided", () => {
    render(<EmptyState title="Empty" description="Nothing here yet" />);
    expect(screen.getByText("Nothing here yet")).toBeDefined();
  });

  it("does not render description when not provided", () => {
    const { container } = render(<EmptyState title="Empty" />);
    const paragraphs = container.querySelectorAll("p");
    expect(paragraphs.length).toBe(0);
  });

  it("renders icon when provided", () => {
    render(<EmptyState title="Empty" icon={<span data-testid="test-icon">icon</span>} />);
    expect(screen.getByTestId("test-icon")).toBeDefined();
  });

  it("does not render icon container when not provided", () => {
    const { container } = render(<EmptyState title="Empty" />);
    const muted = container.querySelector(".text-muted-foreground");
    // Only the description uses text-muted-foreground, and we haven't provided that
    // The icon wrapper should not exist
    expect(muted).toBeNull();
  });

  it("renders action when provided", () => {
    render(<EmptyState title="Empty" action={<button data-testid="action-btn">Add</button>} />);
    expect(screen.getByTestId("action-btn")).toBeDefined();
    expect(screen.getByText("Add")).toBeDefined();
  });

  it("renders all props together", () => {
    render(
      <EmptyState
        title="No wallets"
        description="Create your first wallet"
        icon={<span data-testid="icon">W</span>}
        action={<button>Create</button>}
      />,
    );
    expect(screen.getByText("No wallets")).toBeDefined();
    expect(screen.getByText("Create your first wallet")).toBeDefined();
    expect(screen.getByTestId("icon")).toBeDefined();
    expect(screen.getByText("Create")).toBeDefined();
  });
});

// ── R47: EmptyState Extended ────────────────────────────────

describe("EmptyState extended", () => {
  it("renders Chinese title", () => {
    render(<EmptyState title="沒有錢包" />);
    expect(screen.getByText("沒有錢包")).toBeDefined();
  });

  it("renders Chinese description", () => {
    render(<EmptyState title="空" description="建立你的第一個錢包" />);
    expect(screen.getByText("建立你的第一個錢包")).toBeDefined();
  });

  it("renders with long title", () => {
    const longTitle = "A very long title that goes on and on";
    render(<EmptyState title={longTitle} />);
    expect(screen.getByText(longTitle)).toBeDefined();
  });

  it("has centered layout", () => {
    const { container } = render(<EmptyState title="Test" />);
    const wrapper = container.firstElementChild;
    expect(wrapper?.className).toContain("items-center");
    expect(wrapper?.className).toContain("text-center");
  });
});
