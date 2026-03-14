# Proof of Site Equivalence

**Theorem** *(conditional on axioms A1–A4, stated in §5).*  For every well-formed Rust
config `cfg` and site directory `d`, `PIPELINE(cfg, d) = BASELINE(cfg, d)`,
where BASELINE is native Docusaurus invoked with the semantically equivalent JS config.

---

## 1. Architecture

docusau-rs is a **config adapter**. It does not reimplement Docusaurus. The build, serve,
and deploy steps execute `@docusaurus/core`'s own exported functions:

```
PIPELINE(cfg, d)   :=  B(L(S(cfg)), d)
BASELINE(cfg, d)   :=  B(L(J(cfg)),  d)
```

where:
- **B** = `@docusaurus/core`'s `docusaurus_build` — identical in both cases
- **L** = `@docusaurus/core`'s `load_config` (includes Joi validation) — identical in both cases
- **S(cfg)** = the JS object produced by serializing the Rust config via serde
- **J(cfg)** = the JS object produced by the semantically equivalent hand-written JS config

The only difference between PIPELINE and BASELINE is whether the config object reaching L
was produced by S or by J.

---

## 2. Reduction

The theorem reduces to a single claim:

> **Claim.**  S(cfg) = J(cfg)  for every well-formed cfg.

*Proof that the claim implies the theorem.*
L is deterministic: same input → same output.
∴ S(cfg) = J(cfg) ⟹ L(S(cfg)) = L(J(cfg)).
B is deterministic: same validated config and same site directory → same output.
∴ L(S(cfg)) = L(J(cfg)) ⟹ B(L(S(cfg)), d) = B(L(J(cfg)), d).
∴ PIPELINE(cfg, d) = BASELINE(cfg, d). □

---

## 3. Proof of the Claim

The claim S(cfg) = J(cfg) has two parts:

### 3.1 The design invariant (A1)

**A1 (Design Invariant).** For every field of `DocusaurusConfig`, the value produced by
serde serialization matches the value a semantically equivalent hand-written JS config
would export for that field.

`DocusaurusConfig` is defined to structurally mirror `@docusaurus/types`'s `DocusaurusConfig`.
The serde mapping is:

- field names: `#[serde(rename_all = "camelCase")]` at struct level; exceptions explicit via
  `#[serde(rename = "...")]`
- absent optionals: `#[serde(skip_serializing_if = "Option::is_none")]` — omitted rather than
  null, matching JS where optional fields are simply absent
- enums: `#[serde(rename_all = "lowercase")]` for severity values (`"ignore"`, `"log"`, etc.);
  `#[serde(untagged)]` for sum types (`PluginConfig`) — produces bare value or array,
  matching the `name | [name, options]` JS convention
- empty lists: `#[serde(skip_serializing_if = "Vec::is_empty")]` — omitted, matching JS defaults

This mapping is the **design invariant** of the crate: A1 asserts it holds for every field.
The Joi schema in `@docusaurus/core` then normalizes both objects identically,
so L(S(cfg)) = L(J(cfg)) follows from S(cfg) = J(cfg).

### 3.2 E is an identity on content

`S(cfg)` produces a JSON string n. E transforms n into E(n) via three sequential
substitutions applied in order:

```
  E₁ :  \   →  \\
  E₂ :  `   →  \`
  E₃ :  ${  →  \${
```

These are the only three sequences with special meaning at the **lexer level** of a JS
template literal (ECMAScript 2023 §13.2.8): `\` begins an escape sequence, `` ` `` closes
the literal, and `${` is recognized by the lexer as the start of a template expression.
Escaping them prevents the lexer from treating them as syntax.

**Claim.**  The JS module `module.exports = JSON.parse(\`E(n)\`)` exports the same object
that `JSON.parse(n)` would produce.

*Proof.*  E is applied in a fixed order; we show each step is safe and that the composed
result embeds n faithfully.

