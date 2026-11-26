// The entry the masked-field tests open (fields.test.tsx): one of each masked box with a readout of the value it reports, a
// count of how often it reported, and a plain box to leave the others for. Tests only; nothing imports it. The readouts are
// first so a phone-sized window shows them all.
import { mount, signal, View } from "nsh";
import { IntInput, MoneyInput, PercentInput, TextField, TinInput } from "./fields";
import { Text } from "./text";

function App() {
  const amount = signal("");
  const grouped = signal("1234.56");
  const reports = signal(0);
  const textWrites = signal(0);
  const caretWrites = signal(0);
  const stored = signal("-250");
  const free = signal("");
  const whole = signal("1234.5");
  const odd = signal("1e21");
  const long = signal("1.255");
  const tin = signal("0001234530");
  const rate = signal("12.5");
  const code = signal("0123");
  const other = signal("");
  const host = signal("0.00");
  return (
    <View class="gap-2 p-3">
      <View class="flex-row flex-wrap gap-x-3">
        <Text variant="cap">{`Amount: ${amount() || "empty"}`}</Text>
        <Text variant="cap">{`Grouped: ${grouped() || "empty"}`}</Text>
        <Text variant="cap">{`Reports: ${reports()}`}</Text>
        <Text variant="cap">{`Text writes: ${textWrites()}`}</Text>
        <Text variant="cap">{`Caret writes: ${caretWrites()}`}</Text>
        <Text variant="cap">{`Stored: ${stored() || "empty"}`}</Text>
        <Text variant="cap">{`Free: ${free() || "empty"}`}</Text>
        <Text variant="cap">{`Whole: ${whole() || "empty"}`}</Text>
        <Text variant="cap">{`Odd: ${odd() || "empty"}`}</Text>
        <Text variant="cap">{`Long: ${long() || "empty"}`}</Text>
        <Text variant="cap">{`Tin: ${tin() || "empty"}`}</Text>
        <Text variant="cap">{`Rate: ${rate() || "empty"}`}</Text>
        <Text variant="cap">{`Code: ${code() || "empty"}`}</Text>
        <Text variant="cap">{`Host: ${host() || "empty"}`}</Text>
      </View>
      <View class="flex-row flex-wrap gap-2">
      <TextField width={100} label="Other" value={other()} onChange={(v) => other.set(v)} testID="m-other" />
      <MoneyInput
        width={100}
        label="Amount"
        value={amount()}
        onChange={(v) => {
          amount.set(v);
          reports.update((n) => n + 1);
        }}
        testID="m-amount"
        writeProbe={(write) => (write === "text" ? textWrites.update((n) => n + 1) : caretWrites.update((n) => n + 1))}
      />
      <MoneyInput
        width={100}
        label="Grouped"
        value={grouped()}
        onChange={(v) => grouped.set(v)}
        testID="m-grouped"
        writeProbe={(write) => (write === "text" ? textWrites.update((n) => n + 1) : caretWrites.update((n) => n + 1))}
      />
      <MoneyInput width={100} label="Stored" value={stored()} onChange={(v) => stored.set(v)} testID="m-stored" />
      <MoneyInput width={100} label="Free" negative value={free()} onChange={(v) => free.set(v)} testID="m-free" />
      <MoneyInput width={100} label="Whole" whole value={whole()} onChange={(v) => whole.set(v)} testID="m-whole" />
      <MoneyInput width={100} label="Odd" value={odd()} onChange={(v) => odd.set(v)} testID="m-odd" />
      <MoneyInput width={100} label="Long" value={long()} onChange={(v) => long.set(v)} testID="m-long" />
      <TinInput width={100} label="TIN" value={tin()} onChange={(v) => tin.set(v)} testID="m-tin" />
      <PercentInput width={100} label="Rate" value={rate()} onChange={(v) => rate.set(v)} testID="m-rate" />
      <IntInput width={100} label="Code" maxDigits={4} value={code()} onChange={(v) => code.set(v)} testID="m-code" />
      <MoneyInput
        width={100}
        label="Host"
        value={host()}
        onChange={(v) => {
          // A form that cannot hold a blank amount answers "" with its stored zero, as the host does.
          host.set(v === "" ? "0.00" : v);
        }}
        testID="m-host"
      />
      </View>
    </View>
  );
}

mount(App);
