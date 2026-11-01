// Rendered-background check supplements axe's pseudoContent limitation.
// Development only: own Vite child + mock backend; no application IPC or system writes.
import { chromium } from "../apps/desktop/node_modules/playwright/index.mjs";
import { spawn } from "node:child_process";
import { readFileSync, mkdirSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
const app = fileURLToPath(new URL("../apps/desktop/", import.meta.url));
const out = new URL("../work/glass-contrast/", import.meta.url);
mkdirSync(out, { recursive: true });
const server = spawn(
  process.execPath,
  [
    app + "node_modules/vite/bin/vite.js",
    "--mode",
    "test",
    "--host",
    "127.0.0.1",
    "--port",
    "1421",
    "--strictPort",
  ],
  { cwd: app, windowsHide: true, stdio: "ignore" },
);
const browser = await chromium.launch();
const reports = [];
try {
  for (const language of ["pt-BR", "en"])
    for (const theme of ["light", "dark"])
      for (const width of [1440, 1100, 768, 390]) {
        const page = await browser.newPage({
          viewport: { width, height: 1000 },
          reducedMotion: "reduce",
        });
        const t = JSON.parse(
          readFileSync(app + `src/locales/${language}.json`, "utf8"),
        );
        await page.addInitScript(
          ({ language, theme }) => {
            localStorage.setItem("ametista.language", language);
            localStorage.setItem("ametista.theme", theme);
            localStorage.setItem("ametista.effects", "full");
          },
          { language, theme },
        );
        for (let n = 0; ; n++) {
          try {
            await page.goto("http://127.0.0.1:1421/?benchmark-complete=1");
            break;
          } catch (e) {
            if (n > 30) throw e;
            await new Promise((resolve) => setTimeout(resolve, 200));
          }
        }
        await page.getByRole("radio", { name: t.safe, exact: true }).waitFor();
        await page
          .getByRole("img", { name: `${t.availableRam}: 6 GiB`, exact: true })
          .waitFor();
        async function measure(name) {
          await page.evaluate(() => document.fonts.ready);
          await page.waitForTimeout(100); // Settle finite page entrance before recording geometry.
          const texts = await page.evaluate(() => {
            const walker = document.createTreeWalker(
              document.body,
              NodeFilter.SHOW_TEXT,
            );
            const texts = [];
            for (let node; (node = walker.nextNode());) {
              const el = node.parentElement;
              if (
                !el ||
                !node.textContent.trim() ||
                el.closest(
                  'script, style, [hidden], [aria-hidden="true"], :disabled, [aria-disabled="true"]',
                )
              )
                continue;
              const style = getComputedStyle(el);
              if (
                style.visibility !== "visible" ||
                style.display === "none" ||
                Number(style.opacity) !== 1
              )
                continue;
              const range = document.createRange();
              range.selectNodeContents(node);
              for (const rect of range.getClientRects()) {
                if (
                  rect.width < 2 ||
                  rect.height < 2 ||
                  rect.top < 0 ||
                  rect.bottom > innerHeight ||
                  rect.left < 0 ||
                  rect.right > innerWidth
                )
                  continue;
                const points = [
                  [0.1, 0.1],
                  [0.9, 0.1],
                  [0.5, 0.5],
                  [0.1, 0.9],
                  [0.9, 0.9],
                ];
                if (
                  points.some(([fx, fy]) => {
                    const top = document.elementFromPoint(
                      rect.x + rect.width * fx,
                      rect.y + rect.height * fy,
                    );
                    return top && !el.contains(top) && !top.contains(el);
                  })
                )
                  continue;
                texts.push({
                  text: node.textContent.trim().slice(0, 110),
                  color: style.color,
                  size: parseFloat(style.fontSize),
                  weight: Number(style.fontWeight),
                  x: rect.x,
                  y: rect.y,
                  w: rect.width,
                  h: rect.height,
                });
              }
            }
            return texts;
          });
          const hidden = await page.addStyleTag({
            content:
              "* { color: transparent !important; -webkit-text-fill-color: transparent !important; text-shadow: none !important; }",
          });
          const background = await page.screenshot({ animations: "disabled" });
          await hidden.evaluate((el) => el.remove());
          const samples = await page.evaluate(
            async ({ base64, texts }) => {
              const img = new Image();
              img.src = `data:image/png;base64,${base64}`;
              await img.decode();
              const canvas = document.createElement("canvas");
              canvas.width = img.width;
              canvas.height = img.height;
              const ctx = canvas.getContext("2d", { willReadFrequently: true });
              ctx.drawImage(img, 0, 0);
              const pixels = ctx.getImageData(
                0,
                0,
                canvas.width,
                canvas.height,
              ).data;
              const luminance = (rgb) =>
                rgb
                  .map((c) => {
                    c /= 255;
                    return c <= 0.04045
                      ? c / 12.92
                      : ((c + 0.055) / 1.055) ** 2.4;
                  })
                  .reduce((v, c, i) => v + c * [0.2126, 0.7152, 0.0722][i], 0);
              return texts.map((t) => {
                const fg = t.color.match(/[\d.]+/g).map(Number);
                let minimum = Infinity;
                for (let ix = 0; ix < 9; ix++)
                  for (let iy = 0; iy < 5; iy++) {
                    const x = Math.max(
                        0,
                        Math.min(
                          canvas.width - 1,
                          Math.round(t.x + ((ix + 0.5) * t.w) / 9),
                        ),
                      ),
                      y = Math.max(
                        0,
                        Math.min(
                          canvas.height - 1,
                          Math.round(t.y + ((iy + 0.5) * t.h) / 5),
                        ),
                      );
                    const offset = (y * canvas.width + x) * 4,
                      bg = Array.from(pixels.slice(offset, offset + 3));
                    const alpha = fg[3] ?? 1,
                      painted = fg
                        .slice(0, 3)
                        .map((v, i) => v * alpha + bg[i] * (1 - alpha));
                    const a = luminance(painted),
                      b = luminance(bg);
                    minimum = Math.min(
                      minimum,
                      (Math.max(a, b) + 0.05) / (Math.min(a, b) + 0.05),
                    );
                  }
                return {
                  ...t,
                  minimum,
                  required:
                    t.size >= 24 || (t.size >= 18.667 && t.weight >= 700)
                      ? 3
                      : 4.5,
                };
              });
            },
            { base64: background.toString("base64"), texts },
          );
          reports.push({
            language,
            theme,
            width,
            page: name,
            samples,
            failures: samples.filter((s) => s.minimum < s.required),
          });
        }
        await measure("home");
        const shimmer = await page.addStyleTag({
          content:
            ".btn--primary::after, .button.primary::after { transform: translateX(0) !important; transition: none !important; }",
        });
        await measure("home-shimmer");
        await shimmer.evaluate((el) => el.remove());
        await page
          .getByRole("button", { name: t.simulate, exact: true })
          .click();
        await page.getByText(t.complete, { exact: true }).waitFor();
        await page
          .getByRole("button", { name: t.reviewSelection, exact: true })
          .click();
        for (const name of [
          "items",
          "apps",
          "review",
          "history",
          "benchmark",
          "games",
          "settings",
        ]) {
          await page
            .getByRole("button", { name: t[name], exact: true })
            .click();
          await measure(name);
        }
        await page.close();
        console.log(`${language} ${theme} ${width}: rendered text checked`);
      }
  const failures = reports.flatMap((r) =>
    r.failures.map((f) => ({
      language: r.language,
      theme: r.theme,
      width: r.width,
      page: r.page,
      ...f,
    })),
  );
  writeFileSync(
    new URL("report.json", out),
    JSON.stringify(
      {
        method:
          "45 background pixel samples per visible text range; text hidden only during background capture; WCAG luminance; disabled and occluded text excluded",
        limitations:
          "Visible viewport only; sampled grid, not every glyph pixel; complements axe and visual review; no screen-reader or native WebView validation",
        cases: reports.length,
        textRanges: reports.reduce((n, r) => n + r.samples.length, 0),
        failures,
        reports,
      },
      null,
      2,
    ),
  );
  console.log(
    JSON.stringify({
      cases: reports.length,
      textRanges: reports.reduce((n, r) => n + r.samples.length, 0),
      failures: failures.slice(0, 10),
      totalFailures: failures.length,
    }),
  );
  process.exitCode = failures.length ? 1 : 0;
} finally {
  await browser.close();
  server.kill(); // Only the Vite child created above, never a user's application.
}
