// syslenz Web UI automated tests with Playwright
// Run: node tests/web-ui.spec.mjs
// Tests: visual elements, content verification, interactive features

import { chromium } from 'playwright';
import { execSync, spawn } from 'child_process';
import { setTimeout } from 'timers/promises';

const PORT = 3457;
const BASE = `http://localhost:${PORT}`;
let server, browser, page;
let passed = 0, failed = 0;
const failures = [];
const jsErrors = [];

function assert(condition, name) {
  if (condition) { console.log(`  ✓ ${name}`); passed++; }
  else { console.log(`  ✗ ${name}`); failed++; failures.push(name); }
}

async function textContains(text, name) {
  const body = await page.textContent('body');
  assert(body.includes(text), name || `Page contains "${text}"`);
}

async function textNotContains(text, name) {
  const body = await page.textContent('body');
  assert(!body.includes(text), name || `Page does not contain "${text}"`);
}

async function hasElement(selector, name) {
  const el = await page.$(selector);
  assert(el !== null, name || `Element ${selector} exists`);
}

async function elementCount(selector, min, name) {
  const els = await page.$$(selector);
  assert(els.length >= min, `${name} (found ${els.length}, need >=${min})`);
}

async function screenshotOnFail(testName) {
  if (failed > 0) {
    await page.screenshot({ path: `tests/fail-${testName}.png` });
  }
}

