import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { ProjectForm } from "../ProjectForm";
import type { UserEvent } from "@testing-library/user-event/dist/types/setup/setup";
import type { ProjectCreate } from "@/types/project";

// Mock next-i18next
jest.mock("next-i18next", () => ({
  useTranslation: () => ({
    t: (key: string) => key,
  }),
}));

describe("ProjectForm", () => {
  let user: UserEvent;
  const mockOnSubmit = jest.fn();

  beforeEach(() => {
    user = userEvent.setup();
    mockOnSubmit.mockClear();
  });

  it("renders all form fields", () => {
    render(<ProjectForm onSubmit={mockOnSubmit} />);

    expect(screen.getByLabelText(/project.name/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/project.description/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/project.startDate/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/project.endDate/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/project.budget/i)).toBeInTheDocument();
  });

  it("validates required fields", async () => {
    render(<ProjectForm onSubmit={mockOnSubmit} />);

    await user.click(screen.getByRole("button", { name: /common.submit/i }));

    expect(await screen.findAllByText(/validation.required/i)).toHaveLength(4);
    expect(mockOnSubmit).not.toHaveBeenCalled();
  });

  it("submits form with valid data", async () => {
    render(<ProjectForm onSubmit={mockOnSubmit} />);

    await user.type(screen.getByLabelText(/project.name/i), "Test Project");
    await user.type(screen.getByLabelText(/project.startDate/i), "2024-01-01");
    await user.type(screen.getByLabelText(/project.endDate/i), "2024-12-31");
    await user.type(screen.getByLabelText(/project.budget/i), "10000");

    await user.click(screen.getByRole("button", { name: /common.submit/i }));

    const expectedData: ProjectCreate = {
      name: "Test Project",
      start_date: "2024-01-01",
      end_date: "2024-12-31",
      budget: 10000,
    };

    expect(mockOnSubmit).toHaveBeenCalledWith(expectedData);
  });
});
