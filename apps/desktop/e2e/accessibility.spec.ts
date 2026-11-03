import { test, expect } from './fixtures'
import AxeBuilder from '@axe-core/playwright'
import pt from '../src/locales/pt-BR.json' with { type: 'json' }
test('reflow geometry at 200 and 400 percent, focus and touch targets', async ({ page }) => {
  test.setTimeout(120000)
  await page.goto('/?benchmark-complete=1')
  await page.getByRole('button', { name: pt.simulate, exact: true }).click()
  await expect(page.getByText(pt.complete, { exact: true })).toBeVisible()
  await page.getByRole('button', { name: pt.reviewSelection, exact: true }).click()
  for (const width of [640, 320]) {
    await page.setViewportSize({ width, height: 900 })
    for (const name of [
      'home',
      'items',
      'apps',
      'review',
      'history',
      'benchmark',
      'games',
      'settings',
    ] as const) {
      const target = page.getByRole('button', { name: pt[name], exact: true })
      if (!(await target.isVisible())) await page.locator('.more-navigation summary').click()
      await target.click()
      await expect(page.getByRole('heading', { level: 1 })).toBeFocused()
      expect(
        await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth),
        `${name} ${width}`,
      ).toBe(true)
      const small = await page
        .locator('button:visible,summary:visible,input[type=checkbox]:visible')
        .evaluateAll((elements) =>
          elements
            .filter((element) => {
              const target =
                element instanceof HTMLInputElement
                  ? (element.closest('label') ?? element)
                  : element
              const box = target.getBoundingClientRect()
              return box.width < 39.5 || box.height < 39.5
            })
            .map((element) => ({
              tag: element.tagName,
              name: element.getAttribute('aria-label') ?? element.textContent,
            })),
        )
      expect(small, `${name} ${width}`).toEqual([])
    }
  }
})
test('diagnostic preview, inline errors and result screen use accessible controls', async ({
  page,
}) => {
  await page.goto('/')
  await page.getByRole('button', { name: pt.settings, exact: true }).click()
  await page.getByRole('button', { name: pt.previewDiagnostic, exact: true }).click()
  await expect(page.getByRole('button', { name: pt.saveDiagnostic, exact: true })).toBeDisabled()
  await page.getByLabel(pt.diagnosticConsent).check()
  expect(
    (await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa', 'wcag21aa']).analyze())
      .violations,
  ).toEqual([])
  await page.getByRole('button', { name: pt.home, exact: true }).click()
  await page.getByRole('button', { name: pt.simulate, exact: true }).click()
  await expect(page.getByText(pt.complete, { exact: true })).toBeVisible()
  await page.getByRole('button', { name: pt.reviewSelection, exact: true }).click()
  await page.getByLabel(pt.ackAll).check()
  await page.getByRole('button', { name: pt.apply, exact: true }).click()
  await expect(page.getByRole('heading', { level: 1 })).toHaveText(pt.results)
  await expect(page.getByRole('heading', { level: 1 })).toBeFocused()
  expect(
    (await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa', 'wcag21aa']).analyze())
      .violations,
  ).toEqual([])
})
