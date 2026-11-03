import { mkdirSync } from 'node:fs'
// New-user flows deliberately use the base fixture, with no onboarding bypass.
import { test, expect } from '@playwright/test'
import AxeBuilder from '@axe-core/playwright'
import pt from '../src/locales/pt-BR.json' with { type: 'json' }
import en from '../src/locales/en.json' with { type: 'json' }
for (const language of ['pt-BR', 'en'] as const)
  for (const theme of ['light', 'dark'] as const)
    for (const width of [1440, 390]) {
      test(`first run, focus and axe ${language} ${theme} ${width}`, async ({ page }) => {
        const t = language === 'en' ? en : pt
        await page.setViewportSize({ width, height: 1000 })
        await page.addInitScript(
          ({ language, theme }) => {
            Object.defineProperty(navigator, 'language', { get: () => language })
            localStorage.setItem('ametista.theme', theme)
          },
          { language, theme },
        )
        await page.goto('/')
        for (let step = 0; step < 3; step++) {
          await expect(page.getByRole('heading', { level: 1 })).toHaveText(
            [t.onboardingTitle0, t.onboardingTitle1, t.onboardingTitle2][step],
          )
          await expect(page.getByRole('heading', { level: 1 })).toBeFocused()
          expect(
            await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth),
          ).toBe(true)
          expect(
            (await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa', 'wcag21aa']).analyze())
              .violations,
          ).toEqual([])
          if (language === 'pt-BR') {
            mkdirSync('../../outputs/release-0.6.5/screenshots', { recursive: true })
            await page.screenshot({
              path: `../../outputs/release-0.6.5/screenshots/onboarding-${step}-${theme}-${width}-full.png`,
              fullPage: true,
              animations: 'disabled',
            })
          }
          await page
            .getByRole('button', { name: step < 2 ? t.continue : t.beginSimulation, exact: true })
            .click()
        }
        await expect(page.getByRole('button', { name: t.simulate, exact: true })).toBeVisible()
        expect(await page.evaluate(() => window.__ametistaTest?.executions.length)).toBe(0)
      })
    }
