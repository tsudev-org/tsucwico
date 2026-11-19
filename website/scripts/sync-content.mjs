// Chép ảnh chụp màn hình (docs/screenshots/) và video giới thiệu (docs/video/) của repo vào public/ trước khi build,
// kèm bản AVIF + WebP của mỗi ảnh PNG (PRODUCT_POLICY mục 5: ảnh AVIF/WebP + PNG dự phòng).
// Nguồn duy nhất là docs/ (README cũng dùng ảnh ở đó); public/anh/ và public/video/ là bản sinh ra, không commit.
import { cpSync, readdirSync, rmSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import sharp from 'sharp';

const web = join(dirname(fileURLToPath(import.meta.url)), '..');

const anh = join(web, 'public/anh');
rmSync(anh, { recursive: true, force: true });
cpSync(join(web, '../docs/screenshots'), anh, { recursive: true });
const png = readdirSync(anh).filter((f) => f.endsWith('.png'));
await Promise.all(
  png.flatMap((f) => {
    const nguon = join(anh, f);
    const goc = nguon.slice(0, -4);
    return [
      sharp(nguon).webp({ quality: 82, effort: 5 }).toFile(`${goc}.webp`),
      sharp(nguon).avif({ quality: 55, effort: 5 }).toFile(`${goc}.avif`),
    ];
  }),
);
console.log(`[sync] docs/screenshots/ -> public/anh/ (${png.length} PNG + AVIF + WebP)`);

const video = join(web, 'public/video');
rmSync(video, { recursive: true, force: true });
cpSync(join(web, '../docs/video'), video, { recursive: true });
const poster = join(video, 'gioi-thieu-poster');
await sharp(`${poster}.png`).webp({ quality: 80 }).toFile(`${poster}.webp`);
console.log('[sync] docs/video/ -> public/video/');
