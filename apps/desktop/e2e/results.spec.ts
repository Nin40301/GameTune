import { test, expect } from './fixtures'
import AxeBuilder from '@axe-core/playwright'
import { mkdirSync } from 'node:fs'
import pt from '../src/locales/pt-BR.json' with { type: 'json' }
import en from '../src/locales/en.json' with { type: 'json' }
for (const language of ['pt-BR', 'en'] as const)
  for (const theme of ['light', 'dark'] as const)
    for (const width of [1440, 390]) {
      test(`result evidence ${language} ${theme} ${width}`, async ({ page }) => {
        const t = language === 'en' ? en : pt
        await page.addInitScript(
          ({ language, theme }) => {
            localStorage.setItem('ametista.language', language)
            localStorage.setItem('ametista.theme', theme)
          },
          { language, theme },
        )
        await page.setViewportSize({ width, height: 1000 })
        await page.goto('/')
        await page.getByRole('button', { name: t.simulate, exact: true }).click()
        await expect(page.getByText(t.complete, { exact: true })).toBeVisible()
        await page.getByRole('button', { name: t.reviewSelection, exact: true }).click()
        await page.getByLabel(t.ackAll).check()
        await page.getByRole('button', { name: t.apply, exact: true }).click()
        await expect(page.getByRole('heading', { level: 1 })).toHaveText(t.results)
        expect(
          (await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa', 'wcag21aa']).analyze())
            .violations,
        ).toEqual([])
        if (language === 'pt-BR') {
          mkdirSync('../../outputs/release-0.6.5/screenshots', { recursive: true })
          await page.screenshot({
            path: `../../outputs/release-0.6.5/screenshots/results-${theme}-${width}-full.png`,
            fullPage: true,
            animations: 'disabled',
          })
        }
        expect(await page.evaluate(() => window.__ametistaTest?.executions.length)).toBe(1)
      })
    }
