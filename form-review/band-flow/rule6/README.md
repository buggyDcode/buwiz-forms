# RULES 6 Proofs — Overflowing Inline Specify Boxes (`try/forms-band-flow`)

Real saved-return proofs for the RULES 6 override (`Other Tax Credits/Payments (specify)` and fields of the same shape) entered through the running eBIRForms desktop app (`just nsh-run-agent linux`, synthetic taxpayer `SANTOS, JUANA`, TIN `000-123-453-00000`, `BIR_SFTP_DRY_RUN=1`, `BIR_SFTP_LIVE` unset). Every return's SQLite `data_json` byte length was verified unchanged across print.

## Classification (inspected on each form's own sheet before enabling)

- **Switched ON (8 forms, 9 fields)** — small inline specify box inside the label column sharing its row with the amount box:
  - `2551q/` — **2551Q** Item 17 (`frm2551Qv2018:txt17Specify`)
  - `2550q/` — **2550Q** Item 19 (`frm2550qv2024:addSpecifyNo19`)
  - `1700/` — **1700** Item 57 (`frm1700:txtPg2I57Desc`)
  - `1701a/` — **1701A** Item 63 (`frm1701A:txt63Desc`)
  - `1701q/` — **1701Q** Item 61 (`frm1701q:txt61Desc`)
  - `1701ms/` — **1701MS** Items 7 & 9 (`derived:foreign_tax_credits_description`, `derived:other_credits_description`)
  - `1601c/` — **1601C** Item 20 (`frm1601c:txt20Other`)
  - `1603q/` — **1603Q** Item 16 (`frm1603Q:txt16Other`)
- **Left OFF (3 forms)** — do not have that shape (they put the "(specify)" heading above two full-width character-comb rows `348.67pt × 20.52pt`):
  - `1702EX` (Items 48–49), `1702RT` (Items 53–54), `1702Q` (Schedule 4 Items 6a–6b).

## What each folder contains

Each of the 8 folders (`2551q`, `2550q`, `1700`, `1701a`, `1701q`, `1701ms`, `1601c`, `1603q`) holds:
- `before-page-<N>.png` — target page rendered before RULES 6 was enabled (overflowing/cramped inline specify text).
- `after-preview-page-*.png` and `after-pdf-page-*.png` — after enabling RULES 6: the small inline box stays blank, the amount stays level with the item number, and the full specify text prints at `7.5pt` inside a bordered full-width band directly beneath the row (`rule6-inserted-line`), pushing lower bands down cleanly (and breaking cleanly onto a continuation page where needed, e.g. `1601C`).
- `recording.mp4` — Pattern-N flow recording: live frames of the running eBIRForms GPUI app window open on the form page (including the Tax Credits / Specify part with the long specify text and amount entered and saved), followed by each resulting print sheet produced by clicking **Print preview** (`fp-print`).

## Recording capture note (what it shows and does not show)

The eBIRForms app's native WebKit print preview window cannot be captured directly via `x11grab` on this headless Linux X11 desktop. Each `recording.mp4` therefore captures the **form-page portion directly from the running eBIRForms GPUI app** (via `gpui-agent screenshot`, showing the live form page, the specify section with the entered text/amount, and the `✓ Saved` / `Print preview` controls) and the **sheet portion from the exact `sheet.html` emitted by clicking `Print preview` (`fp-print`)**, rendered page by page. It does not show the native WebKit print dialog chrome or any live BIR transmission.
