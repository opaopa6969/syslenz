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

  // Wait for server to start
  await setTimeout(2000);

  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext({
    viewport: { width: 1280, height: 800 },
    recordVideo: { dir: ASSETS, size: { width: 1280, height: 800 } }
  });
  const page = await context.newPage();

  try {
    console.log('Opening Web UI...');
    await page.goto(BASE);
    await setTimeout(2000);

    // Screenshot: Dashboard (default)
    await page.screenshot({ path: `${ASSETS}/web-dashboard.png` });
    console.log('  [1/8] Dashboard screenshot');
    await setTimeout(1500);

    // Navigate to Classic mode (press O)
    await page.keyboard.press('o');
    await setTimeout(1500);
    await page.screenshot({ path: `${ASSETS}/web-classic.png` });
    console.log('  [2/8] Classic mode screenshot');

    // Navigate down in sidebar
    await page.keyboard.press('j');
    await setTimeout(300);
    await page.keyboard.press('j');
    await setTimeout(300);
    await page.keyboard.press('j');
    await setTimeout(300);
    await page.keyboard.press('j');
    await setTimeout(300);
    await page.keyboard.press('j');
    await setTimeout(500);

    // Drill into detail
    await page.keyboard.press('Enter');
    await setTimeout(1500);
    await page.screenshot({ path: `${ASSETS}/web-detail.png` });
    console.log('  [3/8] Detail view screenshot');

    // Show help (?)
    await page.keyboard.press('?');
    await setTimeout(1000);
    await page.keyboard.press('?');
    await setTimeout(1000);
    await page.keyboard.press('?');
    await setTimeout(1500);
    await page.screenshot({ path: `${ASSETS}/web-help.png` });
    console.log('  [4/8] Help EXTRA screenshot');
    await page.keyboard.press('?');
    await setTimeout(500);

    // Diff view
    await page.keyboard.press('d');
    await setTimeout(2000);
    await page.screenshot({ path: `${ASSETS}/web-diff.png` });
    console.log('  [5/8] Diff view screenshot');

    // Diagnostics
    await page.keyboard.press('x');
    await setTimeout(1500);
    await page.screenshot({ path: `${ASSETS}/web-diagnostics.png` });
    console.log('  [6/8] Diagnostics screenshot');

    // Category Guide
    await page.keyboard.press('c');
    await setTimeout(1500);
    await page.screenshot({ path: `${ASSETS}/web-category.png` });
    console.log('  [7/8] Category Guide screenshot');

    // Switch to Japanese
    await page.keyboard.press('l');
    await setTimeout(1500);

    // Back to Dashboard
    await page.keyboard.press('d');
    await setTimeout(2000);
    await page.screenshot({ path: `${ASSETS}/web-dashboard-ja.png` });
    console.log('  [8/8] Japanese Dashboard screenshot');

    await setTimeout(1000);

  } finally {
    await context.close();
    await browser.close();

    // Stop server
    try { process.kill(-server.pid); } catch(e) {}

    console.log('');
    console.log('Done! Files saved:');
    console.log(`  Video: ${ASSETS}/web-demo.webm`);
    console.log(`  Screenshots: ${ASSETS}/web-*.png`);
    console.log('');
    console.log('To convert video to GIF:');
    console.log(`  ffmpeg -i ${ASSETS}/*.webm -vf "fps=10,scale=1280:-1" -loop 0 ${ASSETS}/web-demo.gif`);
  }
}

main().catch(console.error);
