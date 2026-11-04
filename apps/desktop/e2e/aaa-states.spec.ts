import { test, expect } from './fixtures'
import { navigate, prepare, settle, analyze } from './aaa-helpers'
import { mkdirSync } from 'node:fs'
import pt from '../src/locales/pt-BR.json' with { type: 'json' }
import AxeBuilder from '@axe-core/playwright'
const directory = '../../outputs/phase-6.7/delivery-2/states'
for (const theme of ['light', 'dark'])
  for (const width of [1440, 390])
    test(`AAA empty loading error success ${theme} ${width}`, async ({ page }) => {
      test.setTimeout(120000)
      await page.addInitScript((theme) => localStorage.setItem('ametista.theme', theme), theme)
      await page.setViewportSize({ width, height: 1000 })
      mkdirSync(directory, { recursive: true })
      async function capture(name: string) {
        await settle(page)
        expect(
          (await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa', 'wcag21aa']).analyze())
            .violations,
          name,
        ).toEqual([])
        await page.screenshot({
          path: `${directory}/${name}-${theme}-${width}.png`,
          fullPage: true,
          animations: 'disabled',
        })
      }
      await page.goto('/?ui-state=empty')
      await capture('home-empty')
      await analyze(page)
      for (const screen of ['apps', 'history', 'games', 'benchmark']) {
        await navigate(page, screen)
        await capture(`${screen}-empty`)
      }
      await navigate(page, 'items')
      await page.getByRole('searchbox').fill('nonexistent-fixture')
      await expect(page.getByText(pt.selectionEmpty)).toBeVisible()
      await capture('items-empty')
      await page.goto('/?ui-state=loading')
      await page.locator('.profile-panel .btn--primary').click()
      await expect(page.locator('.skeleton-list')).toBeVisible()
      await capture('home-loading')
      await page.goto('/?ui-state=error')
      await page.locator('.profile-panel .btn--primary').click()
      await expect(page.getByRole('alert')).toBeVisible()
      await capture('home-error')
      await page.goto('/')
      await prepare(page)
      await page.getByLabel(pt.ackAll).check()
      await page.locator('.action-bar .primary').click()
      await expect(page.locator('.success-check')).toBeVisible()
      await capture('results-success')
    })
