// Text entry at phone and desktop size: plain text, the TIN, money, percent and integer masks, the date, PIN cells.
import { expect, open, test } from "nsh/test";
import { openKit, readout, SIZES } from "./harness";

const INVALID_AMOUNT = "Not a valid amount. Correct it or clear the box; it was not saved.";

for (const size of SIZES) {
  test(`${size.name}: touch date sheet validates Done, discards dismissal and clears`, async () => {
    const app = await openKit("../BasePreview.tsx", size.options);
    expect(app.shows("id:kit-one-date")).toBe(true);
    await app.press("id:kit-one-date");
    await app.waitFor("id:kit-one-date-sheet-input");
    await app.type("id:kit-one-date-sheet-input", "08/17/88");
    await app.press("id:kit-one-date-sheet-done");
    expect(app.shows("id:kit-one-date-sheet-input")).toBe(true);
    expect(app.texts()).toContain("Type the four-digit year, for example 08/17/1988.");
    await app.clear("id:kit-one-date-sheet-input");
    await app.type("id:kit-one-date-sheet-input", "08171988");
    await app.press("id:kit-one-date-sheet-done");
    await app.waitFor("id:kit-one-date-sheet-input", { state: "gone" });
    expect(app.texts()).toContain("August 17, 1988");
    await app.press("id:kit-one-date");
    expect(app.value("id:kit-one-date-sheet-input")).toBe("08/17/1988");
    await app.clear("id:kit-one-date-sheet-input");
    await app.type("id:kit-one-date-sheet-input", "01/01/2000");
    await app.press("role:button[Close]");
    await app.waitFor("id:kit-one-date-sheet-input", { state: "gone" });
    expect(app.texts()).toContain("August 17, 1988");
    await app.press("id:kit-one-date");
    await app.press("id:kit-one-date-sheet-clear");
    await app.waitFor("id:kit-one-date-sheet-input", { state: "gone" });
    expect(app.texts()).not.toContain("August 17, 1988");
  });
}

