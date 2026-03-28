// Web UI demo recording with Playwright
// Usage: node docs/record-web.mjs
// Requires: npm install playwright && npx playwright install chromium
// Output: docs/assets/web-demo.webm (video) + docs/assets/web-*.png (screenshots)

import { chromium } from 'playwright';
import { execSync, spawn } from 'child_process';
import { setTimeout } from 'timers/promises';

const PORT = 3456;
const BASE = `http://localhost:${PORT}`;
const ASSETS = 'docs/assets';

// Human-like delays
const PAUSE_SHORT = 800;
const PAUSE_MED = 1500;
const PAUSE_LONG = 2500;
const PAUSE_READ = 3500;
const KEY_INTERVAL = 600;

async function pressSlowly(page, key, pause = KEY_INTERVAL) {
  await page.keyboard.press(key);
  await setTimeout(pause);
}

async function scrollDown(page, times = 3) {
  for (let i = 0; i < times; i++) {
    await page.keyboard.press('j');
    await setTimeout(KEY_INTERVAL);
  }
}

async function main() {
  // Build and start web server
  console.log('Building syslenz with web feature...');
  execSync('source ~/.cargo/env && cargo build --release --features web', {
    shell: '/bin/bash', stdio: 'inherit'
  });

  console.log(`Starting web server on port ${PORT}...`);
  const server = spawn('bash', ['-c', `source ~/.cargo/env && target/release/syslenz --web ${PORT}`], {
    stdio: 'ignore', detached: true
  });

  await setTimeout(2500);

  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext({
    viewport: { width: 1280, height: 800 },
    recordVideo: { dir: ASSETS, size: { width: 1280, height: 800 } }
  });
  const page = await context.newPage();

  try {
    // --- Scene 1: Dashboard ---
    console.log('Opening Web UI...');
    await page.goto(BASE);
    await setTimeout(PAUSE_READ);
    await page.screenshot({ path: `${ASSETS}/web-dashboard.png` });
    console.log('  [1/8] Dashboard');

    // Let it auto-refresh a couple times
    await setTimeout(PAUSE_LONG);

    // --- Scene 2: Classic mode ---
    await pressSlowly(page, 'o', PAUSE_LONG);
    await page.screenshot({ path: `${ASSETS}/web-classic.png` });
    console.log('  [2/8] Classic mode');

    // Browse sources slowly
    await setTimeout(PAUSE_MED);
    await scrollDown(page, 5);
    await setTimeout(PAUSE_SHORT);

    // --- Scene 3: Drill into detail ---
    await pressSlowly(page, 'Enter', PAUSE_LONG);
    await page.screenshot({ path: `${ASSETS}/web-detail.png` });
    console.log('  [3/8] Detail view');

    // Scroll fields slowly
    await setTimeout(PAUSE_MED);
    await scrollDown(page, 4);
    await setTimeout(PAUSE_SHORT);

    // --- Scene 4: Help levels ---
    await pressSlowly(page, '?', PAUSE_MED);   // NORMAL
    await setTimeout(PAUSE_MED);
    await pressSlowly(page, '?', PAUSE_MED);   // DETAILED
    await setTimeout(PAUSE_LONG);
    await pressSlowly(page, '?', PAUSE_READ);  // EXTRA - read it
    await page.screenshot({ path: `${ASSETS}/web-help.png` });
    console.log('  [4/8] Help EXTRA');
    await pressSlowly(page, '?', PAUSE_SHORT); // OFF

    // --- Scene 5: Back and Diff ---
    await pressSlowly(page, 'Backspace', PAUSE_SHORT);
    await pressSlowly(page, 'Backspace', PAUSE_MED);
    await pressSlowly(page, 'd', PAUSE_READ);
    await page.screenshot({ path: `${ASSETS}/web-diff.png` });
    console.log('  [5/8] Diff view');

    // --- Scene 6: Diagnostics ---
    await pressSlowly(page, 'x', PAUSE_READ);
    await page.screenshot({ path: `${ASSETS}/web-diagnostics.png` });
    console.log('  [6/8] Diagnostics');

    // --- Scene 7: Category Guide ---
    await pressSlowly(page, 'c', PAUSE_LONG);
    await scrollDown(page, 3);
    await setTimeout(PAUSE_LONG);
    await page.screenshot({ path: `${ASSETS}/web-category.png` });
    console.log('  [7/8] Category Guide');

    // --- Scene 8: Switch to Japanese ---
    await pressSlowly(page, 'l', PAUSE_LONG);
    await pressSlowly(page, 'd', PAUSE_READ);
    await page.screenshot({ path: `${ASSETS}/web-dashboard-ja.png` });
    console.log('  [8/8] Japanese Dashboard');

    // Linger on final shot
    await setTimeout(PAUSE_LONG);

  } finally {
    await context.close();
    await browser.close();
    try { process.kill(-server.pid); } catch(e) {}

    console.log('');
    console.log('Done! Converting to GIF...');
  }

  // Find the webm and convert
  const { readdirSync } = await import('fs');
  const webm = readdirSync(ASSETS).find(f => f.endsWith('.webm') && f !== 'web-demo.webm');
  if (webm) {
    const src = `${ASSETS}/${webm}`;
    execSync(`mv "${src}" ${ASSETS}/web-demo.webm`);
  }
  execSync(`ffmpeg -i ${ASSETS}/web-demo.webm -vf "fps=8,scale=1280:-1" -loop 0 ${ASSETS}/web-demo.gif -y`, {
    shell: '/bin/bash', stdio: 'inherit'
  });

  console.log('');
  console.log('Files:');
  console.log(`  ${ASSETS}/web-demo.gif`);
  console.log(`  ${ASSETS}/web-demo.webm`);
  console.log(`  ${ASSETS}/web-*.png (8 screenshots)`);
}

main().catch(console.error);
