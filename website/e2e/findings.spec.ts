// One regression test per finding of ux/site/findings (its regression_asset), each written to fail
// on the site the finding was seen on and to pass after its change. A finding still open is
// marked test.fail with its id: the gate stays green and the debt stays visible, and the test
// turns red the day the defect is gone, so the mark goes with the fix. SUF-001's asset is the
// frame check of e2e/pages.spec.ts under `docusaurus serve`, and the test below on a host with
// the layout of GitHub Pages.

import AxeBuilder from '@axe-core/playwright';
import {expect, test, type Page} from '@playwright/test';
import {settle, where} from './bot';
import {frameProblems, watchFrames} from './frames';
import {overflow} from './layout';
import {pagesHost} from './pages-host';
import {adapter, open, sitemapPaths, type Viewport} from './site';

const DESK = {width: 1440, height: 900};

async function at(page: Page, path: string, vp: Viewport = DESK): Promise<void> {
  await page.setViewportSize(vp);
  await open(page, path, 'light');
  await settle(page);
}

test('SUF-001 every architecture frame shows its architecture on a host with the layout of GitHub Pages', async ({page}) => {
  const host = await pagesHost();
  try {
    const statuses = watchFrames(page);
    await page.setViewportSize(DESK);
    const problems: string[] = [];
    const paths = sitemapPaths().filter((p) => p.startsWith('/docs/architectures/'));
    expect(paths.length).toBeGreaterThan(0);
    for (const path of paths) {
      await page.goto(`${host.origin}${adapter.base_path}${path}`, {waitUntil: 'load'});
      await settle(page);
      problems.push(...(await frameProblems(page, statuses)).map((p) => `${path}: ${p}`));
    }
    expect(problems).toEqual([]);
  } finally {
    await host.close();
  }
});

test('SUF-002 an architecture page links to its Marked places from its first screen', async ({page}) => {
  await at(page, '/docs/architectures/author-reviewer');
  const link = page.locator('main').getByRole('link', {name: 'Marked places', exact: true});
  expect(await where(link), 'a link to Marked places is in the first screen').toBe(1);
});

test('SUF-003 the layer map shows the harness card in its first screen at 1440x900', async ({page}) => {
  test.fail(true, 'SUF-003 open');
  await at(page, '/docs/blocks');
  expect(await where(page.locator('.as-map').getByRole('link', {name: /^harness\b/}))).toBe(1);
});

test('SUF-004 a filter chip of the KPI catalogue names its label and its count apart', async ({page}) => {
  test.fail(true, 'SUF-004 open');
  await at(page, '/docs/kpis-catalogue');
  const names = await page.locator('.as-kpis__chips').getByRole('button').evaluateAll((bs) => bs.map((b) => (b as HTMLElement).innerText.replace(/\s+/g, ' ').trim()));
  const fused = (await page.locator('.as-kpis__chips').ariaSnapshot()).split('\n').filter((l) => /button "[^"]*[a-z]\d+"/.test(l));
  expect(names.length).toBeGreaterThan(0);
  expect(fused, 'accessible names such as "gate34"').toEqual([]);
});

test('SUF-005 from the skip link the KPIs entry of the harness page contents is within 30 Tab presses', async ({page}) => {
  await at(page, '/docs/blocks/4-agent/harness');
  await page.keyboard.press('Tab');
  await page.keyboard.press('Enter');
  const toc = page.locator('.table-of-contents').getByRole('link', {name: 'KPIs', exact: true});
  let tabs = 0;
  let reached = false;
  while (tabs < 30 && !reached) {
    await page.keyboard.press('Tab');
    tabs += 1;
    reached = await toc.evaluateAll((els) => (els as Element[]).includes(document.activeElement as Element));
  }
  console.log(`SUF-005: the KPIs entry of the harness contents after ${tabs} Tab presses, ${tabs + 2} key presses with the skip link`);
  expect(reached, 'the KPIs entry of the contents has focus within 30 Tab presses').toBe(true);
});

test('SUF-006 the KPI catalogue table does not scroll sideways at 1024x768', async ({page}) => {
  test.fail(true, 'SUF-006 open');
  await at(page, '/docs/kpis-catalogue', {width: 1024, height: 768});
  expect((await overflow(page)).filter((p) => p.includes('as-kpis__scroll'))).toEqual([]);
});

test('SUF-007 no table or lifecycle of the harness page scrolls sideways at 1024x768', async ({page}) => {
  test.fail(true, 'SUF-007 open');
  await at(page, '/docs/blocks/4-agent/harness', {width: 1024, height: 768});
  expect((await overflow(page)).filter((p) => p.startsWith('scrolls sideways'))).toEqual([]);
});

