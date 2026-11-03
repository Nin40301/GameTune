import { test as base, expect } from '@playwright/test'
// Existing-user flows: first-run onboarding is covered separately, without this fixture.
export const test = base.extend<{ existingUser: void }>({
  existingUser: [
    async ({ page }, use) => {
      await page.addInitScript(() => localStorage.setItem('ametista.onboarding', 'done'))
      await use()
    },
    { auto: true },
  ],
})
export { expect }
