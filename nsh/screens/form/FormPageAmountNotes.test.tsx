// Amount notes on the real form page (FormPageAny.tsx, the demo taxpayer, a scratch database): after a box is left it shows what
// the return holds, and when that is not what was typed a note under it says so. Nothing is held: Save, autosave and leaving go
// on; Queue stops on a note on an open part, the way it stops on any message under a box.
import { expect, open, test } from "nsh/test";

const ENTRY = "./FormPageAny.tsx";
const DESKTOP = { width: 1440, height: 900, platform: "macos" as const };
const BOX = "id:fp-frm2551Qv2018:txt15";
const PROBE = "frm2551Qv2018:txt15";

/** The page on 2551Q (year, quarter), Part II open (Items 14 to 28), the stored txt15 published as `probe <value>`. */
async function page(year: number, period: number) {
  const app = await open(ENTRY, { ...DESKTOP, host: { params: { formCode: "2551Q", year, period, probeKey: PROBE } } });
  await app.waitFor("text:BIR Form 2551Q");
  await app.press("id:fp-nav-2");
  await app.waitFor(BOX);
  return app;
}

const probe = (app: Awaited<ReturnType<typeof page>>) => (app.texts().find((t) => t.startsWith("probe ")) ?? "").slice("probe ".length);

test("a 20-digit amount: once the box is left it shows what the return holds, a note names both, and the save goes through", async () => {
  const app = await page(2025, 1);
  await app.clear(BOX);
  await app.type(BOX, "12345678901234567890");
  await app.submit(BOX);
  // The host refuses it (the official field has 12 digits): the return keeps 0.00, and the save goes through.
  await app.waitFor("text:✓ Saved", { timeout: 8000 });
  const stored = probe(app);
  expect(stored).toBe("0.00");
  // The box shows the stored amount, never the typed text (nor another number made from it).
  expect(app.value(BOX)).toBe(stored);
  const note = app.texts().find((t) => t.includes("you typed")) ?? "";
  expect(note.startsWith("Item 15: you typed 12,345,678,901,234,567,890.00; the return holds 0.00. ")).toBe(true);
  expect(note).toContain("12-digit");
  expect(app.texts()).not.toContain("Unsaved changes");
}, 30_000);

test("the note survives folding and reopening the part, still true: the box shows the stored amount and nothing is stuck", async () => {
  const app = await page(2025, 2);
  await app.clear(BOX);
  await app.type(BOX, "12345678901234567890");
  await app.submit(BOX);
  await app.waitFor("text:✓ Saved", { timeout: 8000 });
  await app.waitUntil(() => app.texts().some((t) => t.includes("you typed")), { timeout: 5000 });
  const stored = probe(app);
  await app.press("id:fp-part-2-toggle");
  await app.waitUntil(() => !app.shows(BOX), { timeout: 3000 });
  await app.press("id:fp-part-2-toggle");
  await app.waitFor(BOX);
  expect(app.value(BOX)).toBe(stored);
  expect(app.texts().some((t) => t.includes(`the return holds ${stored}`))).toBe(true);
  // Back in the box, the note goes; leaving again without a change raises none.
  await app.type(BOX, "");
  await app.waitUntil(() => !app.texts().some((t) => t.includes("you typed")), { timeout: 3000 });
  await app.submit(BOX);
  await app.wait(600);
  expect(app.texts().some((t) => t.includes("you typed"))).toBe(false);
}, 30_000);

test("valid edits made after a note are saved with it: the save line says Saved and the return holds the other box's amount", async () => {
  // Each headless app has a database of its own, so "on disk" is read through the saved draft here (the fixture's probe of the
  // other box) and through a second session on the same database in session.fixes.test.ts.
  const other = "frm2551Qv2018:txt17";
  const app = await open(ENTRY, { ...DESKTOP, host: { params: { formCode: "2551Q", year: 2025, period: 3, probeKey: other } } });
  await app.waitFor("text:BIR Form 2551Q");
  await app.press("id:fp-nav-2");
  await app.waitFor(BOX);
  await app.clear(BOX);
  await app.type(BOX, "12345678901234567890");
  await app.submit(BOX);
  await app.waitUntil(() => app.texts().some((t) => t.includes("you typed")), { timeout: 8000 });
  await app.clear(`id:fp-${other}`);
  await app.type(`id:fp-${other}`, "250");
  await app.submit(`id:fp-${other}`);
  await app.waitUntil(() => probe(app) === "250.00" && app.texts().includes("✓ Saved"), { timeout: 8000 });
  expect(app.texts()).toContain("saved returns 1");
  expect(app.texts()).not.toContain("Unsaved changes");
  expect(app.value(BOX)).toBe("0.00");
}, 40_000);

test("Queue with the amount box still focused leaves it first and stops on its note; once the box is focused again Queue goes on", async () => {
  const app = await page(2025, 4);
  // The refused-edit path's toast ("6 items need attention."): Queue stopped on the page, before the host's queue check.
  const stops = () => app.texts().filter((x) => /^\d+ items? needs? attention\.$/.test(x)).length;
  await app.clear(BOX);
  await app.type(BOX, "12345678901234567890");
  // No leaving step: Queue itself takes the keys out of the box.
  await app.press("id:fp-queue");
  await app.waitUntil(() => app.texts().some((t) => t.includes("you typed")), { timeout: 5000 });
  await app.waitUntil(() => stops() === 1, { timeout: 3000 });
  expect(app.value(BOX)).toBe(probe(app));
  expect(app.shows("text:Queue this return?")).toBe(false);
  // The filer looks at the box: the note goes, and Queue goes on to the host's checks (never to a send here): no new stop toast,
  // a new toast from the host's check instead.
  await app.type(BOX, "");
  await app.submit(BOX);
  await app.waitUntil(() => !app.texts().some((t) => t.includes("you typed")), { timeout: 3000 });
  const before = app.texts().length;
  await app.press("id:fp-queue");
  await app.wait(1200);
  expect(stops()).toBe(1);
  expect(app.texts().length).toBeGreaterThan(before);
  expect(app.shows("text:Queue this return?")).toBe(false);
}, 40_000);

// Unreadable text: on leaving, the money box puts the return's amount back and calls onLeftUnreadable; the session draws the note.
const unreadable = test;

for (const typed of ["1500x", "1500,50"]) {
  unreadable(`unreadable ${typed}: after leaving, the box shows the old amount, a note names both, the draft holds the old amount, Save works`, async () => {
    const app = await page(2024, typed === "1500x" ? 1 : 2);
    await app.clear(BOX);
    await app.type(BOX, "1000");
    await app.submit(BOX);
    await app.waitFor("text:✓ Saved", { timeout: 8000 });
    await app.clear(BOX);
    await app.type(BOX, typed);
    await app.submit(BOX);
    await app.waitUntil(() => app.texts().some((t) => t.includes("is not an amount")), { timeout: 5000 });
    expect(app.value(BOX)).toBe("1,000.00");
    expect(app.texts()).toContain(`Item 15: ${typed} is not an amount. The box kept 1,000.00.`);
    expect(probe(app)).toBe("1,000.00");
    await app.press("id:fp-save").catch(() => {});
    await app.waitFor("text:✓ Saved", { timeout: 8000 });
  }, 30_000);
}
