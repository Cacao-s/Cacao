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
