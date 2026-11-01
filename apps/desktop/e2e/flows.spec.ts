import { test, expect } from '@playwright/test'
import AxeBuilder from '@axe-core/playwright'
test('simulate, review, acknowledge and undo through a mock backend', async ({ page }) => {
  await page.goto('/')
  await page.getByRole('button', { name: /^Simule/ }).click()
  await expect(page.getByText('Análise concluída. Nenhuma alteração foi feita')).toBeVisible()
  await page.getByRole('button', { name: 'Revise os ajustes' }).click()
  await expect(page.getByRole('button', { name: 'Aplique os ajustes' })).toBeDisabled()
  await page.getByLabel('Revisei esta operação e autorizo continuar').check()
  await page.getByRole('button', { name: 'Aplique os ajustes' }).click()
  await expect(page.getByText('Operação concluída. Confira o resultado por item')).toBeVisible()
  await page.getByRole('button', { name: 'Desfaça tudo', exact: true }).click()
  await page.getByLabel('Revisei esta operação e autorizo continuar').check()
  await page.getByRole('button', { name: 'Desfaça', exact: true }).click()
  await expect(page.getByText('Operação concluída. Confira o resultado por item')).toBeVisible()
  expect(await page.evaluate(() => window.__gametuneTest?.executions.length)).toBe(2)
})
test('multi-app removal needs every consent and reinforced high-risk approval', async ({
  page,
}) => {
  await page.goto('/')
  await page.getByRole('button', { name: /^Simule/ }).click()
  await expect(page.getByText('Análise concluída. Nenhuma alteração foi feita')).toBeVisible()
  await page.getByRole('button', { name: 'Limpeza de apps' }).click()
  await page.getByRole('checkbox', { name: 'Remover Microsoft.Copilot', exact: true }).check()
  await page
    .getByRole('checkbox', { name: 'Remover Microsoft.GamingServices', exact: true })
    .check()
  await page.getByRole('button', { name: /Revise a remoção/ }).click()
  const remove = page.getByRole('button', { name: 'Remova os apps' })
  await page.getByLabel('Revisei esta operação e autorizo continuar').check()
  await page
    .getByLabel('Entendo que Desfazer não reinstala este app nem recupera seus dados')
    .nth(0)
    .check()
  await expect(remove).toBeDisabled()
  await page
    .getByLabel('Entendo que Desfazer não reinstala este app nem recupera seus dados')
    .nth(1)
    .check()
  await expect(remove).toBeDisabled()
  await page.getByLabel('Li e aceito o risco deste item').check()
  await expect(remove).toBeDisabled()
  await page.getByLabel('Revisei as fontes e aceito usar este item não verificado').check()
  await expect(remove).toBeEnabled()
  expect(await page.evaluate(() => window.__gametuneTest?.executions.length)).toBe(0)
})
test('cancel remains responsive and a new review is required after UAC cancellation', async ({
  page,
}) => {
  await page.goto('/?uac-cancel=1')
  await page.getByRole('button', { name: /^Simule/ }).click()
  await page.getByRole('button', { name: 'Cancele' }).click()
  await expect(page.getByText('Análise cancelada. Nenhuma alteração foi feita')).toBeVisible()
  expect(await page.evaluate(() => window.__gametuneTest?.executions.length)).toBe(0)
  await page.getByRole('button', { name: /^Simule/ }).click()
  await expect(page.getByText('Análise concluída. Nenhuma alteração foi feita')).toBeVisible()
  await page.getByRole('button', { name: 'Revise os ajustes' }).click()
  await page.getByLabel('Revisei esta operação e autorizo continuar').check()
  await page.getByRole('button', { name: 'Aplique os ajustes' }).click()
  await expect(page.getByRole('alert')).toContainText('Autorização do Windows cancelada')
  await expect(page.getByRole('button', { name: 'Revisão', exact: true })).toBeDisabled()
})
test('keyboard navigation, dialog focus and local settings', async ({ page }) => {
  await page.goto('/')
  await page.getByRole('link', { name: 'Ir para o conteúdo' }).waitFor({ state: 'attached' })
  await page.keyboard.press('Tab')
  await expect(page.getByRole('link', { name: 'Ir para o conteúdo' })).toBeFocused()
  await page.getByRole('button', { name: 'Itens', exact: true }).click()
  await page
    .getByLabel('Veja os detalhes: Desativar Diagnóstico conectado', { exact: true })
    .click()
  await expect(page.getByRole('dialog')).toBeVisible()
  const modalAxe = await new AxeBuilder({ page })
    .withTags(['wcag2a', 'wcag2aa', 'wcag21aa'])
    .analyze()
  expect(modalAxe.violations).toEqual([])
  await page.keyboard.press('Escape')
  await expect(page.getByRole('dialog')).not.toBeVisible()
  await expect(
    page.getByLabel('Veja os detalhes: Desativar Diagnóstico conectado', { exact: true }),
  ).toBeFocused()
  await page.getByRole('button', { name: 'Configurações', exact: true }).click()
  await page.getByLabel('Idioma', { exact: true }).selectOption('en')
  await page.getByLabel('Light', { exact: true }).check()
  await page.reload()
  await expect(page.locator('html')).toHaveAttribute('lang', 'en')
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'light')
})
for (const language of ['pt-BR', 'en'])
  for (const theme of ['dark', 'light'])
    test(`axe AA on all screens in ${language} ${theme}`, async ({ page }) => {
      await page.goto('/')
      await page.getByRole('button', { name: /^Simule/ }).click()
      await expect(page.getByText('Análise concluída. Nenhuma alteração foi feita')).toBeVisible()
      if (theme === 'light' || language === 'en') {
        await page.getByRole('button', { name: 'Configurações', exact: true }).click()
        if (theme === 'light') await page.getByLabel('Claro', { exact: true }).check()
        if (language === 'en') await page.getByLabel('Idioma', { exact: true }).selectOption('en')
      }
      const names =
        language === 'en'
          ? ['Home', 'Items', 'App cleanup', 'History', 'Settings']
          : ['Início', 'Itens', 'Limpeza de apps', 'Histórico', 'Configurações']
      for (const name of names) {
        await page.getByRole('button', { name, exact: true }).click()
        const results = await new AxeBuilder({ page })
          .withTags(['wcag2a', 'wcag2aa', 'wcag21aa'])
          .analyze()
        expect(results.violations, `${theme}: ${name}`).toEqual([])
      }
      await page
        .getByRole('button', { name: language === 'en' ? 'Home' : 'Início', exact: true })
        .click()
      if (language === 'pt-BR')
        await page.screenshot({ path: `test-results/home-${theme}.png`, fullPage: true })
      await page
        .getByRole('button', {
          name: language === 'en' ? 'Review adjustments' : 'Revise os ajustes',
        })
        .click()
      const results = await new AxeBuilder({ page })
        .withTags(['wcag2a', 'wcag2aa', 'wcag21aa'])
        .analyze()
      expect(results.violations).toEqual([])
    })

