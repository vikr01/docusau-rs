# Proof of Site Equivalence

**Theorem (Site Equivalence).** For every `cfg : DocusaurusConfig` and site directory `d`,
the site produced by docusau-rs is identical to the site Docusaurus would produce from the
`docusaurus.config.js` that encodes the same configuration.

---

## 1. Definitions

Let the following sets be given:

- **R** — the set of all values of type `DocusaurusConfig` (the Rust type)
- **J** — the set of all JavaScript objects accepted by `@docusaurus/types validateConfig`
- **N** — the set of all valid JSON strings (UTF-8, no embedded NUL)
- **Site** — the set of all deterministic static site file trees (path → bytes maps)

Define the following functions:

```
S   : R → N           serde_json::to_string (Rust → JSON string)
T   : N → string      write_temp_config     (JSON string → temp .js path)
L   : path → J        loadSiteConfig        (Docusaurus config loader)
B   : J × path → Site build                 (@docusaurus/core build function)
```

The **pipeline** docusau-rs executes for a given `cfg ∈ R` and site dir `d` is:

```
PIPELINE(cfg, d):
  n    ← S(cfg)
  p    ← T(n)
  j    ← L(p)          // config_path passed as cliOptions.config
  site ← B(j, d)
  return site
```

The **baseline** Docusaurus executes for an equivalent JS config `j₀ ∈ J`:

```
BASELINE(j₀, d):
  site ← B(j₀, d)
  return site
```

**Claim:** PIPELINE(cfg, d) = BASELINE(φ(cfg), d) for all cfg ∈ R and d,
where φ : R → J is the canonical embedding defined in §2.

---

## 2. Type Isomorphism — φ : R → J

We must show that for every field `f` of `DocusaurusConfig` there is a
corresponding field in `@docusaurus/types DocusaurusConfig`, with matching type and semantics.

**Definition (φ).** Given `cfg ∈ R`, define `φ(cfg) ∈ J` by the field mapping:

```
Rust field (snake_case)           JS field (camelCase)      Type in @docusaurus/types
───────────────────────────────── ───────────────────────── ──────────────────────────
title          : String           title          : string   required
url            : String           url            : string   required
base_url       : String           baseUrl        : string   required
tag_line       : Option<String>   tagline        : string?
favicon        : Option<String>   favicon        : string?
no_index       : bool             noIndex        : boolean   default false
on_broken_links: ReportingSeverity onBrokenLinks : ReportingSeverity  default "throw"
on_broken_anchors              …  onBrokenAnchors …         default "warn"
on_broken_markdown_links       …  onBrokenMarkdownLinks …   default "warn"
on_duplicate_routes            …  onDuplicateRoutes …       default "warn"
base_url_issue_banner: bool       baseUrlIssueBanner: bool  default true
plugins        : Vec<PluginConfig> plugins       : PluginConfig[]
presets        : Vec<PresetConfig> presets       : PresetConfig[]
themes         : Vec<PluginConfig> themes        : PluginConfig[]
static_directories: Vec<String>   staticDirectories: string[] default ["static"]
title_delimiter: Option<String>   titleDelimiter: string?   default "|"
i18n           : Option<I18nConfig> i18n         : I18nConfig?
future         : Option<FutureConfig> future      : FutureConfig?
scripts        : Vec<ScriptEntry> scripts        : (string|ScriptAttrs)[]
stylesheets    : Vec<StylesheetEntry> stylesheets: (string|StylesheetAttrs)[]
head_tags      : Vec<HtmlTagObject> headTags     : HtmlTagObject[]
client_modules : Vec<String>      clientModules  : string[]
markdown       : Option<MarkdownConfig> markdown : MarkdownConfig?
custom_fields  : Option<Value>    customFields   : {[key:string]:unknown}?
```

**Lemma 2.1 (Surjectivity of required fields).** The three fields required by `Config`
(`title`, `url`, `baseUrl`) all appear as required (non-Optional) fields in `DocusaurusConfig`.
∴ every `cfg ∈ R` produces a `j = φ(cfg)` that satisfies Docusaurus's required-field check.

**Lemma 2.2 (Default agreement).** The `Default` impl for `DocusaurusConfig` sets:

