import { test, expect } from './fixtures'
import { screens, navigate, prepare, settle } from './aaa-helpers'
import { mkdirSync } from 'node:fs'
import pt from '../src/locales/pt-BR.json' with { type: 'json' }
const directory = '../../outputs/phase-6.7/delivery-2/screenshots'
for (const theme of ['light', 'dark'])
  for (const width of [1440, 390]) {
    test(`AAA Linux visual baseline ${theme} ${width}`, async ({ page }) => {
      test.setTimeout(180000)
      await page.addInitScript((theme) => {
        localStorage.setItem('ametista.theme', theme)
        localStorage.setItem('ametista.language', 'pt-BR')
        localStorage.setItem('ametista.effects', 'full')
      }, theme)
      await page.setViewportSize({ width, height: 1000 })
      await page.goto('/?benchmark-complete=1')
      await prepare(page)
      mkdirSync(directory, { recursive: true })
      for (const screen of screens) {
        await navigate(page, screen)
        if (screen === 'games') {
          await page.getByRole('button', { name: pt.addGame, exact: true }).click()
          await page.getByRole('button', { name: /Sample.exe/ }).click()
          await page.getByRole('button', { name: pt.discoverGames, exact: true }).click()
        }
        await settle(page)
        expect(
          await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth),
          screen,
        ).toBe(true)
        expect(
          await page
            .locator('.glass')
            .evaluateAll(
              (nodes) =>
                nodes.filter((node) => getComputedStyle(node).backdropFilter !== 'none').length,
            ),
          screen,
        ).toBeLessThanOrEqual(3)
        expect(
          await page.locator('.btn--primary:visible,.button.primary:visible').count(),
          screen,
        ).toBeLessThanOrEqual(1)
        const name = `${screen}-${theme}-${width}.png`,
          options = {
            animations: 'disabled' as const,
            fullPage: true,
            mask: [page.locator('.sample-stamp')],
          }
        await page.screenshot({ ...options, mask: [], path: `${directory}/${name}` })
        // Linux is canonical. Windows captures are review evidence, never baseline updates.
        if (process.platform === 'linux')
          await expect(page).toHaveScreenshot(name, {
            ...options,
            maxDiffPixelRatio: 0.002,
            threshold: 0.12,
          })
      }
      await page.goto('/design-system')
      await expect(page.getByRole('heading', { name: pt.designSystem })).toBeVisible()
      await settle(page)
      const name = `design-system-${theme}-${width}.png`
      await page.screenshot({
        path: `${directory}/${name}`,
        fullPage: true,
        animations: 'disabled',
      })
      if (process.platform === 'linux')
        await expect(page).toHaveScreenshot(name, {
          fullPage: true,
          animations: 'disabled',
          maxDiffPixelRatio: 0.002,
          threshold: 0.12,
        })
    })
  }
