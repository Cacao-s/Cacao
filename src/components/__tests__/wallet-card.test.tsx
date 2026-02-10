import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { WalletCard } from "../wallet-card";
import type { Wallet } from "@/lib/types";

const mockNavigate = vi.fn();
vi.mock("react-router", () => ({
  useNavigate: () => mockNavigate,
}));

function makeWallet(overrides: Partial<Wallet> = {}): Wallet {
  return {
    id: 1,
    uuid: "w-uuid",
    family_id: 1,
    name: "Main Wallet",
    type: "cash",
    balance_cents: 50000,
    currency: "TWD",
    warning_threshold_cents: 0,
    status: "active",
    sync_version: 1,
    last_synced_at: null,
    is_deleted: 0,
    created_at: "2024-06-15T10:00:00.000Z",
    updated_at: "2024-06-15T10:00:00.000Z",
    ...overrides,
  };
}

describe("WalletCard", () => {
  beforeEach(() => {
    mockNavigate.mockClear();
  });

  it("renders wallet name", () => {
    render(<WalletCard wallet={makeWallet()} />);
    expect(screen.getByText("Main Wallet")).toBeDefined();
  });

  it("renders wallet balance", () => {
    const { container } = render(<WalletCard wallet={makeWallet({ balance_cents: 12345 })} />);
    expect(container.textContent).toContain("123");
  });

  it("shows archived badge for archived wallets", () => {
    render(<WalletCard wallet={makeWallet({ status: "archived" })} />);
    expect(screen.getByText("已封存")).toBeDefined();
  });

  it("does not show archived badge for active wallets", () => {
    render(<WalletCard wallet={makeWallet({ status: "active" })} />);
    expect(screen.queryByText("已封存")).toBeNull();
  });

  it("shows low balance warning when balance < threshold", () => {
    render(
      <WalletCard
        wallet={makeWallet({
          balance_cents: 500,
          warning_threshold_cents: 1000,
        })}
      />,
    );
    expect(screen.getByText("餘額偏低")).toBeDefined();
  });

  it("hides low balance warning when threshold is 0", () => {
    render(
      <WalletCard
        wallet={makeWallet({
          balance_cents: 500,
          warning_threshold_cents: 0,
        })}
      />,
    );
    expect(screen.queryByText("餘額偏低")).toBeNull();
  });

  it("hides low balance warning for archived wallets even if below threshold", () => {
    render(
      <WalletCard
        wallet={makeWallet({
          balance_cents: 500,
          warning_threshold_cents: 1000,
          status: "archived",
        })}
      />,
    );
    expect(screen.queryByText("餘額偏低")).toBeNull();
  });

  it("hides low balance warning when balance equals threshold", () => {
    render(
      <WalletCard
        wallet={makeWallet({
          balance_cents: 1000,
          warning_threshold_cents: 1000,
        })}
      />,
    );
    expect(screen.queryByText("餘額偏低")).toBeNull();
  });

  it("navigates to wallet detail on click", () => {
    render(<WalletCard wallet={makeWallet({ id: 42 })} />);
    fireEvent.click(screen.getByText("Main Wallet"));
    expect(mockNavigate).toHaveBeenCalledWith("/wallets/42");
  });
});
