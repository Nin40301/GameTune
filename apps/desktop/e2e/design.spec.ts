import { test, expect } from './fixtures'
import AxeBuilder from '@axe-core/playwright'
import { mkdirSync, readFileSync } from 'node:fs'
import pt from '../src/locales/pt-BR.json' with { type: 'json' }
import en from '../src/locales/en.json' with { type: 'json' }

const shots = '../../outputs/phase-6.7/screenshots'
for (const language of ['pt-BR', 'en'] as const)
  for (const theme of ['light', 'dark'] as const)
    for (const width of [1440, 1100, 768, 390]) {
      test(`liquid glass AA and layout ${language} ${theme} ${width}`, async ({ page }) => {
        test.setTimeout(120000)
        const t = language === 'en' ? en : pt
        const network: string[] = [],
          errors: string[] = []
        page.on('request', (request) => {
          if (
            !request.url().startsWith('http://127.0.0.1:1420/') &&
            !request.url().startsWith('data:')
          )
            network.push(request.url())
        })
        page.on('pageerror', (error) => errors.push(error.message))
        await page.addInitScript(
          ({ language, theme }) => {
            localStorage.setItem('ametista.language', language)
            localStorage.setItem('ametista.theme', theme)
            localStorage.setItem('ametista.effects', 'full')
          },
          { language, theme },
        )
        await page.setViewportSize({ width, height: 1000 })
        await page.goto('/?benchmark-complete=1')
        await expect(page.getByRole('radio', { name: t.safe, exact: true })).toBeChecked()
        await expect(
          page.getByRole('img', { name: language === 'en' ? /6\.0 GiB/ : /6,0 GiB/ }),
        ).toBeVisible()
        mkdirSync(shots, { recursive: true })
        async function inspect(name: string) {
          await page.evaluate(() => {
            window.scrollTo(0, 0)
            if (document.activeElement instanceof HTMLElement) document.activeElement.blur()
          })
          await page.evaluate(() => document.fonts.ready)
          expect(
            await page.locator('h1').evaluate((el) => getComputedStyle(el).fontOpticalSizing),
          ).toBe('auto')
          expect(
            Math.abs(
              parseFloat(
                await page.locator('h1').evaluate((el) => getComputedStyle(el).letterSpacing),
              ),
            ),
          ).toBeLessThanOrEqual(0.5)
          // Let only the finite page entrance finish; no background animation.
          await page.waitForTimeout(400)
          if (width <= 900) await expect(page.locator('.topbar .mobile-brand')).toBeVisible()
          expect(
            await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth),
            name,
          ).toBe(true)
          const axe = await new AxeBuilder({ page })
            .withTags(['wcag2a', 'wcag2aa', 'wcag21aa'])
            .analyze()
          await test.info().attach(`${name}-axe`, {
            body: JSON.stringify({ violations: axe.violations, incomplete: axe.incomplete }),
            contentType: 'application/json',
          })
          // Capture evidence even if the audit fails, so the first correction round is reviewable.
          if (language === 'pt-BR')
            await page.screenshot({
              path: `${shots}/${name}-${theme}-${width}.png`,
              animations: 'disabled',
            })
          if (language === 'pt-BR' && [1440, 390].includes(width))
            await page.screenshot({
              path: `${shots}/${name}-${theme}-${width}-full.png`,
              fullPage: true,
              animations: 'disabled',
            })
          expect.soft(axe.violations, name).toEqual([])
        }
        await expect(page.locator('.metric--cpu .sparkline')).toBeVisible()
        await inspect('home')
        await page.getByRole('button', { name: t.simulate, exact: true }).click()
        await expect(page.getByText(t.complete, { exact: true })).toBeVisible()
        await page.getByRole('button', { name: t.reviewSelection, exact: true }).click()
        for (const name of [
          'items',
          'apps',
          'review',
          'history',
          'benchmark',
          'games',
          'settings',
        ] as const) {
          const target = page.getByRole('button', { name: t[name], exact: true })
          if (!(await target.isVisible())) await page.locator('.more-navigation summary').click()
          await target.click()
          if (name === 'games') {
            await page.getByRole('button', { name: t.addGame }).click()
            await page.getByRole('button', { name: /Sample.exe/ }).click()
            await page.getByRole('button', { name: t.discoverGames }).click()
          }
          await inspect(name)
        }
        expect(network).toEqual([])
        expect(errors).toEqual([])
        expect(await page.evaluate(() => window.__ametistaTest?.executions.length)).toBe(0)
      })
    }