for (const size of SIZES) {
  test(`${size.name}: text field types, TIN groups while typing`, async () => {
    const app = await openKit("../BasePreview.tsx", size.options);
    await app.clear("id:kit-name");
    await app.type("id:kit-name", "Juana Santos");
    expect(app.value("id:kit-name")).toBe("Juana Santos");
    await app.clear("id:kit-tin");
    expect(readout(app, "Digits:")).toBe("Digits: ");
    await app.type("id:kit-tin", "0001");
    expect(app.value("id:kit-tin")).toBe("000-1");
    await app.type("id:kit-tin", "23453");
    expect(app.value("id:kit-tin")).toBe("000-123-453");
    await app.type("id:kit-tin", "00000999");
    expect(app.value("id:kit-tin")).toBe("000-123-453-00000");
    expect(readout(app, "Digits:")).toBe("Digits: 00012345300000");
    await app.keyboard.press("backspace");
    expect(app.value("id:kit-tin")).toBe("000-123-453-0000");
  });

  test(`${size.name}: TIN paste with dashes keeps every digit`, async () => {
    const app = await openKit("../BasePreview.tsx", size.options);
    await app.clear("id:kit-tin");
    await app.paste("id:kit-tin", { text: "000-123-453-00000" });
    expect(app.value("id:kit-tin")).toBe("000-123-453-00000");
    expect(readout(app, "Digits:")).toBe("Digits: 00012345300000");
  });

  test(`${size.name}: money stays plain while typing, plain value out, two decimals on leaving`, async () => {
    const app = await openKit("../BasePreview.tsx", size.options);
    expect(app.value("id:kit-money")).toBe("1,250,000.00");
    await app.clear("id:kit-money");
    expect(readout(app, "Value:")).toBe("Value: empty");
    await app.type("id:kit-money", "1250000");
    expect(app.value("id:kit-money")).toBe("1250000");
    expect(readout(app, "Value:")).toBe("Value: 1250000");
    // Real key events are separate turns; type the refused letter and the keys around it one at a time.
    await app.type("id:kit-money", ".5");
    await app.type("id:kit-money", "x");
    await app.type("id:kit-money", ".");
    await app.type("id:kit-money", "7");
    expect(app.value("id:kit-money")).toBe("1250000.5x.7");
    expect(readout(app, "Value:")).toBe("Value: 1250000");
    await app.keyboard.press("backspace");
    await app.keyboard.press("backspace");
    await app.keyboard.press("backspace");
    expect(app.value("id:kit-money")).toBe("1250000.5");
    expect(readout(app, "Value:")).toBe("Value: 1250000.5");
    // Leaving the box: another field takes the keys.
    await app.type("id:kit-name", " ");
    expect(app.value("id:kit-money")).toBe("1,250,000.50");
    expect(readout(app, "Value:")).toBe("Value: 1250000.5");
    await app.clear("id:kit-money");
    await app.paste("id:kit-money", { text: "₱ 2,500.75" });
    expect(readout(app, "Value:")).toBe("Value: 2500.75");
  });

  test(`${size.name}: percent, integer, date, PIN`, async () => {
    const app = await openKit("../BasePreview.tsx", size.options);
    await app.clear("id:kit-pct");
    await app.type("id:kit-pct", "12.5.0a");
    expect(app.value("id:kit-pct")).toBe("12.50");
    await app.clear("id:kit-int");
    await app.type("id:kit-int", "0123456");
    expect(app.value("id:kit-int")).toBe("0123");
    await app.clear("id:kit-date");
    await app.type("id:kit-date", "12102020");
    expect(readout(app, "Stored:")).toBe("Stored: 12/10/2020");
    await app.type("id:kit-name", " ");
    expect(app.value("id:kit-date")).toBe("December 10, 2020");
    await app.clear("id:kit-date");
    await app.type("id:kit-date", "13/45");
    expect(app.texts().some((t) => t.startsWith("Type the date as MM/DD/YYYY"))).toBe(false);
    await app.type("id:kit-name", " ");
    expect(app.texts().some((t) => t.startsWith("Type the date as MM/DD/YYYY"))).toBe(true);
    await app.type("id:kit-code", "123456");
    expect(readout(app, "PIN digits:")).toBe("PIN digits: 0 | Code complete: 123456");
    await app.type("id:kit-pin", "4321");
    expect(readout(app, "PIN digits:")).toBe("PIN digits: 4 | Code complete: 123456");
  });

  test(`${size.name}: a date box is MM/DD/YYYY while typed, the full month when left, and its calendar is named for it`, async () => {
    const app = await openKit("../BasePreview.tsx", size.options);
    // The calendar trigger is named for the box, not a fixed "Open the calendar"
    // (the picker's trigger is a combobox to assistive tech).
    expect(app.shows("role:combobox[Pick Date of birth]")).toBe(true);
    await app.clear("id:kit-date");
    // Eight digits rewrite to MM/DD/YYYY while the box has the keys.
    await app.type("id:kit-date", "12102020");
    expect(app.value("id:kit-date")).toBe("12/10/2020");
    expect(readout(app, "Stored:")).toBe("Stored: 12/10/2020");
    // Other shapes stay as typed while typing and take the long form on blur.
    await app.clear("id:kit-date");
    await app.type("id:kit-date", "12-10-2020");
    expect(app.value("id:kit-date")).toBe("12-10-2020");
    await app.type("id:kit-name", " ");
    expect(app.value("id:kit-date")).toBe("December 10, 2020");
    await app.clear("id:kit-date");
    await app.type("id:kit-date", "Dec 10 2020");
    await app.type("id:kit-name", " ");
    expect(app.value("id:kit-date")).toBe("December 10, 2020");
    // Focusing the long text shows the numeric date (an empty type takes the keys).
    await app.type("id:kit-date", "");
    await app.wait(200);
    expect(app.value("id:kit-date")).toBe("12/10/2020");
  });
}

