import trayPt from '../../../crates/monitor/locales/pt-BR.json'
import trayEn from '../../../crates/monitor/locales/en.json'
import { appendCpuSample } from '../src/state/visualHistory'
import { itemName } from '../src/i18n'
import catalog from '../../../catalog/items.json'
import actions from '../../../catalog/game-actions.json'
import pt from '../src/locales/pt-BR.json'
import en from '../src/locales/en.json'
test('CPU history keeps a real 60-second window even at one-second polling', () => {
  let points: { time: number; value: number }[] = []
  for (let time = 0; time <= 70000; time += 1000)
    points = appendCpuSample(points, { time, value: 10 })
  expect(points).toHaveLength(60)
  expect(points[0]?.time).toBe(11000)
  expect(appendCpuSample(points, { time: 70000, value: 20 })).toBe(points)
  expect(appendCpuSample(points, { time: 140000, value: 5 })).toEqual([{ time: 140000, value: 5 }])
})
test('all catalog and game options have noun labels in both languages; action labels remain verbs', () => {
  for (const language of ['pt-BR', 'en'] as const) {
    const t: Record<string, string> = language === 'en' ? en : pt
    for (const item of catalog.items) {
      expect(t[`itemName_${item.id}`]).toBeTruthy()
      expect(itemName(item, language)).not.toMatch(
        /^(Ative|Pause|Desative|Remover|Disable|Enable|Remove) /,
      )
    }
    for (const action of actions.actions)
      expect(t[`gameAction_${action.id}`]).not.toMatch(/^(Ative|Pause|Use|Enable) /)
  }
  expect(pt.simulate).toBe('Simular')
  expect(pt.analyze).toBe('Analisar')
  expect(pt.reviewSelection).toBe('Revisar ajustes')
  expect(en.simulate).toBe('Simulate')
})

test('tray voice remains localized and uses infinitive labels', () => {
  expect(Object.keys(trayPt).sort()).toEqual(Object.keys(trayEn).sort())
  expect(trayPt.open).toBe('Abrir Ametista')
  expect(trayPt.stop).toBe('Parar e restaurar')
})
