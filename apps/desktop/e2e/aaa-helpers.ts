import type { Page } from '@playwright/test'
import { expect } from './fixtures'
export const screens = [
  'home',
  'items',
  'apps',
  'review',
  'history',
  'benchmark',
  'games',
  'settings',
] as const
export async function navigate(page: Page, screen: string) {
  const button = page.locator(`.sidebar [data-page="${screen}"]`)
  if (!(await button.isVisible())) await page.locator('.more-navigation summary').click()
  await button.click()
  await expect(page.locator('h1')).toBeFocused()
}
export async function analyze(page: Page) {
  await page.locator('.profile-panel .btn--primary').click()
  await expect(page.locator('.status-banner__meta')).toHaveText('86 / 86')
}
export async function prepare(page: Page) {
  await analyze(page)
  await page.locator('.profile-panel .btn--glass').click()
}
export async function settle(page: Page) {
  await page.evaluate(async () => {
    await document.fonts.ready
    scrollTo(0, 0)
    if (document.activeElement instanceof HTMLElement) document.activeElement.blur()
  })
  await page.waitForTimeout(550)
}