// The masked boxes against a fixture of their own (masked.fixture.tsx): what a box shows is what it reports.
for (const size of SIZES) {
  test(`${size.name}: money rounds on the digits when the box is left, and the draft takes the rounded value`, async () => {
    const app = await open("./masked.fixture.tsx", size.options);
    await app.type("id:m-amount", "1.255");
    expect(app.value("id:m-amount")).toBe("1.255");
    expect(readout(app, "Amount:")).toBe("Amount: 1.255");
    await app.type("id:m-other", " "); // leaves the amount
    expect(app.value("id:m-amount")).toBe("1.26"); // toFixed showed 1.25 while the draft kept 1.255
    expect(readout(app, "Amount:")).toBe("Amount: 1.26");
    await app.clear("id:m-amount");
    await app.type("id:m-amount", "2.675");
    await app.type("id:m-other", " ");
    expect(app.value("id:m-amount")).toBe("2.68");
    expect(readout(app, "Amount:")).toBe("Amount: 2.68");
    await app.clear("id:m-amount");
    await app.type("id:m-amount", "1.005");
    await app.type("id:m-other", " ");
    expect(app.value("id:m-amount")).toBe("1.01");
    expect(readout(app, "Amount:")).toBe("Amount: 1.01");
  });

  test(`${size.name}: an invalid key stays visible and the report rolls back to the value at focus`, async () => {
    const app = await open("./masked.fixture.tsx", size.options);
    await app.type("id:m-amount", "1250000.5");
    expect(readout(app, "Amount:")).toBe("Amount: 1250000.5");
    await app.type("id:m-amount", "x");
    expect(app.value("id:m-amount")).toBe("1250000.5x"); // invalid text stays exactly as typed
    expect(readout(app, "Amount:")).toBe("Amount: empty"); // the valid prefix is rolled back, not saved
    expect(app.texts()).not.toContain(INVALID_AMOUNT);
    await app.type("id:m-other", " ");
    expect(app.value("id:m-amount")).toBe("1250000.5x");
    expect(readout(app, "Amount:")).toBe("Amount: empty");
    expect(app.texts()).toContain(INVALID_AMOUNT);
  });

  test(`${size.name}: a negative is shown negative; an unallowed minus stays visible, invalid and unreported`, async () => {
    const app = await open("./masked.fixture.tsx", size.options);
    expect(app.value("id:m-stored")).toBe("-250.00"); // a stored negative is shown negative even where no minus may be typed
    await app.type("id:m-stored", "1");
    expect(app.value("id:m-stored")).toBe("-250.001");
    expect(app.texts()).not.toContain(INVALID_AMOUNT);
    expect(readout(app, "Stored:")).toBe("Stored: -250");
    await app.type("id:m-other", " ");
    expect(app.value("id:m-stored")).toBe("-250.001");
    expect(readout(app, "Stored:")).toBe("Stored: -250");
    expect(app.texts()).toContain(INVALID_AMOUNT);
    // Typed minus where it is not allowed stays visible; the whole text is invalid and nothing is reported.
    await app.type("id:m-amount", "-");
    await app.type("id:m-amount", "5");
    expect(app.value("id:m-amount")).toBe("-5");
    expect(readout(app, "Amount:")).toBe("Amount: empty");
    // Pasted negatives also stay exactly visible, invalid and unreported.
    await app.clear("id:m-amount");
    await app.paste("id:m-amount", { text: "-1,234.00" });
    expect(app.value("id:m-amount")).toBe("-1,234.00");
    await app.clear("id:m-amount");
    await app.paste("id:m-amount", { text: "(1,234.00)" });
    expect(app.value("id:m-amount")).toBe("(1,234.00)");
    expect(readout(app, "Amount:")).toBe("Amount: empty");
    // Where negatives are allowed they are taken, brackets too.
    await app.type("id:m-free", "-");
    await app.type("id:m-free", "5");
    expect(app.value("id:m-free")).toBe("-5");
    expect(readout(app, "Free:")).toBe("Free: -5");
    await app.clear("id:m-free");
    await app.paste("id:m-free", { text: "(1,234.00)" });
    expect(app.value("id:m-free")).toBe("(1,234.00)");
    expect(readout(app, "Free:")).toBe("Free: empty");
  });

  test(`${size.name}: pasted text is strict: valid grouped amounts report, ambiguous text stays visible invalid and unreported`, async () => {
    const app = await open("./masked.fixture.tsx", size.options);
    await app.paste("id:m-amount", { text: "1,234,567" });
    expect(app.value("id:m-amount")).toBe("1,234,567");
    expect(readout(app, "Amount:")).toBe("Amount: 1234567");
    await app.clear("id:m-amount");
    await app.paste("id:m-amount", { text: "1,234.56" });
    expect(app.value("id:m-amount")).toBe("1,234.56");
    expect(readout(app, "Amount:")).toBe("Amount: 1234.56");
    for (const bad of ["1.234.567", "1.234,56", "1.2.3"]) {
      await app.clear("id:m-amount");
      await app.paste("id:m-amount", { text: bad });
      expect(app.value("id:m-amount")).toBe(bad);
      expect(readout(app, "Amount:")).toBe("Amount: empty");
      expect(app.texts()).not.toContain(INVALID_AMOUNT);
    }
  });

  test(`${size.name}: a stored value is shown as it is and leaving its box unedited changes nothing; an edit is rounded on leaving`, async () => {
    const app = await open("./masked.fixture.tsx", size.options);
    expect(app.value("id:m-long")).toBe("1.255");
    await app.type("id:m-long", ""); // takes the keys
    await app.type("id:m-other", " "); // leaves
    expect(app.value("id:m-long")).toBe("1.255");
    expect(readout(app, "Long:")).toBe("Long: 1.255");
    await app.type("id:m-long", "9");
    expect(app.value("id:m-long")).toBe("1.2559");
    await app.type("id:m-other", "x");
    expect(app.value("id:m-long")).toBe("1.26");
    expect(readout(app, "Long:")).toBe("Long: 1.26");
  });

  test(`${size.name}: long amounts keep every digit, odd stored values show as they are, partial TINs keep every digit`, async () => {
    const app = await open("./masked.fixture.tsx", size.options);
    await app.type("id:m-amount", "123456789012345678");
    expect(app.value("id:m-amount")).toBe("123456789012345678");
    expect(readout(app, "Amount:")).toBe("Amount: 123456789012345678");
    await app.type("id:m-other", " ");
    expect(app.value("id:m-amount")).toBe("123,456,789,012,345,678.00");
    expect(app.value("id:m-odd")).toBe("1e21"); // was "121"
    expect(readout(app, "Odd:")).toBe("Odd: 1e21");
    expect(app.value("id:m-whole")).toBe("1234.5"); // a typed fraction in a whole-peso box stays as it is
    expect(app.value("id:m-tin")).toBe("000-123-453-0");
    expect(app.value("id:m-rate")).toBe("12.5");
    expect(app.value("id:m-code")).toBe("0123");
  });

  test(`${size.name}: a letter, a second point, a digit past the limit and a pasted minus leave percent and integer boxes as they were`, async () => {
    const app = await open("./masked.fixture.tsx", size.options);
    await app.type("id:m-rate", "x");
    expect(app.value("id:m-rate")).toBe("12.5");
    await app.type("id:m-rate", ".");
    expect(app.value("id:m-rate")).toBe("12.5");
    await app.type("id:m-code", "9");
    expect(app.value("id:m-code")).toBe("0123");
    expect(readout(app, "Code:")).toBe("Code: 0123");
    await app.paste("id:m-code", { text: "-5" });
    expect(app.value("id:m-code")).toBe("0123");
    await app.paste("id:m-rate", { text: "-5" });
    expect(app.value("id:m-rate")).toBe("12.5");
    expect(readout(app, "Rate:")).toBe("Rate: 12.5");
  });
}


