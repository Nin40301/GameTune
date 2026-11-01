import { test, expect } from '@playwright/test'
import AxeBuilder from '@axe-core/playwright'
import { readFileSync } from 'node:fs'
const pt: Record<string, string> = JSON.parse(
  readFileSync(new URL('../src/locales/pt-BR.json', import.meta.url), 'utf8'),
)
const en: Record<string, string> = JSON.parse(
  readFileSync(new URL('../src/locales/en.json', import.meta.url), 'utf8'),
)
for (const locale of ['pt-BR', 'en'] as const)
  for (const theme of ['dark', 'light'] as const) {
    test(`game profiles and monitor axe ${locale} ${theme}`, async ({ page }) => {
      const t = locale === 'en' ? en : pt
      await page.addInitScript(
        ({ locale, theme }) => {
          localStorage.setItem('gametune.language', locale)
          localStorage.setItem('gametune.theme', theme)
        },
        { locale, theme },
      )
      await page.goto('/')
      await page.getByRole('button', { name: t.games, exact: true }).click()
      await page.getByRole('button', { name: t.addGame }).click()
      await page.getByRole('button', { name: /Sample.exe/ }).click()
      await page.getByRole('checkbox', { name: t.enableProfile }).check()
      await expect(page.getByRole('button', { name: t.saveGame })).toBeDisabled()
      await page.getByRole('checkbox', { name: t.gameConsent }).check()
      await page.getByRole('button', { name: t.saveGame }).click()
      await expect(page.getByText(t.profileEnabled)).toBeVisible()
      expect(
        (await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa', 'wcag21aa']).analyze())
          .violations,
      ).toEqual([])
      await page.evaluate(() => {
        window.scrollTo(0, 0)
        if (document.activeElement instanceof HTMLElement) document.activeElement.blur()
      })
      await page.screenshot({ path: `test-results/games-${locale}-${theme}.png`, fullPage: true })
      await page.getByRole('button', { name: t.settings, exact: true }).click()
      await page.getByRole('checkbox', { name: t.monitorConsent }).check()
      await page.getByRole('button', { name: t.startMonitor }).click()
      await expect(page.getByText(t.monitorRunning)).toBeVisible()
      expect(
        (await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa', 'wcag21aa']).analyze())
          .violations,
      ).toEqual([])
      await page.getByRole('button', { name: t.stopMonitor }).click()
      await expect(page.getByText(t.monitorStopped)).toBeVisible()
      expect(await page.evaluate(() => window.__gametuneTest?.executions.length)).toBe(0)
    })
  }
for (const width of [390, 768, 1440])
  test(`games layout ${width}`, async ({ page }) => {
    await page.setViewportSize({ width, height: 900 })
    await page.goto('/')
    await page.getByRole('button', { name: pt.games, exact: true }).click()
    expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true)
  })