test('keyboard, reduced effects, reduced motion and forced colors retain controls', async ({
  page,
}) => {
  await page.goto('/')
  const safe = page.getByRole('radio', { name: pt.safe, exact: true })
  await safe.focus()
  await page.keyboard.press('ArrowRight')
  await expect(page.getByRole('radio', { name: pt.balanced, exact: true })).toBeFocused()
  await page.keyboard.press('End')
  await expect(page.getByRole('radio', { name: pt.custom, exact: true })).toBeChecked()
  await page.keyboard.press('Home')
  await expect(safe).toBeChecked()
  await page.getByRole('button', { name: pt.settings, exact: true }).click()
  await page.getByRole('radio', { name: pt.effects_reduced, exact: true }).click()
  await expect(page.locator('html')).toHaveAttribute('data-effects', 'reduced')
  expect(await page.locator('.sidebar').evaluate((el) => getComputedStyle(el).backdropFilter)).toBe(
    'none',
  )
  await page.reload()
  await expect(page.locator('html')).toHaveAttribute('data-effects', 'reduced')
  await page.emulateMedia({ reducedMotion: 'reduce', forcedColors: 'active' })
  await expect(safe).toBeVisible()
  await safe.focus()
  expect(await safe.evaluate((el) => getComputedStyle(el).outlineWidth)).toBe('3px')
  expect(
    (await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa', 'wcag21aa']).analyze())
      .violations,
  ).toEqual([])
})

test('production CSP and animation rules remain restricted', async () => {
  const config = JSON.parse(
    readFileSync(new URL('../src-tauri/tauri.conf.json', import.meta.url), 'utf8'),
  ) as { app: { security: { csp: string } } }
  expect(config.app.security.csp).toContain("font-src 'self'")
  expect(config.app.security.csp).not.toMatch(/unsafe-inline|unsafe-eval|https:|\*/)
  const styles = readFileSync(new URL('../src/styles/screens.css', import.meta.url), 'utf8')
  expect(styles).not.toMatch(/background-attachment|feDisplacementMap|infinite/)
})

test('mobile navigation exposes four destinations and a keyboard-accessible More panel', async ({
  page,
}) => {
  await page.setViewportSize({ width: 390, height: 900 })
  await page.goto('/')
  await expect(page.getByRole('navigation').getByRole('button')).toHaveCount(4)
  const more = page.locator('.more-navigation summary')
  await more.focus()
  await page.keyboard.press('Enter')
  await expect(page.getByRole('button', { name: pt.history, exact: true })).toBeVisible()
  await page.keyboard.press('Escape')
  await expect(more).toBeFocused()
  await expect(page.getByRole('button', { name: pt.history, exact: true })).not.toBeVisible()
  await more.click()
  await page.getByRole('button', { name: pt.settings, exact: true }).click()
  await expect(page.getByRole('heading', { level: 1 })).toHaveText(pt.settingsTitle)
  await expect(page.locator('details[open]')).toHaveCount(0)
  expect(
    (await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa', 'wcag21aa']).analyze())
      .violations,
  ).toEqual([])
})

test('review removes duplicate status and repeated low-risk badges; item footer clears the last row', async ({
  page,
}) => {
  await page.goto('/')
  await page.getByRole('button', { name: pt.simulate, exact: true }).click()
  await expect(page.getByText(pt.complete, { exact: true })).toBeVisible()
  await page.getByRole('button', { name: pt.reviewSelection, exact: true }).click()
  await expect(page.getByText(pt.complete, { exact: true })).toHaveCount(0)
  await expect(page.getByRole('heading', { name: pt.byType })).toHaveCount(0)
  await expect(page.locator('.review-item .risk-badge')).toHaveCount(0)
  await expect(page.locator('.review-risk-group.low > h2')).toHaveCount(1)
  await page.getByRole('button', { name: pt.items, exact: true }).click()
  await expect(
    page.locator('.page-heading').getByRole('button', { name: pt.analyze }),
  ).toBeVisible()
  await expect(page.getByRole('searchbox')).toHaveAttribute('placeholder', pt.searchPlaceholder)
  for (const width of [1440, 1100, 768, 390]) {
    await page.setViewportSize({ width, height: 1000 })
    const last = page.locator('.item-list .item-row').last()
    await last.scrollIntoViewIfNeeded()
    await page.evaluate(() => window.scrollTo(0, document.body.scrollHeight))
    const row = await last.boundingBox(),
      bar = await page.locator('.sticky-actions').boundingBox()
    expect(row && bar && row.y + row.height <= bar.y).toBe(true)
  }
})