for (const size of SIZES) {
  test(`${size.name}: focused money typing sends no text or caret back, at 8, 10 and 12 digits`, async () => {
    const app = await open("./masked.fixture.tsx", size.options);
    for (const digits of ["12345678", "1234567890", "123456789012", "100000"]) {
      await app.clear("id:m-amount");
      const textWrites = readout(app, "Text writes:");
      const caretWrites = readout(app, "Caret writes:");
      await app.type("id:m-amount", digits);
      expect(app.value("id:m-amount")).toBe(digits);
      expect(readout(app, "Amount:")).toBe(`Amount: ${digits}`);
      expect(readout(app, "Text writes:")).toBe(textWrites);
      expect(readout(app, "Caret writes:")).toBe(caretWrites);
      await app.type("id:m-other", " ");
      expect(app.value("id:m-amount")).toBe(formatGroupedExpectation(digits));
    }
    for (const amount of ["12345678.90", "1234567890.12", "123456789012.34", "99999.99", "1000000.005", "99999999.99", "15000.50"]) {
      await app.clear("id:m-amount");
      const textWrites = readout(app, "Text writes:");
      const caretWrites = readout(app, "Caret writes:");
      await app.type("id:m-amount", amount);
      expect(app.value("id:m-amount")).toBe(amount);
      expect(readout(app, "Amount:")).toBe(`Amount: ${amount}`);
      expect(readout(app, "Text writes:")).toBe(textWrites);
      expect(readout(app, "Caret writes:")).toBe(caretWrites);
    }
  });

  test(`${size.name}: focus on a filled grouped amount writes no text or caret, and typing keeps separators until blur`, async () => {
    const app = await open("./masked.fixture.tsx", size.options);
    expect(app.value("id:m-grouped")).toBe("1,234.56");
    const textWrites = readout(app, "Text writes:");
    const caretWrites = readout(app, "Caret writes:");
    await app.type("id:m-grouped", ""); // focus only
    await app.wait(100);
    expect(app.value("id:m-grouped")).toBe("1,234.56");
    expect(readout(app, "Text writes:")).toBe(textWrites);
    expect(readout(app, "Caret writes:")).toBe(caretWrites);
    const beforeTypeText = readout(app, "Text writes:");
    const beforeTypeCaret = readout(app, "Caret writes:");
    await app.type("id:m-grouped", "9");
    expect(app.value("id:m-grouped")).toBe("1,234.569");
    expect(readout(app, "Grouped:")).toBe("Grouped: 1234.569");
    expect(readout(app, "Text writes:")).toBe(beforeTypeText);
    expect(readout(app, "Caret writes:")).toBe(beforeTypeCaret);
    await app.type("id:m-other", " ");
    expect(app.value("id:m-grouped")).toBe("1,234.57");
    expect(readout(app, "Grouped:")).toBe("Grouped: 1234.57");
  });

  test(`${size.name}: a focused zero is empty and, left untouched, shows its zero again without a new report`, async () => {
    const app = await open("./masked.fixture.tsx", size.options);
    expect(app.value("id:m-host")).toBe("0.00");
    await app.type("id:m-host", "");
    await app.waitUntil(() => app.value("id:m-host") === "", { timeout: 2000 });
    await app.type("id:m-other", " ");
    expect(app.value("id:m-host")).toBe("0.00");
    expect(readout(app, "Host:")).toBe("Host: 0.00");
  });

  test(`${size.name}: select-all, Backspace, then 15000.50 is not rewritten by the host's zero`, async () => {
    const app = await open("./masked.fixture.tsx", size.options);
    await app.clear("id:m-host"); // the harness's select-all and Backspace gesture
    await app.wait(100);
    expect(app.value("id:m-host")).toBe("");
    await app.type("id:m-host", "15000.50");
    expect(app.value("id:m-host")).toBe("15000.50");
    expect(readout(app, "Host:")).toBe("Host: 15000.50");
    await app.type("id:m-other", " ");
    expect(app.value("id:m-host")).toBe("15,000.50");
    expect(readout(app, "Host:")).toBe("Host: 15000.50");
  });

  test(`${size.name}: a plain paste, a third decimal and a refused key send no focused write`, async () => {
    const app = await open("./masked.fixture.tsx", size.options);
    await app.clear("id:m-amount");
    const beforePasteText = readout(app, "Text writes:");
    const beforePasteCaret = readout(app, "Caret writes:");
    await app.paste("id:m-amount", { text: "99999999.99" });
    expect(app.value("id:m-amount")).toBe("99999999.99");
    expect(readout(app, "Amount:")).toBe("Amount: 99999999.99");
    expect(readout(app, "Text writes:")).toBe(beforePasteText);
    expect(readout(app, "Caret writes:")).toBe(beforePasteCaret);
    const beforeText = readout(app, "Text writes:");
    const beforeCaret = readout(app, "Caret writes:");
    await app.type("id:m-amount", "5");
    expect(app.value("id:m-amount")).toBe("99999999.995");
    expect(readout(app, "Amount:")).toBe("Amount: 99999999.995");
    expect(readout(app, "Text writes:")).toBe(beforeText);
    expect(readout(app, "Caret writes:")).toBe(beforeCaret);
    await app.type("id:m-amount", "x.");
    expect(app.value("id:m-amount")).toBe("99999999.995x.");
    expect(readout(app, "Amount:")).toBe("Amount: empty");
    expect(app.texts()).not.toContain(INVALID_AMOUNT);
    await app.type("id:m-other", " ");
    expect(app.value("id:m-amount")).toBe("99999999.995x.");
    expect(readout(app, "Amount:")).toBe("Amount: empty");
    expect(app.texts()).toContain(INVALID_AMOUNT);
  });

  test(`${size.name}: typed commas and letters stay exact, invalid and unreported`, async () => {
    for (const bad of ["15000,50", "150a00.50"]) {
      const app = await open("./masked.fixture.tsx", size.options);
      await app.clear("id:m-amount");
      const textWrites = readout(app, "Text writes:");
      const caretWrites = readout(app, "Caret writes:");
      await app.type("id:m-amount", bad);
      expect(app.value("id:m-amount")).toBe(bad);
      expect(readout(app, "Amount:")).toBe("Amount: empty");
      expect(app.texts()).not.toContain(INVALID_AMOUNT);
      expect(readout(app, "Text writes:")).toBe(textWrites);
      expect(readout(app, "Caret writes:")).toBe(caretWrites);
      await app.type("id:m-other", " ");
      expect(app.value("id:m-amount")).toBe(bad);
      expect(readout(app, "Amount:")).toBe("Amount: empty");
      expect(app.texts()).toContain(INVALID_AMOUNT);
    }
  });

  test(`${size.name}: invalid focused money text is never written back, reported or saved`, async () => {
    for (const bad of ["1500,50", "1.234,56", "1,2,3", "1e5", "0x10", "1 234,50"]) {
      const app = await open("./masked.fixture.tsx", size.options);
      await app.clear("id:m-amount");
      const textWrites = readout(app, "Text writes:");
      const caretWrites = readout(app, "Caret writes:");
      await app.paste("id:m-amount", { text: bad });
      expect(app.value("id:m-amount")).toBe(bad);
      expect(readout(app, "Amount:")).toBe("Amount: empty");
      expect(app.texts()).not.toContain(INVALID_AMOUNT);
      expect(readout(app, "Text writes:")).toBe(textWrites);
      expect(readout(app, "Caret writes:")).toBe(caretWrites);
      await app.type("id:m-other", " ");
      expect(app.value("id:m-amount")).toBe(bad);
      expect(readout(app, "Amount:")).toBe("Amount: empty");
      expect(app.texts()).toContain(INVALID_AMOUNT);
    }
  });

  test(`${size.name}: a fast 15000..50 burst stays exact, invalid and unreported`, async () => {
    const app = await open("./masked.fixture.tsx", size.options);
    await app.clear("id:m-amount");
    const textWrites = readout(app, "Text writes:");
    const caretWrites = readout(app, "Caret writes:");
    await app.type("id:m-amount", "15000..50");
    expect(app.value("id:m-amount")).toBe("15000..50");
    expect(app.texts()).not.toContain(INVALID_AMOUNT);
    expect(readout(app, "Amount:")).toBe("Amount: empty");
    expect(readout(app, "Text writes:")).toBe(textWrites);
    expect(readout(app, "Caret writes:")).toBe(caretWrites);
    await app.type("id:m-other", " ");
    expect(app.value("id:m-amount")).toBe("15000..50");
    expect(readout(app, "Amount:")).toBe("Amount: empty");
    expect(app.texts()).toContain(INVALID_AMOUNT);
  });

  test(`${size.name}: invalid typing rolls back to the focus value, and correcting it reports again with no error`, async () => {
    const app = await open("./masked.fixture.tsx", size.options);
    await app.clear("id:m-two"); // the box held 2,000.00 when this focus began
    await app.type("id:m-two", "1");
    await app.type("id:m-two", ",");
    await app.type("id:m-two", "5");
    expect(app.value("id:m-two")).toBe("1,5");
    expect(readout(app, "Invalid:")).toBe("Invalid: 1,5|true");
    expect(readout(app, "Two:")).toBe("Two: 2000");
    expect(app.texts()).not.toContain(INVALID_AMOUNT);
    await app.type("id:m-two", "0");
    await app.type("id:m-two", "0");
    expect(app.value("id:m-two")).toBe("1,500");
    expect(readout(app, "Invalid:")).toBe("Invalid: null");
    expect(readout(app, "Two:")).toBe("Two: 1500");
    expect(app.texts()).not.toContain(INVALID_AMOUNT);
  });

  test(`${size.name}: leaving invalid text raises it, clearing lifts it, and an outside value replaces it`, async () => {
    const app = await open("./masked.fixture.tsx", size.options);
    await app.clear("id:m-two");
    await app.type("id:m-two", "1500,50");
    expect(readout(app, "Two:")).toBe("Two: 2000");
    expect(app.texts()).not.toContain(INVALID_AMOUNT);
    await app.type("id:m-other", " ");
    expect(app.value("id:m-two")).toBe("1500,50");
    expect(readout(app, "Invalid:")).toBe("Invalid: 1500,50|false");
    expect(readout(app, "Two:")).toBe("Two: 2000");
    expect(app.texts()).toContain(INVALID_AMOUNT);

    await app.clear("id:m-two"); // the harness's select-all and Backspace gesture
    await app.type("id:m-other", " ");
    expect(readout(app, "Invalid:")).toBe("Invalid: null");
    expect(readout(app, "Two:")).toBe("Two: 0");
    expect(app.texts()).not.toContain(INVALID_AMOUNT);

    await app.clear("id:m-two");
    await app.type("id:m-two", "1500,50");
    await app.type("id:m-other", " ");
    expect(app.texts()).toContain(INVALID_AMOUNT);
    await app.press("id:m-set-two");
    expect(app.value("id:m-two")).toBe("3,000.00");
    expect(readout(app, "Invalid:")).toBe("Invalid: null");
    expect(app.texts()).not.toContain(INVALID_AMOUNT);
  });

  test(`${size.name}: a valid money key reports in the same turn, with no timer`, async () => {
    const app = await open("./masked.fixture.tsx", { ...size.options, clock: {} });
    await app.clear("id:m-amount");
    await app.type("id:m-amount", "12");
    expect(readout(app, "Amount:")).toBe("Amount: 12");
  });
}

function formatGroupedExpectation(digits: string): string {
  return `${digits.replace(/\B(?=(\d{3})+(?!\d))/g, ",")}.00`;
}