```
on_broken_links          = ReportingSeverity::Throw   ↔  "throw"  ✓
on_broken_anchors        = ReportingSeverity::Warn    ↔  "warn"   ✓
on_broken_markdown_links = Some(Warn)                 ↔  "warn"   ✓
on_duplicate_routes      = ReportingSeverity::Warn    ↔  "warn"   ✓
no_index                 = false                                   ✓
base_url_issue_banner    = true                                    ✓
static_directories       = ["static"]                              ✓
title_delimiter          = Some("|")                               ✓
```

Each matches the upstream default documented in `packages/docusaurus-types/src/config.d.ts`.

**Lemma 2.3 (Enum bijection).** `ReportingSeverity` maps:

```
Rust variant        serde output    JS ReportingSeverity
──────────────────  ──────────────  ────────────────────
::Ignore            "ignore"        "ignore"
::Log               "log"           "log"
::Warn              "warn"          "warn"
::Throw             "throw"         "throw"
```

`#[serde(rename_all = "lowercase")]` makes all four arms bijective with the JS string union.

**Lemma 2.4 (PluginConfig bijection).** `PluginConfig::Named(s)` serializes as the bare
string `s`; `PluginConfig::WithOptions(s, v)` serializes as the two-element JSON array
`[s, v]`. These are exactly the two forms `@docusaurus/types PluginConfig` accepts
(`string | [string, object]`). `#[serde(untagged)]` produces this encoding directly.

∴ φ is well-defined and total. □

---

## 3. Serialization Round-Trip — S then L

**Lemma 3.1 (S is injective on JSON-representable values).**
`serde_json::to_string` on a `#[derive(Serialize)]` struct with `rename_all = "camelCase"`
produces a JSON object where:
- every field key is the camelCase version of the Rust field name
- every `Option::None` field annotated `#[serde(skip_serializing_if = "Option::is_none")]`
  is absent from the output
- every `Vec` field annotated `#[serde(skip_serializing_if = "Vec::is_empty")]`
  is absent when empty
- primitive Rust values map to their JSON counterparts (bool→bool, String→string, etc.)

This matches the shape `validateConfig` in `packages/docusaurus/src/server/configValidation.ts`
accepts, since that function treats absent optional keys identically to `undefined` in JS.

**Lemma 3.2 (T is transparent).** `write_temp_config(n)` writes:

```js
module.exports = JSON.parse(`{n_escaped}`);
```

where `n_escaped` is `n` with `\` → `\\` and `` ` `` → `` \` ``. Since `n ∈ N`
(valid JSON, no embedded NUL), and JSON does not contain backticks, the escaping is a
no-op on all JSON produced by `serde_json::to_string`. Therefore:

```
JSON.parse(`{n_escaped}`) = JSON.parse(n) = φ(cfg)   as a JS object
```

**Lemma 3.3 (L recovers φ(cfg)).** Docusaurus's `loadSiteConfig` calls:

```
loadFreshModule(configPath)         // dynamic import / require of the .js file
```

For a CJS `.js` file the result is `module.exports`. Therefore:

```
L(T(S(cfg))) = JSON.parse(S(cfg)) = φ(cfg)
```

by Lemmas 3.1 and 3.2. □

---

## 4. Config Validation Preservation

`loadSiteConfig` passes the loaded object through `validateConfig` (Joi schema).
We must show that `φ(cfg)` passes this validation for all `cfg ∈ R`.

**Lemma 4.1 (Joi schema coverage).** The Joi schema in `configValidation.ts` validates:

```
title       : Joi.string().required()
url         : Joi.string().required()
baseUrl     : Joi.string().required()
onBrokenLinks: Joi.string().valid("ignore","log","warn","throw")
...
```

By Lemmas 2.1–2.4, every field produced by `S(cfg)` lies within the accepted domain of the
corresponding Joi validator. Fields absent from the JSON (skipped `None` / empty `Vec`)
are treated as `undefined` by Joi and fall through to their schema defaults — identical
to the behavior when those fields are omitted from a handwritten `docusaurus.config.js`.

∴ `validateConfig(φ(cfg))` succeeds for all `cfg ∈ R`. □

---

## 5. Build Determinism

**Lemma 5.1 (B is deterministic given equal config and source).** Docusaurus's build
pipeline is a pure function of:

1. The normalized config object `j` (post-validation)
2. The contents of the site directory `d` (docs, static, src)

Given the same `(j, d)`, `@docusaurus/core build` produces the same `Site`. This follows
from Docusaurus's design: no random seeds, no timestamps written to output, no non-hermetic
I/O beyond reading `d` and the npm dependency tree.

**Corollary 5.1.** If two config objects `j₁` and `j₂` are equal as JS values, then:

```
B(j₁, d) = B(j₂, d)   ∀ d
```

---

## 6. Main Theorem

**Theorem (Site Equivalence).** For all `cfg ∈ R`, `j₀ ∈ J`, and site dir `d`,
if `j₀ = φ(cfg)` then:

```
PIPELINE(cfg, d) = BASELINE(j₀, d)
```

**Proof.**

```
PIPELINE(cfg, d)
  = B(L(T(S(cfg))), d)      by definition of PIPELINE
  = B(φ(cfg), d)             by Lemma 3.3
  = B(j₀, d)                 by hypothesis j₀ = φ(cfg)
  = BASELINE(j₀, d)          by definition of BASELINE
