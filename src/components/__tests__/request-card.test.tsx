import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { RequestCard } from "../request-card";
import type { Request as AppRequest } from "@/lib/types";

const mockNavigate = vi.fn();
vi.mock("react-router", () => ({
  useNavigate: () => mockNavigate,
}));

function makeRequest(overrides: Partial<AppRequest> = {}): AppRequest {
  return {
    id: 1,
    uuid: "req-uuid",
    family_id: 1,
    requester_member_id: 1,
    wallet_id: 1,
    amount_cents: 20000,
    category: "food",
    notes: null,
    attachment_url: null,
    status: "pending",
    decision_by_member_id: null,
    decision_at: null,
    rejection_reason: null,
    sync_version: 1,
    last_synced_at: null,
    is_deleted: 0,
    created_at: "2024-06-15T10:00:00.000Z",
    updated_at: "2024-06-15T10:00:00.000Z",
    ...overrides,
  };
}

describe("RequestCard", () => {
  beforeEach(() => {
    mockNavigate.mockClear();
  });

  it("renders food category label", () => {
    render(<RequestCard request={makeRequest({ category: "food" })} />);
    expect(screen.getByText("食物")).toBeDefined();
  });

  it("renders transport category label", () => {
    render(<RequestCard request={makeRequest({ category: "transport" })} />);
    expect(screen.getByText("交通")).toBeDefined();
  });

  it("renders education category label", () => {
    render(<RequestCard request={makeRequest({ category: "education" })} />);
    expect(screen.getByText("教育")).toBeDefined();
  });

  it("renders entertainment category label", () => {
    render(<RequestCard request={makeRequest({ category: "entertainment" })} />);
    expect(screen.getByText("娛樂")).toBeDefined();
  });

  it("renders clothing category label", () => {
    render(<RequestCard request={makeRequest({ category: "clothing" })} />);
    expect(screen.getByText("服飾")).toBeDefined();
  });

  it("renders health category label", () => {
    render(<RequestCard request={makeRequest({ category: "health" })} />);
    expect(screen.getByText("醫療")).toBeDefined();
  });

  it("renders other category label", () => {
    render(<RequestCard request={makeRequest({ category: "other" })} />);
    expect(screen.getByText("其他")).toBeDefined();
  });

  it("falls back to other for unknown category", () => {
    render(<RequestCard request={makeRequest({ category: "xyz" })} />);
    expect(screen.getByText("其他")).toBeDefined();
  });

  it("falls back to other when category is null", () => {
    render(<RequestCard request={makeRequest({ category: null })} />);
    expect(screen.getByText("其他")).toBeDefined();
  });

  it("renders amount", () => {
    const { container } = render(<RequestCard request={makeRequest({ amount_cents: 50000 })} />);
    expect(container.textContent).toContain("500");
  });

  it("renders notes when provided", () => {
    render(<RequestCard request={makeRequest({ notes: "School supplies" })} />);
    expect(screen.getByText("School supplies")).toBeDefined();
  });

  it("does not render notes when null", () => {
    render(<RequestCard request={makeRequest({ notes: null })} />);
    expect(screen.queryByText("School supplies")).toBeNull();
  });

  it("renders formatted date", () => {
    const { container } = render(<RequestCard request={makeRequest()} />);
    expect(container.textContent).toContain("2024");
  });

  it("navigates to request detail on click", () => {
    render(<RequestCard request={makeRequest({ id: 77 })} />);
    fireEvent.click(screen.getByText("食物"));
    expect(mockNavigate).toHaveBeenCalledWith("/requests/77");
  });
});
