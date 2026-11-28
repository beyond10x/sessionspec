import path from 'node:path';
import AxeBuilder from '@axe-core/playwright';
import { expect, test, type Page } from '@playwright/test';

// The studio app (ui/studio, generated from studio/studio.ui.yaml) in a browser, served by one
// agenticstudio serve. First the acceptance of the first slice: a draft of
// architectures/workstation.yaml takes a `set` patch as revision 2, then refuses a patch whose
// second rule has a verb the patch language does not know, and stays as it was. Then the layout
// on the largest public architecture, axe-core and overflow in both themes, and the accent token.

const SET = 'set(P, count, 3) :- part{id: P, block: "agent", count: 12}.';
const UNKNOWN_VERB = 'set(P, count, 2) :- part{id: P, block: "agent"}.\nfrobnicate(P) :- part{id: P}.';
const screens = path.resolve(__dirname, '../../../target/screens');
const SIZES = [
  { width: 1366, height: 768 },
  { width: 390, height: 844 },
];

const summary = (page: Page, field: string) => page.locator(`[data-panel="summary"] [data-field="${field}"]`);
const partCount = (page: Page, part: string) =>
  page.locator(`[data-panel="parts"] tr[data-row="${part}"] [data-field="count"]`);
const result = (page: Page) => page.locator('[data-panel="patch"] [data-field="message"]');

async function home(page: Page): Promise<void> {
  await page.goto(process.env.STUDIO_URL!);
  await expect(page.locator('body[data-ready="true"]')).toBeAttached();
}