test('Store reinstall is a separately reviewed explicit action', async ({ page }) => {
  await page.goto('/')
  await page.getByRole('button', { name: 'Histórico', exact: true }).click()
  await page.getByRole('button', { name: 'Abra na Loja' }).click()
  await expect(page.getByRole('button', { name: 'Abra na Loja' })).toBeDisabled()
  expect(await page.evaluate(() => window.__gametuneTest?.executions.length)).toBe(0)
  await page.getByLabel('Revisei esta operação e autorizo continuar').check()
  await page.getByRole('button', { name: 'Abra na Loja' }).click()
  await expect(page.getByText('Operação concluída. Confira o resultado por item')).toBeVisible()
  expect(await page.evaluate(() => window.__gametuneTest?.reviews[0]?.kind)).toBe('reinstall')
})
test('responsive layouts stay within the viewport and make no external requests', async ({
  page,
}) => {
  const external: string[] = []
  page.on('request', (request) => {
    if (!request.url().startsWith('http://127.0.0.1:1420') && !request.url().startsWith('data:'))
      external.push(request.url())
  })
  await page.goto('/')
  for (const width of [1440, 768, 390]) {
    await page.setViewportSize({ width, height: 900 })
    await expect(page.getByRole('heading', { level: 1 })).toBeVisible()
    expect(
      await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth),
    ).toBe(true)
    await page.screenshot({ path: `test-results/home-${width}.png`, fullPage: true })
  }
  expect(external).toEqual([])
})
