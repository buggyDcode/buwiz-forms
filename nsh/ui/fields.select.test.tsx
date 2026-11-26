// What the number boxes do with the clear gesture and with typing, headless: emptying a box reaches the page as one edit and reports
// "" for an empty box, and typing then starts from nothing. Selections and a real caret exist only in a real window: those cases
// (Cmd+A, a selected range, a click's caret, Tab) are in the shots lane, `just nsh-shots select` (shots/select.test.tsx).
import { expect, open, test } from "nsh/test";
import type { App } from "nsh/test";
import { readout } from "./harness";

const WIDE = { width: 800, height: 600, platform: "macos" as const };

const read = (app: App, key: string) => readout(app, key);

test("the clear gesture empties every number box and reports empty; typing then starts from nothing", async () => {
  const app = await open("./select.fixture.tsx", WIDE);
  for (const [id, key, typed, shown, reported] of [
    ["s-some", "Some:", "250", "250", "Some: 250"],
    ["s-zero", "Zero:", "75", "75", "Zero: 75"],
    ["s-rate", "Rate:", "7.5", "7.5", "Rate: 7.5"],
    ["s-count", "Count:", "93", "93", "Count: 93"],
    ["s-tin", "Tin:", "0001", "000-1", "Tin: 0001"],
  ] as const) {
    await app.clear(`id:${id}`);
    expect(app.value(`id:${id}`)).toBe("");
    expect(read(app, key)).toBe(`${key} empty`);
    await app.type(`id:${id}`, typed);
    expect(app.value(`id:${id}`)).toBe(shown);
    expect(read(app, key)).toBe(reported);
  }
}, 120_000);


test("zero percent and whole-number boxes are empty when focused, so 4 then 2 saves 42", async () => {
  const app = await open("./select.fixture.tsx", WIDE);
  for (const [id, key] of [
    ["s-rate", "Rate:"],
    ["s-count", "Count:"],
  ] as const) {
    await app.type(`id:${id}`, ""); // focus the input; the headless harness has no pointer click on inputs
    await app.waitUntil(() => app.value(`id:${id}`) === "");
    await app.type(`id:${id}`, "4");
    expect(app.value(`id:${id}`)).toBe("4");
    await app.type(`id:${id}`, "2");
    expect(app.value(`id:${id}`)).toBe("42");
    expect(read(app, key)).toBe(`${key} 42`);
  }
});

test("zero percent and whole-number boxes are empty on Tab entry, so 4 then 2 saves 42", async () => {
  const app = await open("./select.fixture.tsx", WIDE);
  for (const [before, id, key] of [
    ["s-zero", "s-rate", "Rate:"],
    ["s-rate", "s-count", "Count:"],
  ] as const) {
    await app.type(`id:${before}`, ""); // focus the preceding box; the headless harness has no pointer click on inputs
    await app.keyboard.press("tab");
    await app.waitUntil(() => app.value(`id:${id}`) === "");
    await app.keyboard.press("4");
    expect(app.value(`id:${id}`)).toBe("4");
    await app.keyboard.press("2");
    expect(app.value(`id:${id}`)).toBe("42");
    expect(read(app, key)).toBe(`${key} 42`);
  }
});

test("input in the zero focus's own turn wins over the empty write and reads without its stale zero prefix", async () => {
  const app = await open("./select.fixture.tsx", { ...WIDE, clock: {} });
  await app.press("id:s-zero-same");
  await app.waitUntil(() => readout(app, "Zero:") === "Zero: 15");
  // The filer's text wins: when the deferred write is due, editing has already superseded it.
  await app.clock?.runFor(5);
  expect(readout(app, "Zero:")).toBe("Zero: 15");
  expect(app.value("id:s-zero")).toBe("0.0015");
});

test("input before the focus callback reads without the stale zero in money, percent and count boxes", async () => {
  const app = await open("./select.fixture.tsx", WIDE);
  for (const [button, key] of [
    ["s-zero-input-first", "Zero:"],
    ["s-rate-input-first", "Rate:"],
    ["s-count-input-first", "Count:"],
  ] as const) {
    await app.press(`id:${button}`);
    if (key === "Zero:") expect(app.value("id:s-zero")).toBe("0.0015");
    await app.waitUntil(() => readout(app, key) === `${key} 15`);
  }
  await app.type("id:s-free", "x");
  expect(app.value("id:s-zero")).toBe("15.00");
  expect(readout(app, "Zero:")).toBe("Zero: 15");
});

test("after the zero box is empty, a typed 0.005 keeps its own leading zeroes", async () => {
  const app = await open("./select.fixture.tsx", WIDE);
  await app.type("id:s-zero", "");
  await app.waitUntil(() => app.value("id:s-zero") === "");
  await app.type("id:s-zero", "0.005");
  expect(readout(app, "Zero:")).toBe("Zero: 0.005");
});

test("a percent typed over a zero stays shown after leaving, matching what it reported", async () => {
  const app = await open("./select.fixture.tsx", WIDE);
  await app.type("id:s-rate", "");
  await app.waitUntil(() => app.value("id:s-rate") === "");
  await app.type("id:s-rate", "42");
  await app.type("id:s-free", "x");
  await app.wait(100);
  expect(readout(app, "Rate:")).toBe("Rate: 42");
  expect(app.value("id:s-rate")).toBe("42");
});

test("V2: percent and whole-number input in the zero focus's own turn read without the stale zero", async () => {
  const app = await open("./select.fixture.tsx", { ...WIDE, clock: {} });
  await app.press("id:s-rate-same");
  await app.waitUntil(() => readout(app, "Rate:") === "Rate: 15");
  expect(app.value("id:s-rate")).toBe("015");
  await app.press("id:s-count-same");
  await app.waitUntil(() => readout(app, "Count:") === "Count: 15");
  expect(app.value("id:s-count")).toBe("015");
});

test("V2: after the first input, genuine zeroes and points are the filer's own text", async () => {
  const app = await open("./select.fixture.tsx", { ...WIDE, clock: {} });
  await app.press("id:s-zero-same");
  await app.waitUntil(() => readout(app, "Zero:") === "Zero: 15");
  await app.keyboard.press("0");
  await app.keyboard.press(".");
  await app.keyboard.press("0");
  await app.keyboard.press("0");
  await app.keyboard.press("1");
  await app.keyboard.press("2");
  expect(readout(app, "Zero:")).toBe("Zero: 150.0012");
});
