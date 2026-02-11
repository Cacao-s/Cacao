import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { TransactionItem } from "../transaction-item";
import type { Transaction } from "@/lib/types";

function makeTx(overrides: Partial<Transaction> = {}): Transaction {
  return {
    id: 1,
    uuid: "tx-uuid",
    family_id: 1,
    wallet_id: 1,
    type: "credit",
    amount_cents: 10000,
    source_type: "manual",
    source_id: null,
    category: null,
    occurred_at: "2024-06-15T10:00:00.000Z",
    notes: null,
    sync_version: 1,
    last_synced_at: null,
    is_deleted: 0,
    created_at: "2024-06-15T10:00:00.000Z",
    ...overrides,
  };
}

describe("TransactionItem", () => {
  it("renders manual source label", () => {
    render(<TransactionItem transaction={makeTx({ source_type: "manual" })} />);
    expect(screen.getByText("手動記帳")).toBeDefined();
  });

  it("renders allowance source label", () => {
    render(<TransactionItem transaction={makeTx({ source_type: "allowance" })} />);
    expect(screen.getByText("津貼發放")).toBeDefined();
  });

  it("renders request source label", () => {
    render(<TransactionItem transaction={makeTx({ source_type: "request" })} />);
    expect(screen.getByText("請款支出")).toBeDefined();
  });

  it("renders adjustment source label", () => {
    render(<TransactionItem transaction={makeTx({ source_type: "adjustment" })} />);
    expect(screen.getByText("調整")).toBeDefined();
  });

  it("falls back to manual for unknown source_type", () => {
    render(<TransactionItem transaction={makeTx({ source_type: "unknown_src" })} />);
    expect(screen.getByText("手動記帳")).toBeDefined();
  });

  it("displays formatted date", () => {
    const { container } = render(
      <TransactionItem transaction={makeTx({ occurred_at: "2024-06-15T10:00:00.000Z" })} />,
    );
    expect(container.textContent).toContain("2024");
  });

  it("displays amount for credit transaction", () => {
    const { container } = render(
      <TransactionItem transaction={makeTx({ type: "credit", amount_cents: 25000 })} />,
    );
    expect(container.textContent).toContain("250");
  });

  it("displays amount for debit transaction", () => {
    const { container } = render(
      <TransactionItem transaction={makeTx({ type: "debit", amount_cents: 3000 })} />,
    );
    expect(container.textContent).toContain("30");
  });
});

// ── R45: TransactionItem Extended ───────────────────────────

describe("TransactionItem credit vs debit styling", () => {
  it("shows green color class for credit direction icon", () => {
    const { container } = render(<TransactionItem transaction={makeTx({ type: "credit" })} />);
    const greenIcon = container.querySelector(".text-green-500");
    expect(greenIcon).not.toBeNull();
  });

  it("shows destructive color class for debit direction icon", () => {
    const { container } = render(<TransactionItem transaction={makeTx({ type: "debit" })} />);
    const redIcon = container.querySelector(".text-destructive");
    expect(redIcon).not.toBeNull();
  });

  it("shows + sign for credit amount", () => {
    const { container } = render(
      <TransactionItem transaction={makeTx({ type: "credit", amount_cents: 5000 })} />,
    );
    expect(container.textContent).toContain("+");
  });

  it("shows - sign for debit amount", () => {
    const { container } = render(
      <TransactionItem transaction={makeTx({ type: "debit", amount_cents: 5000 })} />,
    );
    expect(container.textContent).toContain("-");
  });

  it("renders zero amount transaction", () => {
    const { container } = render(
      <TransactionItem transaction={makeTx({ type: "credit", amount_cents: 0 })} />,
    );
    expect(container.textContent).toContain("0");
  });

  it("renders very large amount", () => {
    const { container } = render(
      <TransactionItem transaction={makeTx({ type: "credit", amount_cents: 99999999 })} />,
    );
    expect(container.textContent).toContain("999,999");
  });
});
