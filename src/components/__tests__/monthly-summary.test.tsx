import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { MonthlySummary } from "../monthly-summary";
import type { MonthlySummary as MonthlySummaryData } from "@/hooks/use-transactions";

function makeSummary(overrides: Partial<MonthlySummaryData> = {}): MonthlySummaryData {
  return {
    total_credit_cents: 100000,
    total_debit_cents: 30000,
    net_change_cents: 70000,
    transaction_count: 5,
    ...overrides,
  };
}

describe("MonthlySummary", () => {
  it("renders month label", () => {
    render(<MonthlySummary summary={makeSummary()} year={2024} month={6} />);
    expect(screen.getByText("2024 年 6 月 統計")).toBeDefined();
  });

  it("renders credit amount", () => {
    const { container } = render(
      <MonthlySummary summary={makeSummary({ total_credit_cents: 50000 })} year={2024} month={1} />,
    );
    expect(container.textContent).toContain("入帳");
    expect(container.textContent).toContain("500");
  });

  it("renders debit amount", () => {
    const { container } = render(
      <MonthlySummary summary={makeSummary({ total_debit_cents: 20000 })} year={2024} month={1} />,
    );
    expect(container.textContent).toContain("出帳");
    expect(container.textContent).toContain("200");
  });

  it("renders net change", () => {
    const { container } = render(
      <MonthlySummary summary={makeSummary({ net_change_cents: 30000 })} year={2024} month={1} />,
    );
    expect(container.textContent).toContain("淨變動");
  });

  it("renders transaction count", () => {
    render(
      <MonthlySummary summary={makeSummary({ transaction_count: 12 })} year={2024} month={1} />,
    );
    expect(screen.getByText("共 12 筆交易")).toBeDefined();
  });

  it("renders zero transaction count", () => {
    render(
      <MonthlySummary summary={makeSummary({ transaction_count: 0 })} year={2024} month={1} />,
    );
    expect(screen.getByText("共 0 筆交易")).toBeDefined();
  });

  it("renders December correctly", () => {
    render(<MonthlySummary summary={makeSummary()} year={2024} month={12} />);
    expect(screen.getByText("2024 年 12 月 統計")).toBeDefined();
  });

  it("renders all zero amounts", () => {
    const { container } = render(
      <MonthlySummary
        summary={makeSummary({
          total_credit_cents: 0,
          total_debit_cents: 0,
          net_change_cents: 0,
          transaction_count: 0,
        })}
        year={2024}
        month={1}
      />,
    );
    expect(container.textContent).toContain("共 0 筆交易");
  });
});