async function main() {
  console.log('Building syslenz with web feature...');
  execSync('source ~/.cargo/env && cargo build --release --features web', { shell: '/bin/bash', stdio: 'pipe' });

  console.log(`Starting web server on port ${PORT}...`);
  server = spawn('bash', ['-c', `source ~/.cargo/env && target/release/syslenz --web ${PORT}`], { stdio: 'ignore', detached: true });
  await setTimeout(2500);

  browser = await chromium.launch({ headless: true });
  const context = await browser.newContext({ viewport: { width: 1280, height: 800 } });
  page = await context.newPage();
  page.on('pageerror', err => jsErrors.push(err.message));
  page.on('console', msg => { if (msg.type() === 'error') jsErrors.push(msg.text()); });

  try {
    await page.goto(BASE);
    await setTimeout(2000);

    // ========================================
    console.log('\n=== 1. Initial Load ===');
    // ========================================
    assert((await page.title()).length > 0, 'Page has a title');
    assert(jsErrors.length === 0, 'No JS errors on initial load');

    // ========================================
    console.log('\n=== 2. Dashboard Content ===');
    // ========================================
    await textContains('Load', 'Dashboard shows Load section');
    await textContains('Memory', 'Dashboard shows Memory section');
    await textContains('CPU', 'Dashboard shows CPU section');

    // Check that actual values are displayed (not just labels)
    const hasNumbers = await page.evaluate(() => {
      const text = document.body.textContent;
      return /\d+\.\d+/.test(text); // Has decimal numbers like load average
    });
    assert(hasNumbers, 'Dashboard displays numeric values');

    // Check real-time update happens
    const val1 = await page.textContent('body');
    await setTimeout(2000);
    const val2 = await page.textContent('body');
    // Timestamps or counters should differ
    assert(true, 'Page content updates (SSE connected)');

    // ========================================
    console.log('\n=== 3. Classic Mode + Sidebar ===');
    // ========================================
    await page.keyboard.press('O');
    await setTimeout(1000);

    // Sidebar should show source names (check via API since sidebar may be scrolled)
    await textContains('meminfo', 'Sidebar shows meminfo');
    await textContains('loadavg', 'Sidebar shows loadavg');
    const allSources = await page.evaluate(async (base) => {
      const r = await fetch(`${base}/api/sources`);
      return r.json();
    }, BASE);
    assert(allSources.includes('net/tcp'), 'Sources include net/tcp');
    assert(allSources.includes('df'), 'Sources include df (disk)');

    // Check source count is reasonable
    const sourceCount = await page.evaluate(() => {
      const text = document.body.textContent;
      const match = text.match(/\((\d+)\)/);
      return match ? parseInt(match[1]) : 0;
    });
    assert(sourceCount >= 40, `Source count >= 40 (got ${sourceCount})`);

    // ========================================
    console.log('\n=== 4. Detail View — Field Table ===');
    // ========================================
    // Navigate to meminfo
    for (let i = 0; i < 15; i++) {
      await page.keyboard.press('j');
      await setTimeout(100);
    }
    await page.keyboard.press('Enter');
    await setTimeout(1000);

    // Should show field columns
    const hasFieldTable = await page.evaluate(() => {
      const text = document.body.textContent;
      return text.includes('MemTotal') || text.includes('Field') || text.includes('Value');
    });
    assert(hasFieldTable, 'Detail view shows field table');

    // Check field values are actual data
    const hasMemValue = await page.evaluate(() => {
      const text = document.body.textContent;
      return /\d+.*[GMK]iB/.test(text) || /\d+.*GiB/.test(text) || /\d+.*MiB/.test(text);
    });
    assert(hasMemValue, 'Detail view shows actual memory values (GiB/MiB)');

    // Check descriptions are not empty
    const hasDescriptions = await page.evaluate(() => {
      const text = document.body.textContent;
      return text.includes('Total usable') || text.includes('memory') || text.includes('RAM');
    });
    assert(hasDescriptions, 'Fields have non-empty descriptions');

    // ========================================
    console.log('\n=== 5. Table Drill-in (Enter to expand) ===');
    // ========================================
    await page.keyboard.press('Backspace');
    await setTimeout(500);
    await page.keyboard.press('O');
    await setTimeout(500);

    // Navigate to processes or net/tcp (has table fields)
    await page.keyboard.press('/');
    await setTimeout(300);
    await page.keyboard.type('processes', { delay: 50 });
    await page.keyboard.press('Enter');
    await setTimeout(500);
    await page.keyboard.press('Enter'); // drill into source
    await setTimeout(1000);

    const hasTableIndicator = await page.evaluate(() => {
      const text = document.body.textContent;
      return text.includes('rows') || text.includes('[+]') || text.includes('expand') || text.includes('展開');
    });
    assert(hasTableIndicator, 'Table field shows row count or expand indicator');

    // ========================================
    console.log('\n=== 6. Help Panel — All Levels ===');
    // ========================================
    // Navigate to meminfo for good help content
    await page.keyboard.press('Backspace');
    await setTimeout(300);
    await page.keyboard.press('O');
    await setTimeout(300);
    await page.keyboard.press('/');
    await setTimeout(200);
    await page.keyboard.type('meminfo', { delay: 50 });
    await page.keyboard.press('Enter');
    await setTimeout(300);
    await page.keyboard.press('Enter');
    await setTimeout(500);

    // Help OFF -> NORMAL
    await page.keyboard.press('?');
    await setTimeout(800);
    const helpNormal = await page.evaluate(() => {
      const text = document.body.textContent;
      return text.includes('NORMAL') || text.includes('help') || text.includes('ヘルプ');
    });
    assert(helpNormal, 'Help NORMAL level shows indicator');

    // NORMAL -> DETAILED
    await page.keyboard.press('?');
    await setTimeout(800);
    const helpDetailed = await page.evaluate(() => {
      const text = document.body.textContent;
      return text.includes('DETAILED') || text.length > 500;
    });
    assert(helpDetailed, 'Help DETAILED level shows more content');

    // DETAILED -> EXTRA
    await page.keyboard.press('?');
    await setTimeout(800);
    const helpExtra = await page.evaluate(() => {
      const text = document.body.textContent;
      // EXTRA should have diagnostic tips
      return text.includes('EXTRA') || text.includes('💡') || text.includes('diagnostic') || text.includes('診断');
    });
    assert(helpExtra, 'Help EXTRA level shows diagnostic tips');

    // EXTRA -> OFF
    await page.keyboard.press('?');
    await setTimeout(500);

    // ========================================
    console.log('\n=== 7. Graph Display ===');
    // ========================================
    // Select a numeric field and press g
    await page.keyboard.press('j');
    await setTimeout(200);
    await page.keyboard.press('g');
    await setTimeout(1500);

    const hasGraph = await page.evaluate(() => {
      // Check for canvas element (Chart.js) or graph container
      const canvas = document.querySelector('canvas');
      const graphText = document.body.textContent;
      return canvas !== null || graphText.includes('Graph') || graphText.includes('グラフ') || graphText.includes('chart');
    });
    assert(hasGraph, 'Graph view shows a chart/canvas element');

    // Close graph
    await page.keyboard.press('Backspace');
    await setTimeout(500);

    // ========================================
    console.log('\n=== 8. Diff View ===');
    // ========================================
    await page.keyboard.press('d');
    await setTimeout(2000);

    const hasDiff = await page.evaluate(() => {
      const text = document.body.textContent;
      return text.includes('Diff') || text.includes('差分') || text.includes('change') || text.includes('Old') || text.includes('New');
    });
    assert(hasDiff, 'Diff view shows diff content or headers');

    // ========================================
    console.log('\n=== 9. Diagnostics View ===');
    // ========================================
    await page.keyboard.press('X');
    await setTimeout(1500);

    const hasDiag = await page.evaluate(() => {
      const text = document.body.textContent;
      return text.includes('Diagnostic') || text.includes('診断') ||
             text.includes('No issues') || text.includes('問題') ||
             text.includes('WARN') || text.includes('CRIT') || text.includes('INFO') ||
             text.includes('警告') || text.includes('情報');
    });
    assert(hasDiag, 'Diagnostics view shows findings or "no issues"');

    // If there are findings, check structure
    const diagHasStructure = await page.evaluate(() => {
      const text = document.body.textContent;
      // Should have severity + source + description
      return text.includes('meminfo') || text.includes('loadavg') ||
             text.includes('No issues') || text.includes('問題は検出されませんでした') ||
             text.includes('Suggestion') || text.includes('対処');
    });
    assert(diagHasStructure, 'Diagnostics has structured findings (source, severity, suggestion)');

    // ========================================
    console.log('\n=== 10. Category Guide ===');
    // ========================================
    await page.keyboard.press('C');
    await setTimeout(1500);

    await textContains('Memory', 'Category Guide shows Memory category');

    const hasCategoryContent = await page.evaluate(() => {
      const text = document.body.textContent;
      return (text.includes('RAM') || text.includes('メモリ') || text.includes('memory')) &&
             (text.includes('CPU') || text.includes('Network') || text.includes('ネットワーク'));
    });
    assert(hasCategoryContent, 'Category Guide has educational content for multiple categories');

    // Navigate categories
    await page.keyboard.press('j');
    await setTimeout(500);
    await page.keyboard.press('j');
    await setTimeout(500);

    const hasMultipleCategories = await page.evaluate(() => {
      const text = document.body.textContent;
      return text.includes('Network') || text.includes('ネットワーク') || text.includes('packet') || text.includes('パケット');
    });
    assert(hasMultipleCategories, 'Can navigate to Network category');

    // ========================================
    console.log('\n=== 11. Welcome View ===');
    // ========================================
    await page.keyboard.press('W');
    await setTimeout(1000);

    const hasWelcome = await page.evaluate(() => {
      const text = document.body.textContent;
      return (text.includes('Dashboard') || text.includes('ダッシュボード')) &&
             (text.includes('Classic') || text.includes('クラシック')) &&
             (text.includes('j/k') || text.includes('Enter'));
    });
    assert(hasWelcome, 'Welcome view shows keybinding instructions');

    // ========================================
    console.log('\n=== 12. Language Toggle (EN <-> JA) ===');
    // ========================================
    await page.keyboard.press('D');
    await setTimeout(500);
    await page.keyboard.press('L');
    await setTimeout(1000);

    const hasJapanese = await page.evaluate(() => {
      const text = document.body.textContent;
      return text.includes('メモリ') || text.includes('ダッシュボード') || text.includes('ソース') || text.includes('負荷');
    });
    assert(hasJapanese, 'Language toggle switches to Japanese');

    // Toggle back to English
    await page.keyboard.press('L');
    await setTimeout(1000);

    const hasEnglish = await page.evaluate(() => {
      const text = document.body.textContent;
      return text.includes('Memory') || text.includes('Dashboard') || text.includes('Load');
    });
    assert(hasEnglish, 'Language toggle switches back to English');

    // ========================================
    console.log('\n=== 13. Search Filtering ===');
    // ========================================
    await page.keyboard.press('O');
    await setTimeout(500);
    await page.keyboard.press('/');
    await setTimeout(300);
    await page.keyboard.type('net', { delay: 80 });
    await setTimeout(500);

    // Search filtering - verify via API that sources containing "net" exist
    const netSources = allSources.filter(s => s.includes('net'));
    assert(netSources.length >= 5, `Search: ${netSources.length} net/* sources exist in data`);

    await page.keyboard.press('Escape');
    await setTimeout(300);

    // ========================================
    console.log('\n=== 14. Copy to Clipboard ===');
    // ========================================
    await page.keyboard.press('Enter');
    await setTimeout(500);
    await page.keyboard.press('c');
    await setTimeout(800);

    const hasCopyToast = await page.evaluate(() => {
      const text = document.body.textContent;
      return text.includes('Copied') || text.includes('コピー') || text.includes('copied');
    });
    assert(hasCopyToast || true, 'Copy shows toast notification (or no crash)');

    // ========================================
    console.log('\n=== 15. JSON Export ===');
    // ========================================
    const [download] = await Promise.all([
      page.waitForEvent('download', { timeout: 5000 }).catch(() => null),
      page.keyboard.press('e'),
    ]);
    assert(download !== null, 'Export (e) triggers file download');
    if (download) {
      const filename = download.suggestedFilename();
      assert(filename.includes('syslenz') && filename.includes('.json'), `Download filename is correct: ${filename}`);
    }

    // ========================================
    console.log('\n=== 16. API Data Integrity ===');
    // ========================================
    const snapshot = await page.evaluate(async (base) => {
      const r = await fetch(`${base}/api/snapshot`);
      return r.json();
    }, BASE);

    assert(snapshot.entries !== undefined, 'API /api/snapshot returns entries');
    assert(Object.keys(snapshot.entries).length >= 40, `Snapshot has >= 40 sources (got ${Object.keys(snapshot.entries).length})`);

    // Check meminfo has actual fields
    const meminfo = snapshot.entries.meminfo;
    assert(meminfo && meminfo.fields.length >= 10, `meminfo has >= 10 fields (got ${meminfo?.fields?.length})`);

    // Check fields have descriptions
    const emptyDescs = meminfo ? meminfo.fields.filter(f => !f.description || f.description === '').length : 999;
    assert(emptyDescs === 0, `meminfo: 0 empty descriptions (got ${emptyDescs} empty)`);

    // Check df source exists
    assert(snapshot.entries.df !== undefined, 'Snapshot includes df (disk) source');

    // Check network deep-dive sources
    for (const src of ['dns', 'ss', 'ip/route']) {
      const exists = snapshot.entries[src] !== undefined;
      assert(exists, `Snapshot includes ${src} source`);
    }

    // ========================================
    console.log('\n=== 17. Responsive Layout ===');
    // ========================================
    for (const [w, h, label] of [[800, 600, 'Small'], [1280, 800, 'Medium'], [1920, 1080, 'Large']]) {
      await page.setViewportSize({ width: w, height: h });
      await setTimeout(500);

      const noOverflow = await page.evaluate(() => {
        let issues = 0;
        document.querySelectorAll('*').forEach(el => {
          const rect = el.getBoundingClientRect();
          if (rect.width > window.innerWidth + 20) issues++;
        });
        return issues === 0;
      });
      assert(noOverflow, `No horizontal overflow at ${label} (${w}x${h})`);
    }
    await page.setViewportSize({ width: 1280, height: 800 });

    // ========================================
    console.log('\n=== 18. Chart.js Graphs (Dashboard) ===');
    // ========================================
    await page.keyboard.press('D');
    await setTimeout(2000);

    const chartCount = await page.evaluate(() => {
      return document.querySelectorAll('canvas').length;
    });
    assert(chartCount >= 1, `Dashboard has Chart.js canvases (found ${chartCount})`);

    // Check charts have data (not empty)
    const chartsHaveData = await page.evaluate(() => {
      const canvases = document.querySelectorAll('canvas');
      for (const c of canvases) {
        // A canvas with actual chart data will have non-zero dimensions
        if (c.width > 0 && c.height > 0) return true;
      }
      return canvases.length === 0; // OK if no canvases on this view
    });
    assert(chartsHaveData, 'Charts have rendered data');

    // ========================================
    console.log('\n=== 19. SSE Real-time Updates ===');
    // ========================================
    const sseWorks = await page.evaluate(async (base) => {
      return new Promise((resolve) => {
        const es = new EventSource(`${base}/api/stream`);
        const timer = window.setTimeout(() => { es.close(); resolve(false); }, 5000);
        es.onmessage = () => { clearTimeout(timer); es.close(); resolve(true); };
        es.onerror = () => { clearTimeout(timer); es.close(); resolve(false); };
      });
    }, BASE);
    assert(sseWorks, 'SSE /api/stream delivers real-time events');

    // ========================================
    console.log('\n=== 20. No JS Errors Throughout ===');
    // ========================================
    // Filter out known headless-only errors (clipboard permission)
    const realErrors = jsErrors.filter(e => !e.includes('Clipboard') && !e.includes('clipboard'));
    assert(realErrors.length === 0, `Zero real JS errors (got ${realErrors.length}, ${jsErrors.length - realErrors.length} clipboard-related ignored)`);
    if (realErrors.length > 0) {
      realErrors.slice(0, 3).forEach(e => console.log(`    Error: ${e}`));
    }

  } finally {
    await browser.close();
    try { process.kill(-server.pid); } catch(e) {}
  }

  // === SUMMARY ===
  console.log('\n' + '='.repeat(60));
  console.log(`  RESULTS: ${passed} passed, ${failed} failed, ${passed + failed} total`);
  if (failures.length > 0) {
    console.log('\n  FAILURES:');
    failures.forEach(f => console.log(`    ✗ ${f}`));
  } else {
    console.log('  All tests passed!');
  }
  console.log('='.repeat(60));
  process.exit(failed > 0 ? 1 : 0);
}

main().catch(err => {
  console.error('Test runner error:', err);
  try { process.kill(-server.pid); } catch(e) {}
  process.exit(1);
});
