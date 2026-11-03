import { mkdirSync } from 'node:fs'
import { test, expect } from './fixtures'
import pt from '../src/locales/pt-BR.json' with { type: 'json' }
import en from '../src/locales/en.json' with { type: 'json' }
for (const language of ['pt-BR', 'en'] as const)
  for (const theme of ['light', 'dark'] as const)
    for (const width of [1440, 390]) {
      test(`observed defects 1–12 ${language} ${theme} ${width}`, async ({ page }) => {
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
        // 2 useful navigation targets, 6 simulation rather than search, 7 precision.
        await expect(page.getByRole('list', { name: t.statusStrip })).toBeVisible()
        await expect(page.locator('.status-strip .status-chip')).toHaveCount(3)
        await expect(
          page.getByRole('button', { name: t.simulate, exact: true }).locator('.lucide-scan-eye'),
        ).toBeVisible()
        await expect(
          page.getByRole('img', {
            name:
              language === 'en'
                ? /available of 16\.0 GiB: 6\.0 GiB/
                : /disponíveis de 16,0 GiB: 6,0 GiB/,
          }),
        ).toBeVisible()
        await expect(page.locator('.metric--cpu .sparkline')).toBeVisible()
        // 8 equal metric heights, 9 balanced short slogan, 11 real ghost control.
        const heights = await page
          .locator('.vitals .metric')
          .evaluateAll((nodes) => nodes.map((n) => n.getBoundingClientRect().height))
        expect(Math.max(...heights) - Math.min(...heights)).toBeLessThanOrEqual(1)
        expect(
          await page.locator('.brand small').evaluate((n) => getComputedStyle(n).textWrap),
        ).toBe('balance')
        const pause = page.getByRole('button', { name: t.pauseMetrics, exact: true })
        await expect(pause).toHaveClass(/btn--ghost/)
        await expect(pause.locator('.lucide-pause')).toBeVisible()
        await page.getByRole('button', { name: t.simulate, exact: true }).click()
        await expect(page.getByText(t.complete, { exact: true })).toBeVisible()
        await page.getByRole('button', { name: t.items, exact: true }).click()
        // 4 no repeated selection counter. 10 secondary text >=13.5px.
        await expect(page.locator('.page-heading .counter')).toHaveCount(0)
        expect(
          (
            await page
              .locator('.item-row small')
              .evaluateAll((nodes) => nodes.map((n) => parseFloat(getComputedStyle(n).fontSize)))
          ).every((size) => size >= 13.5),
        ).toBe(true)
        // 1 long banner text and count never touch, including wrapped layout.
        await page.locator('.status-banner__text').evaluate((n) => {
          n.textContent = (n.textContent + ' ').repeat(5)
        })
        const text = await page.locator('.status-banner__text').boundingBox(),
          meta = await page.locator('.status-banner__meta').boundingBox()
        expect(
          text &&
            meta &&
            (meta.x >= text.x + text.width + 12 || meta.y >= text.y + text.height + 10),
        ).toBe(true)
        // 3 fixed at the window edge (above mobile navigation), opaque action surface.
        const bar = page.locator('.action-bar')
        expect(await bar.evaluate((n) => getComputedStyle(n).position)).toBe('fixed')
        const box = await bar.boundingBox()
        expect(Math.abs(box!.y + box!.height - (1000 - (width <= 900 ? 88 : 0)))).toBeLessThan(1)
        expect(
          await page
            .locator('.action-bar__inner')
            .evaluate((n) => getComputedStyle(n).backgroundColor),
        ).not.toContain('rgba')
        await page.evaluate(() => scrollTo(0, document.body.scrollHeight))
        const last = await page.locator('.item-row').last().boundingBox(),
          barAfter = await bar.boundingBox()
        expect(last && barAfter && last.y + last.height <= barAfter.y).toBe(true)
        if (language === 'pt-BR') {
          mkdirSync('../../outputs/phase-6.7/screenshots', { recursive: true })
          await page.screenshot({
            path: `../../outputs/phase-6.7/screenshots/items-bottom-${theme}-${width}.png`,
            animations: 'disabled',
          })
        }
        // 12 toggles use switches; consent alone uses a checkbox.
        await page.getByRole('button', { name: t.games, exact: true }).click()
        await expect(page.locator('.monitor-card')).toContainText(t.monitorStopped)
        await expect(page.locator('.game-monitor-summary')).toContainText(t.gameSetupSteps)
        await page.getByRole('button', { name: t.addGame, exact: true }).click()
        await page.getByRole('button', { name: /Sample.exe/ }).click()
        await expect(page.locator('.game-action').getByRole('checkbox')).toHaveCount(0)
        await expect(page.locator('.game-action').getByRole('switch')).toHaveCount(5)
        await expect(page.getByRole('switch', { name: t.enableProfile, exact: true })).toBeVisible()
        await expect(page.getByRole('checkbox', { name: t.gameConsent, exact: true })).toBeVisible()
        expect(await page.evaluate(() => window.__ametistaTest?.executions.length)).toBe(0)
      })
    }
