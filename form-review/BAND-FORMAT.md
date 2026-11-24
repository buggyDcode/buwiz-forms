# Page Band Layout Format (`formgen-page-layout` v1)

This document specifies the per-page band layout metadata emitted by the frozen-HTML generator (`buwiz-forms-html`) and consumed by `bir-print`'s in-page band-flow engine (Rules 5 and 6 in `form-review/RULES.md`).

## 1. Script element & backward compatibility

Embed a single JSON script element inside `<head>` (or immediately after `<script type="application/json" id="formgen-bands">`) in `html-frozen/<slug>/index.html`:

```html
<script type="application/json" id="formgen-page-layout">{ ... }</script>
```

- Existing `<script type="application/json" id="formgen-bands">` stays intact.
- Forms or pages that do not yet have `formgen-page-layout` continue to use the existing continuation-sheet / plain-fallback engine without error.
- When a return has no overflowing rows and no Rule 6 inserted lines, the engine does not run band reflow and emits the frozen page byte-for-byte unchanged.

## 2. Top-level JSON schema

```json
{
  "version": 1,
  "pages": [
    {
      "page": 2,
      "width_pt": 612.0,
      "height_pt": 936.0,
      "content_bottom_y": 914.0,
      "table_x0": 17.64,
      "table_x1": 594.48,
      "bands": [
        {
          "id": "p2b0",
          "kind": "page_header",
          "y0": 0.0,
          "y1": 129.0,
          "keep_with_next": false,
          "keep_with_prev": false,
          "repeat_on_break": true,
          "min_height_pt": null,
          "page_label_text_id": "p2t4",
          "growable_id": null,
          "table_group": null,
          "svg_ids": ["h1", "v1"],
          "svg_indices": [0, 1],
          "text_ids": ["p2t0", "p2t1", "p2t4"],
          "cell_ids": ["p2c0", "p2c1"],
          "spanning_svg_ids": []
        }
      ],
      "rule6_fields": [
        {
          "writer_key": "txtSpecify",
          "cell_id": "p1c42",
          "band_id": "p1b8",
          "insert_after_y": 512.4,
          "x0": 17.64,
          "x1": 594.48
        }
      ]
    }
  ]
}
```

## 3. Page fields (`pages[]`)

| Field | Type | Meaning |
|---|---|---|
| `page` | `usize` (1-based) | Page number matching `<div class="page page-<N>" id="page-<N>">`. |
| `width_pt` | `f64` | Native page width in points (matches `style="width:Wpt;height:Hpt"` and SVG `viewBox`). |
| `height_pt` | `f64` | Native page height in points. |
| `content_bottom_y` | `f64` | Lowest Y coordinate (in native page pt) that content may occupy before breaking to a new page. Must leave at least **18 pt** free at the bottom for the shared page footer (Rule 7) plus any bottom page margin. |
| `table_x0` | `f64` | Left X coordinate of the main outer form frame (used for Rule 6 inserted rectangles and closing rules at mid-table page breaks). |
| `table_x1` | `f64` | Right X coordinate of the main outer form frame. |
| `bands` | `Band[]` | Ordered, contiguous, non-overlapping top-to-bottom partition covering `[0, height_pt]`. |
| `rule6_fields` | `Rule6Field[]` | Optional list (may be empty `[]`) of Rule 6 "Other … (specify)" fields on this page. |

## 4. Band fields (`bands[]`)