/** Chooses an architecture in the header's select and opens a draft of it; lands on its page. */
async function openDraft(page: Page, name: string): Promise<void> {
  await home(page);
  await page.getByRole('combobox', { name: 'Architecture' }).click();
  await page.getByRole('option', { name, exact: true }).click();
  await page.getByRole('button', { name: 'Open draft', exact: true }).click();
  await expect(page).toHaveURL(/#\/draft\/[0-9a-f-]{36}$/);
  await expect(page.locator('[data-slot="draft-graph"] svg.graph')).toBeVisible();
  await expect(summary(page, 'revision')).toBeVisible();
}

async function showParts(page: Page): Promise<void> {
  const toggle = page.locator('[data-panel="parts"] button[aria-expanded]');
  if ((await toggle.getAttribute('aria-expanded')) === 'false') await toggle.click();
}

async function apply(page: Page, text: string): Promise<void> {
  await page.locator('#patch-arguments').fill(text);
  await page.getByRole('button', { name: 'Apply', exact: true }).click();
}

async function theme(page: Page, t: 'light' | 'dark'): Promise<void> {
  await page.locator(`[data-theme-choice="${t}"]`).click();
  await expect(page.locator('html')).toHaveAttribute('data-theme', t);
}

async function serious(page: Page): Promise<string[]> {
  const r = await new AxeBuilder({ page }).analyze();
  return r.violations
    .filter((v) => v.impact === 'serious' || v.impact === 'critical')
    .map((v) => `${v.id}: ${v.nodes.map((n) => n.target.join(' ')).join(', ')}`);
}

async function overflow(page: Page): Promise<number> {
  return page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
}

test.describe('the first slice', () => {
  test.describe.configure({ mode: 'serial' });
  let draft = '';

  test('a set patch on a draft at revision 1 shows revision 2, worker at 3, operator unchanged and the check passing', async ({ page }) => {
    await openDraft(page, 'workstation');
    await showParts(page);
    await expect(summary(page, 'revision')).toHaveText('1');
    await expect(partCount(page, 'worker')).toHaveText('12');
    const operator = (await page.locator('[data-panel="parts"] tr[data-row="operator"]').textContent()) ?? '';
    expect(operator).toContain('operator');

    await apply(page, SET);

    await expect(result(page)).toContainText('Applied as revision 2');
    await expect(summary(page, 'revision')).toHaveText('2');
    await expect(partCount(page, 'worker')).toHaveText('3');
    await expect(page.locator('[data-panel="parts"] tr[data-row="operator"]')).toHaveText(operator);
    await expect(summary(page, 'passed')).toHaveText('passes');
    await expect(page.locator('[data-graph-facts]')).toContainText('revision 2');
    draft = page.url();
  });

  test('a patch whose rule 2 has an unknown verb shows the refusal and leaves revision 2 as it was', async ({ page }) => {
    expect(draft).toMatch(/#\/draft\//);
    await page.goto(draft);
    await expect(summary(page, 'revision')).toHaveText('2');
    const digest = (await summary(page, 'digest').getAttribute('data-value')) ?? '';
    expect(digest).toMatch(/^[0-9a-f]{64}$/);
    await expect(summary(page, 'digest')).toHaveText(`${digest.slice(0, 12)}…`);

    await apply(page, UNKNOWN_VERB);

    await expect(result(page)).toContainText('Refused at rule 2');
    await expect(result(page)).toContainText('UnknownVerb');
    await expect(page.locator('[data-panel="patch"] [data-field="outcome"]')).toHaveText('Refused');
    await expect(summary(page, 'revision')).toHaveText('2');
    await expect(summary(page, 'digest')).toHaveAttribute('data-value', digest);
    await showParts(page);
    await expect(partCount(page, 'worker')).toHaveText('3');
  });
});

test.describe('live', () => {
  test('two pages on one draft: an apply in the first and an agent call both show in the second without a reload', async ({ page, browser }) => {
    await openDraft(page, 'workstation');
    const id = page.url().split('/').pop()!;
    const other = await (await browser.newContext()).newPage();
    await other.goto(page.url());
    await expect(summary(other, 'revision')).toHaveText('1');
    await other.evaluate(() => ((window as unknown as { stamp: number }).stamp = 1));

    await apply(page, SET);
    await expect(result(page)).toContainText('Applied as revision 2');
    await expect(summary(other, 'revision')).toHaveText('2', { timeout: 2_000 });

    const api = (p: string, body: unknown, headers: Record<string, string> = {}) =>
      other.request.post(`${process.env.STUDIO_URL}api/${p}`, { data: body, headers }).then((r) => r.json());
    const session = (await api('sessions', { draft_id: id })).session.session_id;
    const call = await api(
      `sessions/${session}/calls`,
      {
        tool: 'apply',
        arguments: { patch: 'set(P, count, 5) :- part{id: P, block: "agent", count: 3}.' },
      },
      { 'X-Studio-Caller': 'Agent' },
    );
    expect(call.call.answer.outcome).toBe('Applied');
    expect(call.call.caller).toBe('Agent');
    await expect(summary(other, 'revision')).toHaveText('3', { timeout: 2_000 });
    await expect(summary(page, 'revision')).toHaveText('3', { timeout: 2_000 });
    await expect(other.locator('[data-graph-facts]')).toContainText('revision 3');
    expect(await other.evaluate(() => (window as unknown as { stamp?: number }).stamp)).toBe(1);
    await other.context().close();
  });
});

test.describe('the largest public architecture', () => {
  test('on 09-factory at 1366x768 a part label reads at 11 px or more and the patch editor is in view', async ({ page }) => {
    await page.setViewportSize({ width: 1366, height: 768 });
    await openDraft(page, 'ladder/09-factory');
    const label = page.locator('[data-slot="draft-graph"] svg.graph text.name').first();
    await expect(label).toBeVisible();
    const px = await label.evaluate((el) => {
      const t = el as SVGTextElement;
      const m = t.getScreenCTM();
      return parseFloat(getComputedStyle(t).fontSize) * (m ? Math.hypot(m.a, m.b) : 1);
    });
    expect(px).toBeGreaterThanOrEqual(11);
    const editor = await page.locator('#patch-arguments').boundingBox();
    expect(editor).not.toBeNull();
    expect(editor!.y).toBeGreaterThanOrEqual(0);
    expect(editor!.x).toBeGreaterThanOrEqual(0);
    expect(editor!.y + editor!.height).toBeLessThanOrEqual(768);
    expect(editor!.x + editor!.width).toBeLessThanOrEqual(1366);
    await expect(page.getByRole('button', { name: 'Apply', exact: true })).toBeInViewport({ ratio: 1 });
    expect(await page.locator('iframe').count()).toBe(0);
  });

  test('clicking a part shows its properties; zoom and fit change the scale', async ({ page }) => {
    await page.setViewportSize({ width: 1366, height: 768 });
    await openDraft(page, 'ladder/09-factory');
    const node = page.locator('[data-slot="draft-graph"] .node[data-part]').first();
    const id = (await node.getAttribute('data-part')) ?? '';
    await node.click();
    const card = page.locator('[data-selected-part]');
    await expect(card).toHaveAttribute('data-part', id);
    await expect(card).toContainText('block');
    await expect(node.locator(':scope > [role="button"]')).toHaveAttribute('aria-pressed', 'true');
    const viewport = page.locator('[data-slot="draft-graph"] [data-scale]');
    const before = Number(await viewport.getAttribute('data-scale'));
    await page.getByRole('button', { name: 'Zoom in' }).click();
    await expect.poll(async () => Number(await viewport.getAttribute('data-scale'))).toBeGreaterThan(before);
    await page.getByRole('button', { name: 'Fit' }).click();
    await expect.poll(async () => Number(await viewport.getAttribute('data-scale'))).toBeLessThan(before);
    await page.keyboard.press('Escape');
  });

  test('the draft view shows the block panels of ui/views for a part, on their fixtures', async ({ page }) => {
    await page.setViewportSize({ width: 1366, height: 768 });
    await openDraft(page, 'workstation');
    const slot = page.locator('[data-slot="draft-panels"]');
    const toggle = page.locator('[data-panel="panels"] button[aria-expanded]').first();
    if ((await toggle.getAttribute('aria-expanded')) === 'false') await toggle.click();
    await expect(slot).toHaveAttribute('data-part', /.+/);
    await expect(slot.locator('.island [data-panel="rows"] tr[data-row]').first()).toBeAttached();
    await slot.getByRole('combobox', { name: 'Part' }).click();
    await page.getByRole('option', { name: /^builds / }).click();
    await expect(slot.locator('.island')).toHaveAttribute('data-block', 'pool');
    await expect(slot.locator('.island [data-panel="rows"] tr[data-row]').first()).toBeAttached();
  });

  test('the description is two lines with more, in the side panel', async ({ page }) => {
    await page.setViewportSize({ width: 1366, height: 768 });
    await openDraft(page, 'ladder/09-factory');
    const description = summary(page, 'description');
    await expect(description).toBeVisible();
    const lines = await description.locator('p').evaluate((p) => p.clientHeight / parseFloat(getComputedStyle(p).lineHeight));
    expect(lines).toBeLessThanOrEqual(2.05);
    await expect(description.getByRole('button', { name: 'more' })).toBeVisible();
    const side = await page.locator('aside').first().boundingBox();
    const box = await description.boundingBox();
    expect(box!.x).toBeGreaterThanOrEqual(side!.x);
  });
});

test.describe('axe and overflow', () => {
  for (const size of SIZES) {
    for (const t of ['light', 'dark'] as const) {
      test(`no serious or critical finding and no horizontal overflow at ${size.width}x${size.height}, ${t}`, async ({ page }) => {
        await page.setViewportSize(size);
        await openDraft(page, 'workstation');
        await theme(page, t);
        await showParts(page);
        await apply(page, UNKNOWN_VERB);
        await expect(result(page)).toContainText('Refused at rule 2');
        expect(await serious(page)).toEqual([]);
        expect(await overflow(page)).toBeLessThanOrEqual(0);
        await page.goto(process.env.STUDIO_URL!);
        await expect(page.getByRole('heading', { name: 'Drafts', exact: true })).toBeVisible();
        expect(await serious(page)).toEqual([]);
        expect(await overflow(page)).toBeLessThanOrEqual(0);
      });
    }
  }
});

test('the accent token of ui/kit draws the primary button', async ({ page }) => {
  await home(page);
  const button = page.getByRole('button', { name: 'Open draft', exact: true });
  await page.evaluate(() => document.documentElement.style.setProperty('--kit-accent', 'rgb(1, 2, 3)'));
  await expect(button).toHaveCSS('background-color', 'rgb(1, 2, 3)');
});

test('screenshots of the studio on 09-factory in both themes', async ({ page }) => {
  test.skip(!process.env.SCREENS, 'SCREENS=1 writes the screenshots');
  await page.setViewportSize({ width: 1366, height: 768 });
  await openDraft(page, 'ladder/09-factory');
  for (const t of ['light', 'dark'] as const) {
    await theme(page, t);
    await page.mouse.move(0, 0);
    await page.screenshot({ path: path.join(screens, `studio-09-factory-${t}-1366x768.png`), animations: 'disabled' });
  }
  await theme(page, 'light');
  await page.locator('[data-slot="draft-graph"] .node[data-part]').nth(2).click();
  await page.locator('#patch-arguments').fill(UNKNOWN_VERB);
  await page.getByRole('button', { name: 'Apply', exact: true }).click();
  await expect(result(page)).toContainText('Refused at rule 2');
  await page.screenshot({ path: path.join(screens, 'studio-09-factory-selected-refused-light-1366x768.png'), animations: 'disabled' });
  await page.setViewportSize({ width: 390, height: 844 });
  await page.screenshot({ path: path.join(screens, 'studio-09-factory-light-390x844.png'), animations: 'disabled' });
});
