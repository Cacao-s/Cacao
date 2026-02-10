import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { SetupGuard } from "../setup-guard";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

const mockNavigate = vi.fn();
let mockQueryResult: { data: boolean | undefined; isLoading: boolean } = {
  data: undefined,
  isLoading: true,
};

vi.mock("@tanstack/react-query", () => ({
  useQuery: () => mockQueryResult,
}));

vi.mock("react-router", () => ({
  Navigate: ({ to }: { to: string }) => {
    mockNavigate(to);
    return <div data-testid="navigate">{to}</div>;
  },
  Outlet: () => <div data-testid="outlet">Protected Content</div>,
}));

describe("SetupGuard", () => {
  it("shows loading spinner while loading", () => {
    mockQueryResult = { data: undefined, isLoading: true };
    const { container } = render(<SetupGuard />);
    expect(container.querySelector(".animate-spin")).not.toBeNull();
  });

  it("redirects to /setup when not setup", () => {
    mockQueryResult = { data: false, isLoading: false };
    render(<SetupGuard />);
    expect(screen.getByTestId("navigate")).toBeDefined();
    expect(screen.getByTestId("navigate").textContent).toBe("/setup");
  });

  it("renders outlet when setup is complete", () => {
    mockQueryResult = { data: true, isLoading: false };
    render(<SetupGuard />);
    expect(screen.getByTestId("outlet")).toBeDefined();
    expect(screen.getByText("Protected Content")).toBeDefined();
  });

  it("does not show spinner when not loading", () => {
    mockQueryResult = { data: true, isLoading: false };
    const { container } = render(<SetupGuard />);
    expect(container.querySelector(".animate-spin")).toBeNull();
  });

  it("redirects when data is undefined and not loading", () => {
    mockQueryResult = { data: undefined, isLoading: false };
    render(<SetupGuard />);
    expect(screen.getByTestId("navigate")).toBeDefined();
  });
});
