import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, it, expect, vi } from 'vitest';
import { AppShell } from './AppShell';

// Tauri APIs are not available in jsdom — stub the module boundary
vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));

describe('AppShell', () => {
  it('renders all three navigation tabs', () => {
    render(<AppShell />);
    expect(screen.getByRole('tab', { name: 'Cue Pad' })).toBeInTheDocument();
    expect(screen.getByRole('tab', { name: 'Monitor' })).toBeInTheDocument();
    expect(screen.getByRole('tab', { name: 'Settings' })).toBeInTheDocument();
  });

  it('shows Cue Pad panel by default', () => {
    render(<AppShell />);
    expect(screen.getByText(/Cue Pad — coming in Epic 3/i)).toBeVisible();
  });

  it('switches to Monitor panel when Monitor tab is clicked', async () => {
    const user = userEvent.setup();
    render(<AppShell />);
    await user.click(screen.getByRole('tab', { name: 'Monitor' }));
    expect(screen.getByText(/Monitor — coming in Epic 2/i)).toBeVisible();
  });

  it('switches to Settings panel when Settings tab is clicked', async () => {
    const user = userEvent.setup();
    render(<AppShell />);
    await user.click(screen.getByRole('tab', { name: 'Settings' }));
    expect(screen.getByText(/Settings — coming in Epic 2\+/i)).toBeVisible();
  });

  it('TabsList has an accessible aria-label', () => {
    render(<AppShell />);
    expect(screen.getByRole('tablist', { name: 'Main navigation' })).toBeInTheDocument();
  });
});
