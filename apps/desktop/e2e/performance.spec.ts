import { test, expect } from '@playwright/test'
import { readFileSync, readdirSync } from 'node:fs'
import pt from '../src/locales/pt-BR.json' with { type: 'json' }

test('compiled production CSS actually disables blur in reduced effects and forced colors', async ({
  page,
}) => {
  const assets = new URL('../dist/assets/', import.meta.url)
  const file = readdirSync(assets).find(
    (name) => name.startsWith('index-') && name.endsWith('.css'),
  )
  expect(file, 'Run npm run build before the compiled CSS regression test').toBeDefined()
  const css = readFileSync(new URL(file!, assets), 'utf8')
  await page.setContent(
    '<main class="app-bg"><aside class="sidebar glass"></aside><header class="topbar glass"></header><section class="profile-panel glass"></section></main>',
  )
  await page.addStyleTag({ content: css })
  const blur = () =>
    page
      .locator('.glass')
      .evaluateAll((elements) =>
        elements.map((element) => getComputedStyle(element).backdropFilter),
      )
  for (const theme of ['light', 'dark']) {
    await page.locator('html').evaluate((html, theme) => {
      html.dataset.theme = theme
      html.dataset.effects = 'full'
    }, theme)
    expect((await blur()).every((value) => value.includes('blur('))).toBe(true)
    await page.locator('html').evaluate((html) => {
      html.dataset.effects = 'reduced'
    })
    expect(await blur()).toEqual(['none', 'none', 'none'])
  }
  await page.locator('html').evaluate((html) => {
    html.dataset.effects = 'full'
  })
  const session = await page.context().newCDPSession(page)
  await session.send('Emulation.setEmulatedMedia', {
    features: [{ name: 'prefers-reduced-transparency', value: 'reduce' }],
  })
  expect(await blur()).toEqual(['none', 'none', 'none'])
  await session.send('Emulation.setEmulatedMedia', { features: [] })
  await session.detach()
  await page.emulateMedia({ forcedColors: 'active' })
  expect(await blur()).toEqual(['none', 'none', 'none'])
})

test('idle glass keeps identity with three bounded blurred surfaces and no retained page animation', async ({
  page,
}) => {
  await page.addInitScript(() => localStorage.setItem('ametista.effects', 'full'))
  await page.goto('/')
  await expect(
    page.getByRole('img', { name: `${pt.availableRam}: 6 GiB`, exact: true }),
  ).toBeVisible()
  await page.waitForTimeout(450)
  const styles = await page.evaluate(() => ({
    font: getComputedStyle(document.documentElement).fontFamily,
    background: getComputedStyle(document.querySelector('.app-bg')!).backgroundImage,
    pseudo: getComputedStyle(document.querySelector('.app-bg')!, '::before').content,
    fill: getComputedStyle(document.querySelector('.page')!).animationFillMode,
    blur: [...document.querySelectorAll('.glass')].filter(
      (el) => getComputedStyle(el).backdropFilter !== 'none',
    ).length,
    orbTransition: getComputedStyle(document.querySelector('.orb circle')!).transitionDuration,
  }))
  expect(styles.font).toContain('Plus Jakarta Sans Variable')
  expect(styles.background).toBe('none')
  expect(styles.pseudo).not.toBe('none')
  expect(styles.fill).toBe('none')
  expect(styles.blur).toBe(3)
  expect(styles.orbTransition).toBe('0s')
  await expect(page.getByRole('button', { name: pt.simulate, exact: true })).toBeVisible()
  expect(await page.evaluate(() => window.__ametistaTest?.executions.length)).toBe(0)
})

test('focus and visibility suspend visual activity without applying any operation', async ({
  page,
}) => {
  await page.goto('/')
  await expect(page.locator('html')).toHaveAttribute('data-window-active', 'true')
  await page.evaluate(() => window.dispatchEvent(new Event('blur')))
  await expect(page.locator('html')).toHaveAttribute('data-window-active', 'false')
  await page.evaluate(() => window.dispatchEvent(new Event('focus')))
  await expect(page.locator('html')).toHaveAttribute('data-window-active', 'true')
  await page.evaluate(() => {
    Object.defineProperty(document, 'visibilityState', { configurable: true, get: () => 'hidden' })
    document.dispatchEvent(new Event('visibilitychange'))
  })
  await expect(page.locator('html')).toHaveAttribute('data-window-active', 'false')
  await page.evaluate(() => {
    Object.defineProperty(document, 'visibilityState', { configurable: true, get: () => 'visible' })
    document.dispatchEvent(new Event('visibilitychange'))
    window.dispatchEvent(new Event('focus'))
  })
  await expect(page.locator('html')).toHaveAttribute('data-window-active', 'true')
  expect(await page.evaluate(() => window.__ametistaTest?.executions.length)).toBe(0)
})
