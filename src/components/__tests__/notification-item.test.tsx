import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { NotificationItem } from "../notification-item";
import type { Notification } from "@/hooks/use-notifications";

function makeNotification(overrides: Partial<Notification> = {}): Notification {
  return {
    id: 1,
    event_type: "request_approved",
    payload: null,
    is_read: 0,
    read_at: null,
    created_at: "2024-06-15T10:30:00.000Z",
    ...overrides,
  };
}

describe("NotificationItem", () => {
  it("renders known event type label", () => {
    render(<NotificationItem notification={makeNotification()} />);
    expect(screen.getByText("請款已核准")).toBeDefined();
  });

  it("renders request_submitted label", () => {
    render(
      <NotificationItem
        notification={makeNotification({ event_type: "request_submitted" })}
      />,
    );
    expect(screen.getByText("新請款")).toBeDefined();
  });

  it("renders request_rejected label", () => {
    render(
      <NotificationItem
        notification={makeNotification({ event_type: "request_rejected" })}
      />,
    );
    expect(screen.getByText("請款已駁回")).toBeDefined();
  });

  it("renders allowance_disbursed label", () => {
    render(
      <NotificationItem
        notification={makeNotification({ event_type: "allowance_disbursed" })}
      />,
    );
    expect(screen.getByText("津貼已發放")).toBeDefined();
  });

  it("renders low_balance label", () => {
    render(
      <NotificationItem
        notification={makeNotification({ event_type: "low_balance" })}
      />,
    );
    expect(screen.getByText("餘額偏低")).toBeDefined();
  });

  it("renders member_joined label", () => {
    render(
      <NotificationItem
        notification={makeNotification({ event_type: "member_joined" })}
      />,
    );
    expect(screen.getByText("新成員加入")).toBeDefined();
  });

  it("renders unknown event_type as-is", () => {
    render(
      <NotificationItem
        notification={makeNotification({ event_type: "unknown_event" })}
      />,
    );
    expect(screen.getByText("unknown_event")).toBeDefined();
  });

  it("shows unread indicator for unread notifications", () => {
    const { container } = render(
      <NotificationItem notification={makeNotification({ is_read: 0 })} />,
    );
    const dot = container.querySelector(".rounded-full.bg-primary");
    expect(dot).not.toBeNull();
  });

  it("hides unread indicator for read notifications", () => {
    const { container } = render(
      <NotificationItem notification={makeNotification({ is_read: 1 })} />,
    );
    const dot = container.querySelector(".rounded-full.bg-primary");
    expect(dot).toBeNull();
  });

  it("applies font-semibold for unread notifications", () => {
    render(
      <NotificationItem notification={makeNotification({ is_read: 0 })} />,
    );
    const label = screen.getByText("請款已核准");
    expect(label.className).toContain("font-semibold");
  });

  it("does not apply font-semibold for read notifications", () => {
    render(
      <NotificationItem notification={makeNotification({ is_read: 1 })} />,
    );
    const label = screen.getByText("請款已核准");
    expect(label.className).not.toContain("font-semibold");
  });

  it("parses payload with amount_cents", () => {
    const payload = JSON.stringify({ amount_cents: 50000 });
    render(
      <NotificationItem
        notification={makeNotification({ payload })}
      />,
    );
    expect(screen.getByText(/NT\$500/)).toBeDefined();
  });

  it("parses payload with reason", () => {
    const payload = JSON.stringify({ reason: "Too expensive" });
    render(
      <NotificationItem
        notification={makeNotification({ payload })}
      />,
    );
    expect(screen.getByText("Too expensive")).toBeDefined();
  });

  it("parses payload with both amount and reason", () => {
    const payload = JSON.stringify({
      amount_cents: 10000,
      reason: "Birthday gift",
    });
    const { container } = render(
      <NotificationItem
        notification={makeNotification({ payload })}
      />,
    );
    expect(container.textContent).toContain("NT$100");
    expect(container.textContent).toContain("Birthday gift");
  });

  it("handles invalid JSON payload gracefully", () => {
    render(
      <NotificationItem
        notification={makeNotification({ payload: "not json{" })}
      />,
    );
    // Should not crash, just renders without description
    expect(screen.getByText("請款已核准")).toBeDefined();
  });

  it("calls onClick when clicked", () => {
    const handleClick = vi.fn();
    render(
      <NotificationItem
        notification={makeNotification()}
        onClick={handleClick}
      />,
    );
    fireEvent.click(screen.getByRole("button"));
    expect(handleClick).toHaveBeenCalledTimes(1);
  });

  it("renders created_at as formatted datetime", () => {
    const { container } = render(
      <NotificationItem notification={makeNotification()} />,
    );
    // Should contain a formatted date string
    expect(container.textContent).toContain("2024");
  });
});