test('SUF-008 a lifecycle diagram that scrolls on a phone can be reached by keyboard', async ({page}) => {
  await at(page, '/docs/blocks/4-agent/harness', {width: 390, height: 844});
  // The region attributes are set after hydration, when a ResizeObserver has measured the box. On a
  // loaded machine that comes after the load event: wait for it (at most 20 s), then let the checks
  // below report what is still missing.
  await page
    .waitForFunction(
      () =>
        Array.from(document.querySelectorAll('main *')).every((el) => {
          const s = getComputedStyle(el);
          const scrolls = (s.overflowX === 'auto' || s.overflowX === 'scroll') && el.scrollWidth > el.clientWidth + 1;
          return !scrolls || el.closest('pre') !== null || (el as HTMLElement).tabIndex === 0;
        }),
      undefined,
      {timeout: 20_000},
    )
    .catch(() => undefined);
  const r = await new AxeBuilder({page}).withRules(['scrollable-region-focusable']).analyze();
  expect(r.violations.flatMap((v) => v.nodes.map((n) => n.target.join(' ')))).toEqual([]);
  // Each box that scrolls sideways is a region named for what it holds, so the focus says where it is.
  const unnamed = await page.locator('main').evaluate((m) =>
    Array.from(m.querySelectorAll('*'))
      .filter((el) => {
        const s = getComputedStyle(el);
        return (s.overflowX === 'auto' || s.overflowX === 'scroll') && el.scrollWidth > el.clientWidth + 1 && !el.closest('pre');
      })
      .filter((el) => el.getAttribute('role') !== 'region' || !el.getAttribute('aria-label') || (el as HTMLElement).tabIndex !== 0)
      .map((el) => el.className),
  );
  expect(unnamed, 'boxes that scroll sideways without a focusable named region').toEqual([]);
});

test('SUF-009 the layer map does not scroll sideways at 390x844', async ({page}) => {
  test.fail(true, 'SUF-009 open');
  await at(page, '/docs/blocks', {width: 390, height: 844});
  expect((await overflow(page)).filter((p) => p.includes('as-map__frame'))).toEqual([]);
});

test('SUF-010 the links of the menu end inside its drawer at 768x1024', async ({page}) => {
  test.fail(true, 'SUF-010 open');
  await at(page, '/', {width: 768, height: 1024});
  await page.getByRole('button', {name: 'Toggle navigation bar'}).click();
  await settle(page);
  const link = page.getByRole('navigation', {name: 'Main'}).getByRole('link', {name: 'Building blocks', exact: true});
  const fits = await link.evaluate((el) => {
    const drawer = el.closest('.navbar-sidebar');
    return !!drawer && el.getBoundingClientRect().right <= drawer.getBoundingClientRect().right + 0.5;
  });
  expect(fits, 'the link ends inside the drawer').toBe(true);
});

test('SUF-011 the site title in the open menu is not cut off at 768x1024', async ({page}) => {
  test.fail(true, 'SUF-011 open');
  await at(page, '/', {width: 768, height: 1024});
  await page.getByRole('button', {name: 'Toggle navigation bar'}).click();
  await settle(page);
  expect((await overflow(page)).filter((p) => p.startsWith('text cut off'))).toEqual([]);
});

/** The smallest computed font size of each selector's matches in `main`, below 14 px only. */
async function under14(page: Page, selectors: string[]): Promise<string[]> {
  return page.locator('main').evaluate((m, sels) => {
    const out: string[] = [];
    for (const sel of sels) {
      const els = Array.from(m.querySelectorAll(sel)).filter((el) => (el as HTMLElement).offsetParent !== null);
      if (!els.length) {
        out.push(`${sel}: none on the page`);
        continue;
      }
      const min = Math.min(...els.map((el) => parseFloat(getComputedStyle(el).fontSize)));
      if (min < 13.95) out.push(`${sel}: ${min.toFixed(2)} px`);
    }
    return out;
  }, selectors);
}

test('SUF-013 the gate marker and the KPI table cells are at least 14 px at 1440x900', async ({page}) => {
  await at(page, '/docs/kpis-catalogue');
  const catalogue = await under14(page, ['.as-kpis .as-tag--gate', '.as-kpis .as-tag--planned', '.as-kpis thead th', '.as-kpis thead button', '.as-kpis tbody td', '.as-kpis tbody td code', '.as-chip small']);
  await at(page, '/docs/blocks/4-agent/harness');
  const block = await under14(page, ['article table th', 'article table td', 'article table td code']);
  await at(page, '/docs/architectures/author-reviewer');
  const arch = await under14(page, ['article table th', 'article table td', 'article .as-tag--marked']);
  expect([...catalogue, ...block.map((b) => `harness ${b}`), ...arch.map((a) => `author-reviewer ${a}`)]).toEqual([]);
});