| Field | Type | Meaning |
|---|---|---|
| `id` | `string` | Unique band ID within the document (e.g. `p2b0`, `p2b1`, …). |
| `kind` | `string` | One of the 8 enum values below. |
| `y0` | `f64` | Top Y coordinate of the band in native page points. |
| `y1` | `f64` | Bottom Y coordinate of the band in native page points (`y1 >= y0`). |
| `keep_with_next` | `bool` | Never break immediately after this band (set `true` on `section_title` and `table_head`). |
| `keep_with_prev` | `bool` | Never break immediately before this band (set `true` on `total_row`). |
| `repeat_on_break` | `bool` | Re-emit this band at the top of a continuation page (`true` on `page_header`, and on `table_head` when breaking mid-table). |
| `min_height_pt` | `f64 \| null` | Required when `kind == "elastic_box"`, `null` otherwise. The engine will never shrink an elastic box below `min_height_pt` and will never shrink a non-elastic band. Never mark a box where the taxpayer or bank writes/stamps as elastic below a usable size; when in doubt emit `plain_block` with `min_height_pt: null`. |
| `page_label_text_id` | `string \| null` | On `page_header`: the `<span id="...">` holding the page number text (e.g. `Page 2`). On spawned continuation pages, the engine replaces this label with `Page <p> (continued)` (always in full, never `(cont.)`). |
| `growable_id` | `string \| null` | When `kind == "row_template"`: the `id` of the matching band in `<script id="formgen-bands">` (e.g. `"p2g1"`). `null` for all other kinds. |
| `table_group` | `string \| null` | Shared group identifier tying together a table's `section_title`, `table_head`, `row_template`, and `total_row` — and linking paired half-tables (e.g. 1707 Schedule 2 upper/lower, 1801 Schedule 1 upper/lower) so each half grows in place with its own rows. |
| `svg_ids` | `string[]` | `data-rule-id` (or `id`) values of SVG elements in `<svg class="rl">` owned by this band. |
| `svg_indices` | `usize[]` | 0-based child indices (among direct drawable children of `<svg class="rl">`, or within `<g class="layer-structural">` if grouped) for SVG elements that have no `id`/`data-rule-id` (such as comb group-mark ticks, decorative shading, or un-ID'd borders). |
| `text_ids` | `string[]` | `id` values of `<span>` elements in `<div class="layer-text">` owned by this band. |
| `cell_ids` | `string[]` | `id` values of `<div class="c">` elements in `<div class="layer-cells">` owned by this band. Any fill-time overlay keyed to a cell (inside the cell element or carrying `data-overflow-of="<cell_id>"`) moves automatically with that cell. |
| `spanning_svg_ids` | `string[]` | Vertical border/separator SVG elements (by `data-rule-id`/`id`) that span across `y0..y1` of a `row_template` or `elastic_box` and must stretch by `+n * row_pitch_pt` on growth, shrink by `-dh` on elastic compression, or split at a mid-table page break with a closing horizontal rule. |

### Band `kind` values

1. `page_header` — Top strip (form number, title, barcode, page label, TIN, taxpayer's name). Opens every continuation page spawned from this page (`repeat_on_break: true`).
2. `section_title` — Part or schedule heading banner (`keep_with_next: true`).
3. `table_head` — Table column captions (`keep_with_next: true`, `repeat_on_break: true`). When repeated after a mid-table break, `(continued)` is appended once (skipped if the heading already contains `continuation` or `continued`).
4. `row_template` — Growable `k`-row table body referenced by `growable_id`. Breaks only between rows; extra rows are inserted directly beneath row `k-1` at `row_pitch_pt`. On a mid-table break, the engine keeps at least one row with `table_head`, keeps at least one row with `total_row`, and draws `"Continued on the next page"` below the last row on the breaking page.
5. `total_row` — Table footer/total (`keep_with_prev: true`). Prints once, immediately after the table's final row wherever that row lands; never printed on an earlier page if the table continues past it.
6. `plain_block` — Any non-growing static section (e.g. Schedule 1, Taxpayer Classification, signature block). Indivisible (`keep_together`).
7. `elastic_box` — Empty stamp, validation, or remarks box with an explicit `min_height_pt`. Consumed only after `trailing_space` is exhausted and before breaking a page.
8. `trailing_space` — Empty bottom space below the last content band up to `content_bottom_y`. Consumed first.

### Conservation invariant

Every drawable SVG node, `<span id>` in `<div class="layer-text">`, and `<div class="c" id>` in `<div class="layer-cells">` on a page with layout data must be owned by **exactly one** band (via `svg_ids`/`svg_indices`/`spanning_svg_ids`, `text_ids`, `cell_ids`). Across any flowed output document:

- Every non-repeating element appears **exactly once** across the output pages.
- Every element in a `repeat_on_break` band appears **once per page that band opens**.
- No element's shifted bounding box lies outside `[0, content_bottom_y]` (except page-footer / bottom-frame margin elements in `page_header` or below `content_bottom_y` on untouched pages).

## 5. Rule 6 fields (`rule6_fields[]`)

| Field | Type | Meaning |
|---|---|---|
| `writer_key` | `string` | BIR writer field key for the inline "specify" box (e.g. `txtSpecify`). |
| `cell_id` | `string` | `<div class="c" id="...">` of the small inline specify box. |
| `band_id` | `string` | Band owning that item row. |
| `insert_after_y` | `f64` | Y coordinate (native page pt) immediately below the item row where the full-width wrapped rectangle band is inserted when the specify text overflows `cell_id`. |
| `x0`, `x1` | `f64` | Left and right X coordinates of the inserted full-width bordered rectangle. |

## 6. Addition to existing `formgen-bands` (`rows[template_row]`)

In `<script type="application/json" id="formgen-bands">`, each `rows[i]` entry currently carries `rules`, `cells`, and `texts`. Add one optional field on `rows[i]` (required on `rows[template_row]`):

```json
"structural_rects": [
  { "x": 419.11, "y": 445.87, "w": 2.16, "h": 6.96, "fill": "#000000" }
]
```

Include every un-ID'd structural SVG `<rect>` that sits inside that row's vertical span (comb group-mark ticks, grey decimal shading, etc.) so cloned rows never rely on bounding-box heuristics. If omitted on older metadata, the engine falls back to scanning un-ID'd structural rects within `[row_y[t], row_y[t+1]]`.
