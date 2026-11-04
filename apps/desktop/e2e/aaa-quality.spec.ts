import { test, expect } from './fixtures'
import { navigate, prepare, screens } from './aaa-helpers'
import pt from '../src/locales/pt-BR.json' with { type: 'json' }
import AxeBuilder from '@axe-core/playwright'

for (const language of ['pt-BR', 'en'])
  for (const theme of ['light', 'dark'])
    for (const width of [1440, 390]) {
      test(`AAA pseudo +40% ${language} ${theme} ${width}`, async ({ page }) => {
        test.setTimeout(120000)
        await page.addInitScript(
          ({ language, theme }) => {
            localStorage.setItem('ametista.language', language)
            localStorage.setItem('ametista.theme', theme)
            localStorage.setItem('__ametistaPseudo', 'true')
          },
          { language, theme },
        )
        await page.setViewportSize({ width, height: 1000 })
        await page.goto('/?benchmark-complete=1')
        await prepare(page)
        for (const screen of screens) {
          await navigate(page, screen)
          if (screen === 'games') {
            await page.locator('.games-panel > .action-row .primary').click()
            await page.getByRole('button', { name: /Sample.exe/ }).click()
            await page.locator('.games-panel > .action-row .secondary').first().click()
          }
          // Only rendered elements are audited; screen-reader-only text is intentionally clipped.
          const overflow = await page.evaluate(() =>
            [
              ...document.querySelectorAll<HTMLElement>(
                'button,summary,label,p,h1,h2,h3,small,select,.badge,.pill,.nav-label',
              ),
            ]
              .filter((element) => {
                const style = getComputedStyle(element),
                  box = element.getBoundingClientRect()
                return (
                  box.width > 0 &&
                  box.height > 0 &&
                  style.visibility !== 'hidden' &&
                  !element.closest('.sr-only') &&
                  !['inline', 'contents'].includes(style.display) &&
                  element.scrollWidth > element.clientWidth + 2
                )
              })
              .map((element) => ({
                tag: element.tagName,
                text: element.textContent?.slice(0, 110),
                width: element.clientWidth,
                scroll: element.scrollWidth,
              })),
          )
          expect.soft(overflow, screen).toEqual([])
          expect
            .soft(
              await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth),
              screen,
            )
            .toBe(true)
        }
      })
    }
for (const width of [640, 320])
  test(`AAA reflow equivalent to ${(1280 / width) * 100}%`, async ({ page }) => {
    test.setTimeout(120000)
    await page.setViewportSize({ width, height: 900 })
    await page.goto('/?benchmark-complete=1')
    await prepare(page)
    for (const screen of screens) {
      await navigate(page, screen)
      expect(
        await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth),
        screen,
      ).toBe(true)
    }
  })

test('native-size Home keeps the orb caption and all metric content inside their cards', async ({
  page,
}) => {
  await page.setViewportSize({ width: 1100, height: 740 })
  await page.goto('/')
  await expect(page.locator('.metric--cpu .sparkline')).toBeVisible()
  const orb = await page.locator('.orb').boundingBox(),
    caption = await page.locator('.orb__center span').boundingBox()
  expect(
    orb && caption && caption.x >= orb.x && caption.x + caption.width <= orb.x + orb.width,
  ).toBe(true)
  expect(
    await page
      .locator('.vitals .metric')
      .evaluateAll((nodes) => nodes.every((node) => node.scrollHeight <= node.clientHeight + 1)),
  ).toBe(true)
})

test('entire simulate review apply MOCK result undo flow works with the keyboard alone', async ({
  page,
}) => {
  await page.goto('/')
  async function tabTo(selector: string) {
    for (let index = 0; index < 100; index++) {
      if (
        await page
          .locator(selector)
          .evaluateAll((elements) => elements.some((element) => element === document.activeElement))
      )
        return
      await page.keyboard.press('Tab')
    }
    throw Error(`Keyboard could not reach ${selector}`)
  }
  await tabTo('.profile-panel .btn--primary')
  await page.keyboard.press('Enter')
  await expect(page.getByText(pt.complete, { exact: true })).toBeVisible()
  await tabTo('.profile-panel .btn--glass')
  await page.keyboard.press('Enter')
  await expect(page.locator('.page--review h1')).toBeFocused()
  await tabTo('.final-consent input[type="checkbox"]')
  await page.keyboard.press('Space')
  await tabTo('.action-bar .primary')
  await page.keyboard.press('Enter')
  await expect(page.locator('.page--results h1')).toBeFocused()
  await expect(page.getByText(pt.operationDone, { exact: true })).toBeVisible()
  await tabTo('.page--results .btn--primary')
  await page.keyboard.press('Enter')
  await expect(page.locator('.page--review h1')).toBeFocused()
  await tabTo('.final-consent input[type="checkbox"]')
  await page.keyboard.press('Space')
  await tabTo('.action-bar .primary')
  await page.keyboard.press('Enter')
  await expect(page.locator('.page--results h1')).toBeFocused()
  expect(await page.evaluate(() => window.__ametistaTest?.executions.length)).toBe(2)
})

test('mobile More menu remains above the review and apps action bars', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 900 })
  await page.goto('/')
  await prepare(page)
  await navigate(page, 'history')
  await expect(page.locator('h1')).toHaveText(pt.history)
  await navigate(page, 'apps')
  await expect(page.locator('.action-bar')).toBeVisible()
  await navigate(page, 'settings')
  await expect(page.locator('h1')).toHaveText(pt.settingsTitle)
})

for (const language of ['pt-BR', 'en'])
  for (const theme of ['light', 'dark'])
    test(`design system state axe ${language} ${theme}`, async ({ page }) => {
      await page.addInitScript(
        ({ language, theme }) => {
          localStorage.setItem('ametista.language', language)
          localStorage.setItem('ametista.theme', theme)
        },
        { language, theme },
      )
      await page.goto('/design-system')
      await expect(page.locator('.design-states')).toBeVisible()
      expect(
        (await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa', 'wcag21aa']).analyze())
          .violations,
      ).toEqual([])
      const first = page.locator('.design-states .btn').first()
      await first.hover()
      await first.focus()
      expect(await first.evaluate((element) => getComputedStyle(element).outlineWidth)).toBe('3px')
      // Contrast of the primary ink against the documented base, not a claim about every glass pixel.
      const contrast = await page.evaluate(() => {
        const style = getComputedStyle(document.documentElement)
        const luminance = (hex: string) => {
          const c = hex.trim().replace('#', '')
          const v = [0, 2, 4]
            .map((i) => parseInt(c.slice(i, i + 2), 16) / 255)
            .map((s) => (s <= 0.04045 ? s / 12.92 : ((s + 0.055) / 1.055) ** 2.4))
          return v[0] * 0.2126 + v[1] * 0.7152 + v[2] * 0.0722
        }
        const ink = luminance(style.getPropertyValue('--ink')),
          bg = luminance(style.getPropertyValue('--bg-base'))
        return (Math.max(ink, bg) + 0.05) / (Math.min(ink, bg) + 0.05)
      })
      expect(contrast).toBeGreaterThanOrEqual(7)
    })