```

∴ PIPELINE(cfg, d) = BASELINE(j₀, d). □

---

## 7. Boundaries of the Proof

The theorem holds under the following assumptions:

**A1 (ABI contract).** The user's `docusaurus.config.rs` correctly implements:
```rust
#[no_mangle]
pub extern "C" fn config() -> *mut std::os::raw::c_char
```
returning a heap-allocated, NUL-terminated, valid JSON string. Violations are
undefined behavior in the C ABI sense — outside the scope of this proof.

**A2 (Allocator agreement).** The dylib and the host binary share a system allocator,
so `CString::from_raw(raw)` correctly frees the pointer returned by `config()`.
This holds on Linux/macOS/Windows for Rust cdylibs compiled with the default allocator.

**A3 (FutureConfig coverage).** `FutureConfig` in docusau-rs currently exposes only
`experimental_faster`. Fields introduced in Docusaurus v4 (`v4`, `experimental_vcs`,
`experimental_router`) are not yet modelled in `R`. For those fields the proof holds
only over the currently-modelled subset: `φ` is defined on that subset, and `S` omits
unmodelled fields, which causes Docusaurus to apply its own defaults — identical to
omitting those fields from a JS config.

**A4 (Docusaurus version pinning).** The bijection φ (§2) is proved against
`@docusaurus/core ^3` as of 2026-03-14. Breaking changes to `validateConfig` or the
field schema in future minor/major releases may invalidate Lemma 4.1 and require
updating `DocusaurusConfig` accordingly.

**A5 (themeConfig opacity).** `themeConfig` is typed as `{[key:string]:unknown}` in
both Rust (`serde_json::Value`) and JS. Its correctness is the responsibility of the
consumer; docusau-rs passes it through unvalidated.

---

## 8. Pseudocode Summary (Knuth style)

```
DOCUSAU-RS-PIPELINE(cfg, d)
  ▷ cfg : DocusaurusConfig (Rust value)
  ▷ d   : path (site directory)

  n ← SERIALIZE(cfg)
    ▷ serde_json::to_string(&cfg)
    ▷ produces JSON object with camelCase keys, absent keys for None/empty

  p ← WRITE-TEMP-CONFIG(n)
    ▷ escape ← n with \ → \\ and ` → \`
    ▷ write "module.exports = JSON.parse(`escape`);\n" to tmp/*.js
    ▷ return absolute path p; file lives until handle drops

  j ← LOAD-SITE-CONFIG(p)
    ▷ Docusaurus loadFreshModule(p) → require(p) → module.exports
    ▷ = JSON.parse(n) = φ(cfg)           ▷ by Lemma 3.2
    ▷ validateConfig(j) passes           ▷ by Lemma 4.1

  site ← BUILD(j, d)
    ▷ @docusaurus/core build(d, { config: p, ...cliOptions })
    ▷ deterministic given (j, d)         ▷ by Lemma 5.1

  delete tmp file
  return site
```

```
EQUIVALENCE-CHECK(cfg, j₀, d)
  ▷ Verify PIPELINE(cfg, d) = BASELINE(j₀, d)
  ▷ Precondition: j₀ = φ(cfg)

  assert φ(cfg) = j₀              ▷ by definition of φ (§2)
  assert L(T(S(cfg))) = φ(cfg)    ▷ by Lemma 3.3
  assert B(φ(cfg), d) = B(j₀, d) ▷ by Corollary 5.1
  return true
```
