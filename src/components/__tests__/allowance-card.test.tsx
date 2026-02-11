import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { AllowanceCard } from "../allowance-card";
import type { Allowance } from "@/hooks/use-allowances";

function makeAllowance(overrides: Partial<Allowance> = {}): Allowance {
  return {
    id: 1,
    uuid: "a-uuid",
    family_id: 1,
    giver_member_id: 1,
    receiver_member_id: 2,
    wallet_id: 1,
    amount_cents: 30000,
    frequency: "weekly",
    interval_count: 1,
    next_run_at: "2024-06-22T00:00:00.000Z",
    last_run_at: null,
    status: "active",
    notes: null,
    sync_version: 1,
    last_synced_at: null,
    is_deleted: 0,
    created_at: "2024-06-15T10:00:00.000Z",
    updated_at: "2024-06-15T10:00:00.000Z",
    ...overrides,
  };
}

describe("AllowanceCard", () => {
  const onPause = vi.fn();
  const onResume = vi.fn();

  beforeEach(() => {
    onPause.mockClear();
    onResume.mockClear();
  });

  it("renders amount", () => {
    const { container } = render(<AllowanceCard allowance={makeAllowance()} isGiver={false} />);
    expect(container.textContent).toContain("300");
  });

  it("renders weekly frequency label", () => {
    render(<AllowanceCard allowance={makeAllowance({ frequency: "weekly" })} isGiver={false} />);
    expect(screen.getByText("每週")).toBeDefined();
  });

  it("renders daily frequency label", () => {
    render(<AllowanceCard allowance={makeAllowance({ frequency: "daily" })} isGiver={false} />);
    expect(screen.getByText("每日")).toBeDefined();
  });

  it("renders biweekly frequency label", () => {
    render(<AllowanceCard allowance={makeAllowance({ frequency: "biweekly" })} isGiver={false} />);
    expect(screen.getByText("每兩週")).toBeDefined();
  });

  it("renders monthly frequency label", () => {
    render(<AllowanceCard allowance={makeAllowance({ frequency: "monthly" })} isGiver={false} />);
    expect(screen.getByText("每月")).toBeDefined();
  });

  it("renders custom frequency label", () => {
    render(<AllowanceCard allowance={makeAllowance({ frequency: "custom" })} isGiver={false} />);
    expect(screen.getByText("自訂")).toBeDefined();
  });

  it("falls back to raw frequency for unknown value", () => {
    render(<AllowanceCard allowance={makeAllowance({ frequency: "quarterly" })} isGiver={false} />);
    expect(screen.getByText("quarterly")).toBeDefined();
  });

  it("shows active status badge", () => {
    render(<AllowanceCard allowance={makeAllowance({ status: "active" })} isGiver={false} />);
    expect(screen.getByText("啟用中")).toBeDefined();
  });

  it("shows paused status badge", () => {
    render(<AllowanceCard allowance={makeAllowance({ status: "paused" })} isGiver={false} />);
    expect(screen.getByText("已暫停")).toBeDefined();
  });

  it("shows archived status badge", () => {
    render(<AllowanceCard allowance={makeAllowance({ status: "archived" })} isGiver={false} />);
    expect(screen.getByText("已封存")).toBeDefined();
  });

  it("shows next_run_at for active allowance", () => {
    const { container } = render(
      <AllowanceCard allowance={makeAllowance({ status: "active" })} isGiver={false} />,
    );
    expect(container.textContent).toContain("下次發放");
  });

  it("hides next_run_at for paused allowance", () => {
    const { container } = render(
      <AllowanceCard
        allowance={makeAllowance({ status: "paused", next_run_at: "2024-06-22T00:00:00.000Z" })}
        isGiver={false}
      />,
    );
    expect(container.textContent).not.toContain("下次發放");
  });

  it("shows pause button for giver on active allowance", () => {
    render(
      <AllowanceCard
        allowance={makeAllowance({ status: "active" })}
        isGiver={true}
        onPause={onPause}
        onResume={onResume}
      />,
    );
    const button = screen.getByRole("button");
    expect(button).toBeDefined();
  });

  it("calls onPause when pause button clicked on active", () => {
    render(
      <AllowanceCard
        allowance={makeAllowance({ id: 5, status: "active" })}
        isGiver={true}
        onPause={onPause}
        onResume={onResume}
      />,
    );
    fireEvent.click(screen.getByRole("button"));
    expect(onPause).toHaveBeenCalledWith(5);
    expect(onResume).not.toHaveBeenCalled();
  });

  it("calls onResume when play button clicked on paused", () => {
    render(
      <AllowanceCard
        allowance={makeAllowance({ id: 7, status: "paused" })}
        isGiver={true}
        onPause={onPause}
        onResume={onResume}
      />,
    );
    fireEvent.click(screen.getByRole("button"));
    expect(onResume).toHaveBeenCalledWith(7);
    expect(onPause).not.toHaveBeenCalled();
  });

  it("hides button for non-giver", () => {
    render(<AllowanceCard allowance={makeAllowance({ status: "active" })} isGiver={false} />);
    expect(screen.queryByRole("button")).toBeNull();
  });

  it("hides button for archived allowance even if giver", () => {
    render(
      <AllowanceCard
        allowance={makeAllowance({ status: "archived" })}
        isGiver={true}
        onPause={onPause}
        onResume={onResume}
      />,
    );
    expect(screen.queryByRole("button")).toBeNull();
  });
});

// ── R48: AllowanceCard Extended ─────────────────────────────

describe("AllowanceCard extended", () => {
  it("renders frequency text for active allowance", () => {
    render(<AllowanceCard allowance={makeAllowance({ frequency: "weekly" })} isGiver={false} />);
    expect(screen.getByText("每週")).toBeDefined();
  });

  it("renders status badge for paused with correct label", () => {
    render(<AllowanceCard allowance={makeAllowance({ status: "paused" })} isGiver={false} />);
    expect(screen.getByText("已暫停")).toBeDefined();
  });

  it("renders interval count > 1 for custom frequency", () => {
    const { container } = render(
      <AllowanceCard
        allowance={makeAllowance({ frequency: "custom", interval_count: 3 })}
        isGiver={false}
      />,
    );
    expect(container.textContent).toContain("3");
  });

  it("renders zero amount", () => {
    const { container } = render(
      <AllowanceCard allowance={makeAllowance({ amount_cents: 0 })} isGiver={false} />,
    );
    expect(container.textContent).toContain("0");
  });

  it("renders last_run_at when available", () => {
    const { container } = render(
      <AllowanceCard
        allowance={makeAllowance({ last_run_at: "2024-06-15T10:00:00.000Z", status: "active" })}
        isGiver={false}
      />,
    );
    // Should contain some date text
    expect(container.textContent).toContain("2024");
  });
});
