import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { RequestStatusBadge } from "../request-status-badge";

describe("RequestStatusBadge", () => {
  it("renders draft status with correct label", () => {
    render(<RequestStatusBadge status="draft" />);
    expect(screen.getByText("草稿")).toBeDefined();
  });

  it("renders pending status with correct label", () => {
    render(<RequestStatusBadge status="pending" />);
    expect(screen.getByText("待審核")).toBeDefined();
  });

  it("renders approved status with correct label", () => {
    render(<RequestStatusBadge status="approved" />);
    expect(screen.getByText("已核准")).toBeDefined();
  });

  it("renders rejected status with correct label", () => {
    render(<RequestStatusBadge status="rejected" />);
    expect(screen.getByText("已駁回")).toBeDefined();
  });

  it("renders cancelled status with correct label", () => {
    render(<RequestStatusBadge status="cancelled" />);
    expect(screen.getByText("已取消")).toBeDefined();
  });

  it("renders unknown status as-is with outline variant", () => {
    render(<RequestStatusBadge status="unknown_status" />);
    expect(screen.getByText("unknown_status")).toBeDefined();
    const badge = screen.getByText("unknown_status");
    expect(badge.getAttribute("data-variant")).toBe("outline");
  });

  it("applies correct badge variant for draft (outline)", () => {
    render(<RequestStatusBadge status="draft" />);
    const badge = screen.getByText("草稿");
    expect(badge.getAttribute("data-variant")).toBe("outline");
  });

  it("applies correct badge variant for pending (default)", () => {
    render(<RequestStatusBadge status="pending" />);
    const badge = screen.getByText("待審核");
    expect(badge.getAttribute("data-variant")).toBe("default");
  });

  it("applies correct badge variant for rejected (destructive)", () => {
    render(<RequestStatusBadge status="rejected" />);
    const badge = screen.getByText("已駁回");
    expect(badge.getAttribute("data-variant")).toBe("destructive");
  });
});
