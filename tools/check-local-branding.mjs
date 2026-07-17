import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import process from "node:process";

const root = resolve(import.meta.dirname, "..");
const expectedImages = new Map([
  [
    "assets/images/sponsor-alipay.jpg",
    "8e50166194d3e78953248b94506737156767bbfb9059d82736d04f1c5827afa2",
  ],
  [
    "assets/images/sponsor-wechat.jpg",
    "37c111fad288fc98f056ce3489eb5b29d689790f9a94ead5fdb96fda75a66d86",
  ],
]);
const readmes = ["README.md", "README_EN.md"];
const forbiddenReadmeMarkers = [
  "## 赞助商",
  "## Sponsors",
  "Want to be shown below?",
  "想显示在下方？",
  "jojocode.com",
  "aigocode.com/invite/CodexPlusPlus",
  "apikey.fun/register?aff=CODEX",
];
const expectedReadmeImages = [
  "assets/images/sponsor-alipay.jpg",
  "assets/images/sponsor-wechat.jpg",
];

const failures = [];
for (const readme of readmes) {
  const text = readFileSync(resolve(root, readme), "utf8");
  for (const marker of forbiddenReadmeMarkers) {
    if (text.includes(marker)) {
      failures.push(`${readme} contains forbidden upstream sponsor marker: ${marker}`);
    }
  }
  for (const image of expectedReadmeImages) {
    if (!text.includes(image)) {
      failures.push(`${readme} does not reference owner donation image: ${image}`);
    }
  }
}

for (const [image, expectedHash] of expectedImages) {
  const bytes = readFileSync(resolve(root, image));
  const actualHash = createHash("sha256").update(bytes).digest("hex");
  if (actualHash !== expectedHash) {
    failures.push(`${image} SHA-256 changed from the owner-approved image`);
  }
}

if (failures.length > 0) {
  for (const failure of failures) {
    console.error(`branding guard: ${failure}`);
  }
  process.exit(1);
}

console.log("Local README and donation branding guard passed.");