*Step 1 — E₁.* Replace every `\` with `\\`. After this step, no lone backslash remains
in the string. In particular, E₁ introduces only `\\` sequences and cannot produce `` \` ``
or `\${`.

*Step 2 — E₂.* Replace every `` ` `` with `` \` ``. The `\` introduced here originates from
E₂ alone (E₁ already doubled all prior backslashes), so no double-escaping ambiguity arises.

*Step 3 — E₃.* Replace every `${` with `\${`. Any `${` still present at this point is
literal content (not yet an escape sequence), and the introduced `\` has no prior backslash
adjacent to it (E₁ handled all original backslashes, and E₂ produces only the two-character
sequence `` \` `` — which ends with a backtick, not `$` — and therefore cannot create a new
`${` that E₃ would then spuriously match).

The resulting string E(n), embedded in the template literal, is parsed by the JS lexer as
a sequence of `TemplateCharacter` tokens. Each escape introduced by E₁–E₃ is inverted
exactly by the ECMAScript 2023 §12.9.6 template value (TV) rules for untagged template
literals:

| Escape in E(n) | Introduced by | Lexer rule                               | TV (content) |
|----------------|---------------|------------------------------------------|--------------|
| `\\`           | E₁            | SingleEscapeCharacter `\` → `\`          | `\`          |
| `` \` ``       | E₂            | SingleEscapeCharacter `` ` `` → `` ` ``  | `` ` ``      |
| `\${`          | E₃            | `\$` = NonEscapeCharacter → `$`; `{` bare character → `{` | `${` |

(`$` is a NonEscapeCharacter because it is neither a SingleEscapeCharacter nor a digit nor
`x` nor `u` nor a LineTerminator — ECMAScript 2023 §12.9.6.)

All other characters pass through verbatim. The TV of the whole template is therefore
exactly n.

Note: E(n) appears only in the **template literal source**. The JS engine evaluates the
template literal first, recovering the original string n as its TV, and only then passes n
to JSON.parse. JSON.parse never sees E(n); it sees n. The escape sequences in E(n) are a
source-level concern, fully resolved by template evaluation before JSON.parse is invoked.

∴ eval_tmpl(E(n)) = n, and JSON.parse(eval_tmpl(E(n))) = JSON.parse(n). □

Because S(cfg) embeds n via E and the JS module exports the value recovered by eval_tmpl(E(n)),
the JS object obtained is the same as if n had been parsed directly.

### 3.3 Conclusion

By 3.1, the pre-validation JS object is the same (A1).
By 3.2, E does not alter content.
∴ S(cfg) = J(cfg), and by §2, PIPELINE(cfg, d) = BASELINE(cfg, d). □

---

## 4. Corollaries

### 4.1 Forward compatibility

When `@docusaurus/core` adds new config fields, the pipeline requires no structural change.
Any field the user adds to their Rust config serializes through S unchanged. If the new field
is typed in Rust, A1 (§3.1) extends to it by the same serde mapping rules. If it is
untyped, `serde_json::Value` carries it verbatim. In both cases the object reaching L is
identical to what a hand-written JS config would produce. Docusaurus's own Joi schema accepts
or rejects it — no docusau-rs code needs to change.

### 4.2 Plugin support (JS and Rust)

Plugin entries are `Named(String)` or `WithOptions(String, serde_json::Value)`. Both serialize
to either a bare string or a two-element array — the JS `name | [name, options]` convention.
The values are opaque to docusau-rs: it does not interpret them.

- **JS plugins**: referenced by npm package name or path. S passes the string through; L's
  plugin loader receives it identically to a hand-written JS config.
- **Rust plugins**: compiled to a `.node` napi-rs addon, then referenced by filesystem path.
  From L's perspective this is a Node.js module exporting a plugin factory — the same contract
  as any JS plugin.

∴ plugin fields are config fields; the main theorem (§3) covers them without modification.

---

## 5. Assumptions

**A1.** `DocusaurusConfig` and its serde attributes faithfully mirror `@docusaurus/types`'s
`DocusaurusConfig`. The mapping rules are stated in §3.1; correctness is a design invariant
maintained by the crate, verified against the Joi schema.

**A2.** E is an identity on content (proved in §3.2).

**A3.** B and L are deterministic: same inputs → same outputs. This holds for
`@docusaurus/core`'s pipeline; user-installed plugins with external side effects are outside scope.

**A4.** The C FFI layer correctly conveys the serialized config string without corruption
(valid UTF-8, no truncation, correct allocator pairing).
