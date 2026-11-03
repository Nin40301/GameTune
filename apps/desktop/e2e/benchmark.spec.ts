import { test, expect } from './fixtures'
import AxeBuilder from '@axe-core/playwright'

for (const locale of ['pt-BR', 'en'] as const)
  for (const theme of ['dark', 'light'] as const) {
    test(`benchmark axe, distributions and exports ${locale} ${theme}`, async ({ page }) => {
      await page.addInitScript(
        ({ locale, theme }) => {
          localStorage.setItem('ametista.language', locale)
          localStorage.setItem('ametista.theme', theme)
        },
        { locale, theme },
      )
      await page.goto('/?benchmark-complete=1')
      await page.getByRole('button', { name: /Medições|Measurements/, exact: true }).click()
      await expect(
        page.getByText(locale === 'en' ? 'Inconclusive' : 'Inconclusivo', { exact: true }),
      ).toBeVisible()
      await expect(
        page.getByRole('checkbox', {
          name:
            locale === 'en' ? 'Name and path anonymization' : 'Anonimização de nomes e caminhos',
        }),
      ).toBeChecked()
      expect(
        (await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa', 'wcag21aa']).analyze())
          .violations,
      ).toEqual([])
      await page.screenshot({
        path: `test-results/benchmark-${locale}-${theme}.png`,
        fullPage: true,
      })
    })
  }
test('guided run is read-only, cancelable and keyboard accessible', async ({ page }) => {
  await page.goto('/')
  await page.getByRole('button', { name: /Medições|Measurements/, exact: true }).click()
  const start = page.getByRole('button', { name: 'Iniciar rodada de 60 s' })
  await expect(start).toBeDisabled()
  for (const box of await page
    .getByRole('group', { name: 'Confirme antes de cada rodada' })
    .getByRole('checkbox')
    .all())
    await box.check()
  await start.focus()
  await page.keyboard.press('Enter')
  await expect(page.getByRole('progressbar', { name: 'Tempo da rodada' })).toBeVisible()
  expect(
    (await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa', 'wcag21aa']).analyze())
      .violations,
  ).toEqual([])
  await page.getByRole('button', { name: 'Descartar rodada' }).click()
  await expect(
    page.getByText('A rodada interrompida foi descartada. As concluídas foram preservadas'),
  ).toBeVisible()
  expect(await page.evaluate(() => window.__ametistaTest?.executions.length)).toBe(0)
})
for (const width of [390, 768, 1440])
  test(`benchmark layout at ${width}`, async ({ page }) => {
    await page.setViewportSize({ width, height: 900 })
    await page.goto('/')
    await page.getByRole('button', { name: /Medições|Measurements/, exact: true }).click()
    expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true)
  })
