import { readFileSync } from 'fs';
import { writeFile, mkdir, access } from 'fs/promises';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';
import sharp from 'sharp';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const RUST_FILE_PATH = join(__dirname, '..', 'src-tauri', 'crates', 'olm_core', 'src', 'social_registry.rs');
const OUTPUT_DIR = join(__dirname, '..', 'public', 'social-avatars');

interface AccountImage {
  id: string;
  url: string;
}

function parseRustFile(path: string): AccountImage[] {
  const content = readFileSync(path, 'utf-8');
  const lines = content.split('\n');
  const accounts: AccountImage[] = [];

  let currentId: string | null = null;
  let inProfileImageUrl = false;
  let urlBuffer = '';

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];

    // Extract ID
    const idMatch = line.match(/id:\s*"([^"]+)"\.to_string\(\)/);
    if (idMatch) {
      currentId = idMatch[1];
      continue;
    }

    // Check for profile_image_url: None
    if (line.includes('profile_image_url: None') && currentId) {
      // Skip this account - no image
      currentId = null;
      inProfileImageUrl = false;
      urlBuffer = '';
      continue;
    }

    // Check for start of profile_image_url: Some(
    if (line.includes('profile_image_url: Some(') && currentId) {
      inProfileImageUrl = true;
      urlBuffer = '';
      continue;
    }

    // If we're collecting URL parts
    if (inProfileImageUrl && currentId) {
      // Check for closing paren on same line or just the URL
      const urlMatch = line.match(/"(https:\/\/[^"]+)"/);
      if (urlMatch) {
        urlBuffer += urlMatch[1];
      }

      // Check if the block ends
      if (line.includes('),') || line.includes('.to_string(),')) {
        if (urlBuffer) {
          accounts.push({ id: currentId, url: urlBuffer });
        }
        currentId = null;
        inProfileImageUrl = false;
        urlBuffer = '';
      }
    }

    // If we see a closing brace for SocialAccount, reset state
    if (line.trim() === '},' || line.trim() === '},') {
      if (currentId && inProfileImageUrl && urlBuffer) {
        accounts.push({ id: currentId, url: urlBuffer });
      }
      currentId = null;
      inProfileImageUrl = false;
      urlBuffer = '';
    }
  }

  return accounts;
}

async function fileExists(path: string): Promise<boolean> {
  try {
    await access(path);
    return true;
  } catch {
    return false;
  }
}

async function downloadImage(url: string): Promise<Buffer> {
  const response = await fetch(url, {
    headers: {
      'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36'
    }
  });

  if (!response.ok) {
    throw new Error(`HTTP ${response.status}: ${response.statusText}`);
  }

  const arrayBuffer = await response.arrayBuffer();
  return Buffer.from(arrayBuffer);
}

async function convertToWebp(buffer: Buffer): Promise<Buffer> {
  return sharp(buffer)
    .webp({ quality: 85, effort: 4 })
    .toBuffer();
}

async function main() {
  console.log('Parsing social_registry.rs...');
  const accounts = parseRustFile(RUST_FILE_PATH);
  console.log(`Found ${accounts.length} accounts with profile images`);

  // Ensure output directory exists
  await mkdir(OUTPUT_DIR, { recursive: true });

  let successCount = 0;
  let failCount = 0;
  const failures: string[] = [];

  for (const account of accounts) {
    const outputPath = join(OUTPUT_DIR, `${account.id}.webp`);

    // Check if already exists
    if (await fileExists(outputPath)) {
      console.log(`  [SKIP] ${account.id} - already exists`);
      successCount++;
      continue;
    }

    try {
      console.log(`  [DOWNLOAD] ${account.id} from ${account.url}`);
      const imageBuffer = await downloadImage(account.url);
      const webpBuffer = await convertToWebp(imageBuffer);
      await writeFile(outputPath, webpBuffer);
      console.log(`  [OK] ${account.id} -> ${account.id}.webp`);
      successCount++;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      console.error(`  [FAIL] ${account.id}: ${errorMessage}`);
      failCount++;
      failures.push(`${account.id}: ${errorMessage}`);
    }
  }

  console.log('\n--- Summary ---');
  console.log(`Total accounts: ${accounts.length}`);
  console.log(`Success: ${successCount}`);
  console.log(`Failed: ${failCount}`);

  if (failures.length > 0) {
    console.log('\nFailures:');
    failures.forEach(f => console.log(`  - ${f}`));
  }
}

main().catch(error => {
  console.error('Fatal error:', error);
  process.exit(1);
});