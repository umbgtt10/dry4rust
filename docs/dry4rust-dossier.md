# `dry4rust` — Pre-Project Research Dossier

*Everything worth having on the table before writing line one of a best-in-class
Rust duplication / anti-copy tool.*

---

## 0. The uncomfortable starting fact

A competent, recent, AST-based Rust duplication tool **already exists**:
`cargo-dupes` (crate `code-dupes`, published February 2026). It parses Rust with
`syn`, normalizes functions/methods/closures (positional placeholders for
identifiers, literals erased but types preserved), fingerprints for exact matches,
and uses the Dice coefficient for near-duplicates. It has CI subcommands,
percentage thresholds, test exclusion, and documented fingerprint suppression.

It was written for exactly your reason — LLM coding agents emit duplicate code —
and its author explicitly rejected `rust-code-analysis`, `jscpd` and SonarQube as
not feeling right in a Rust codebase.

**Consequence:** a token- or AST-fingerprint duplication gate for Rust is no
longer greenfield. Building another one is a me-too exercise. This dossier is
therefore organized around a single question — *where is the ground `cargo-dupes`
and the generic tools do not cover?* — because that is the only place "best
possible" means anything.

The short answer, argued in §10: **semantic redundancy detection tuned for the
agent loop.** Not "find historical copy-paste" (solved) but "tell me this function
already exists before I let an agent write it again."

---

## 1. Prior art

### 1.1 Rust-specific

| Tool | Approach | Granularity | Status / niche |
|---|---|---|---|
| **`cargo-dupes` / `code-dupes`** | `syn` AST normalization + fingerprint; Dice for near-dup | fn / method / closure | Feb 2026. The direct incumbent. CI-ready, threshold gate, suppression. |
| **`similarity-rs`** | code-similarity CLI | fn-level | April 2026. Overlapping intent; worth reading before building. |
| **`rust-code-analysis` (Mozilla)** | tree-sitter, multi-language metrics | metrics, not clone-first | Mature, general static analysis; clone detection is not its focus. |
| **`duplicate-function-checker` (Lattimore)** | binary symbol normalization | monomorphized fn in binary | Different problem entirely: measures *compiled* bloat from monomorphization, x86_64/Linux. |
| **`duplihere`** | copy/paste in structured text | text blocks | Language-agnostic text, not Rust-aware. |
| `duplicate_code`, `duplicate-checker`, `SmartShreds` etc. | file hashing (SHA-256) | whole file | *File* duplicate finders, not code-clone tools. Ignore for this purpose. |
| `duplicate` (crate) | proc-macro for intentional duplication | — | Opposite purpose; named confusingly. |

**Read before building:** `cargo-dupes` (its blog series documents the exact
normalization decisions), `similarity-rs`, and `rust-code-analysis` for its
tree-sitter grammar handling.

### 1.2 Cross-language / industry

| Tool | Approach | Notes |
|---|---|---|
| **jscpd** | token-based, % duplication, CI gate | The de-facto modern standard. ~5k stars. Best reference for gate ergonomics: threshold, report format, baseline. Tokenizes Rust adequately — which is *why* a token-level `dry4rust` adds little. |
| **PMD CPD** | token-based | Veteran, huge language list, the original "fail build past N tokens." |
| **Simian** | token/line-based | Commercial, multi-language. |
| **SonarQube / SonarCloud** | token + metrics | Enterprise gate; duplication is one axis among many. |

### 1.3 Research-grade (the frontier tools)

| Tool | Representation | Catches |
|---|---|---|
| **CCFinder / CCFinderX** | token | Type 1–2, the research reference implementation. |
| **NiCad** | AST + normalization | Type 1–3 near-miss; strong on parameterized/near clones. |
| **Deckard** | AST → characteristic vectors | Type 1–3 at scale via vector clustering. |
| **SourcererCC** | token + index | Scales to very large corpora. |
| **CCAligner, CCGraph** | token / PDG | Alignment-based and graph-based variants. |
| **ML / DL / LLM** (ASTNN, CodeBERT, GPT-based) | learned embeddings / execution | The only class that reaches Type 4 (semantic). Precision/scale still open. |

---

## 2. The conceptual spine: clone taxonomy

Every design decision below reduces to *which of these you target*. The
four-type taxonomy is the field's shared vocabulary:

- **Type 1 — textual.** Identical but for whitespace, comments, layout. Trivial.
- **Type 2 — lexical / parameterized.** Type 1 plus renamed identifiers, changed
  literals and types. Easy with normalization (this is what placeholder
  substitution buys you).
- **Type 3 — near-miss / syntactic.** Type 2 plus added, removed, modified or
  reordered statements. Sub-divided in benchmarks into **strong / moderate /
  weak** T3 by decreasing similarity. *This is where refactoring value lives* and
  where difficulty starts.
- **Type 4 — semantic.** Syntactically dissimilar, functionally equivalent. The
  understudied, largely-open problem: token/AST/PDG methods score near-zero here;
  only ML/execution-based approaches make progress.

Where the tools land: token/AST/PDG reliably catch **Type 1–3**; **Type 4** is a
research frontier requiring learned embeddings or dynamic execution, with
precision, scale and even *definitional* clarity still contested.

**The strategic read:** `cargo-dupes` covers Type 1–2 and is reaching into Type 3
(Dice near-dup). Competing there is a marginal-improvement game. Type 4 is where
nobody has a production Rust tool — and, not coincidentally, where the
agent-redundancy problem actually lives (an agent that reimplements a function
from scratch produces a *Type 4* clone, not a copy-paste).

---

## 3. The core open questions

These are the decisions that define the tool. None has a default-correct answer;
each is a genuine fork.

### 3.1 What is the single number the gate fires on?

`crap4rust` works because CRAP is one defensible scalar per function. Duplication
has *no* canonical scalar, so you must choose one, and the choice is the tool:

- **% duplicated lines** (jscpd) — intuitive, but line-based and gameable by
  reformatting, and says nothing about *where* the refactor is.
- **% duplicated AST nodes / tokens** — reformat-proof, but a percentage still
  hides actionability.
- **Count of clone groups above a size threshold** — actionable (each group is a
  refactor), but not a ratio, so hard to compare across repos.
- **Largest clone / clone-mass distribution** — surfaces the worst offender, but
  a single number loses the tail.

Open question: is the gate a **ratio** (portfolio health) or a **count of
findings** (a work list)? They imply different tools. For a ratchet gate
(§3.5), a *count of new groups* is often more honest than a drifting percentage.

### 3.2 Granularity — what is a unit?

- **Whole function / method / closure** (cargo-dupes) — clean boundaries, easy
  report, misses sub-function duplication.
- **Statement sequence / block** — catches the repeated block inside two otherwise
  different functions, which is the higher-value and harder case. Requires
  windowing and alignment; explodes candidate count.
- **Sub-tree of arbitrary size** — most general (Deckard-style), most expensive,
  hardest to report actionably.

The interesting duplication in mature Rust is usually **sub-function** — a
repeated match-arm shape, an error-mapping block, a validation preamble. Whole-
function detection is the easy 80% that is already done.

### 3.3 Normalization aggressiveness

A dial, not a setting. Too aggressive and genuinely distinct functions that share
structure collapse into false positives; too little and Type 2 slips through.

- Identifiers → positional placeholders? (yes, standard)
- Literals → erased, type-preserved? (cargo-dupes does this; but `0` vs `1` as a
  base case can be *semantically* load-bearing)
- Types → erased or preserved? (erasing merges `Vec<u8>`/`Vec<u32>` handlers —
  sometimes a real duplicate, sometimes the whole point of a generic that *should*
  exist)
- Control-flow → normalized (e.g. `while`↔`for`, `if-else`↔`match`)? Reaches into
  Type 3 but multiplies false positives fast.

Every notch toward abstraction trades precision for recall. The right default is
an empirical question you cannot answer from the armchair — it needs a labeled
Rust corpus (§6).

### 3.4 Which clone types are in scope?

The single most consequential scoping decision. Recommended honest position:

- **Type 1–2:** table stakes, already commoditized. Include, don't innovate.
- **Type 3:** the CI-gate sweet spot *if* false positives are controlled. This is
  the defensible competitive ground for a batch tool.
- **Type 4:** out of scope for a *gate* (too noisy, too slow), but the entire
  point for an *agent assistant* (§10). Do not try to make one tool do both.

### 3.5 Batch-CI vs ratchet vs agent-loop

Three different products wearing the same name:

- **Batch/CI gate** — run in CI, fail past threshold. What cargo-dupes and jscpd
  are. Crowded.
- **Ratchet gate** — like `crap4rust`'s ethos: *don't add new duplication*, ignore
  the existing debt. Baseline + diff. More adoptable in real codebases; still
  batch.
- **Agent-loop / editor** — runs on every agent turn or save, fast and
  incremental, answering "does this already exist?" *before* the code lands. This
  is the unoccupied, high-value mode (§10) and it changes every other decision
  (latency budget, incrementality, semantic reach).

---

## 4. Rust-specific challenges (where generic tools break)

This is the section a generic tokenizer cannot help with, and where a Rust-native
tool earns its keep — or drowns in false positives.

- **Macros, declarative and procedural.** Do you analyze pre- or post-expansion?
  Pre-expansion, a `macro_rules!` that generates ten near-identical impls looks
  like ten clones that are actually *one* deliberate abstraction. Post-expansion,
  everything a macro generates looks duplicated. Both are wrong by default; you
  need a policy, and it is not obvious.
- **Generics and monomorphization.** The existence of a generic is often the
  *cure* for duplication. Flagging two type-specialized functions as clones can be
  actively harmful advice if the correct fix is a generic they already avoided for
  a reason (object safety, coherence, performance).
- **Trait impls.** Idiomatic Rust is full of structurally identical impls
  (`From`, `Display`, `Ord`, newtype forwarding). These are *required* boilerplate,
  not refactorable duplication. A tool that flags them fights the language and
  gets uninstalled.
- **Derive macros.** `#[derive(...)]`-generated code must be excluded or it swamps
  everything.
- **`cfg` conditional compilation.** The same function under `#[cfg(feature=...)]`
  twice is intentional and correct. Flagging cfg-variant duplication is a false
  positive by construction.
- **Error-handling boilerplate.** `?`, `map_err(...)`, match-on-`Result` chains
  repeat constantly and are mostly not worth extracting. High false-positive
  density.
- **`match` arm shapes.** A rich source of genuine near-duplication *and* of
  unavoidable structural repetition. Distinguishing the two is the hard part.

The through-line: **Rust has a high rate of intentional, correct structural
repetition.** A tool tuned on Java/JS false-positive rates will be unusable here.
This is precisely why cargo-dupes' author found generic tools "not right."

---

## 5. The false-positive problem (its own section, because it kills these tools)

In Rust specifically, the failure mode is not missed clones — it is *crying wolf*.
Every idiom in §4 is a false-positive generator. A duplication tool's adoption is
governed almost entirely by its false-positive rate on idiomatic code, because a
gate that flags required trait impls gets switched off within a day.

Design implications:

- **Suppression must be first-class**, documented, and diff-reviewable
  (cargo-dupes' fingerprint-ignore-with-reason is the right shape; borrow it).
- **Idiom-awareness beats raw structural matching.** Knowing that a block is a
  trait impl, a derive, or a cfg variant — and discounting it — is worth more than
  a cleverer similarity metric.
- **Baseline / ratchet** sidesteps the debt-flagging problem: only new duplication
  is actionable, so the tool never nags about the existing idiomatic repetition.
- **Precision over recall, decisively.** A gate that catches 60% of real
  duplication with near-zero false positives is adopted; one that catches 95% with
  a 20% false-positive rate is deleted. This is the opposite of the research-
  benchmark incentive (which rewards recall), and it is the single most important
  product instinct here.

---

## 6. Benchmarks and ground truth

You cannot tune §3.3 or measure §5 without labeled data, and here Rust is
impoverished:

- **BigCloneBench** — the standard clone benchmark, but Java-only, and criticized
  for imbalance and an ambiguous Type-4 definition that misleads ML training.
- **GPTCloneBench / SemanticCloneBench** — GPT-generated semantic and
  cross-language clones; again not Rust.
- **No established Rust clone benchmark exists.** This is a gap *and* an
  opportunity: a labeled Rust clone corpus (especially one of *agent-generated*
  redundancy) would itself be a citable contribution and the evaluation substrate
  for the tool. Consider building it as deliverable zero.

Practical bootstrap: mine your own history. You have four repos that reached
`verdict=clean`, and a documented pattern of agents producing duplication. That
is a private, honest, Rust-native, agent-relevant labeled set nobody else has.

---

## 7. Algorithmic / representation trade-offs

| Representation | Catches | Cost | False-positive risk in Rust |
|---|---|---|---|
| **Line / text** | T1 | trivial | high (formatting-sensitive) |
| **Token** | T1–2 | low, O(n) with index | medium; blind to structure |
| **AST (`syn`)** | T1–3 | moderate | medium; the cargo-dupes tier |
| **AST + normalization + alignment** | strong T3 | higher (near-pairwise) | tunable |
| **PDG (program dependence graph)** | reordered T3, some T4 | high; hard to build for Rust | lower structurally, but heavy |
| **Learned embedding** (CodeBERT-class) | T4 | high; model + infra | different failure modes (opaque FPs) |
| **Dynamic / execution** | T4 | very high; needs runnable units | impractical as a gate |

Scale note: exact detection via fingerprint hashing is O(n). **Near-duplicate
detection is the O(n²) trap** — naive pairwise comparison across a large workspace
is quadratic. Mitigations: LSH / MinHash bucketing, characteristic-vector
clustering (Deckard's trick), or fingerprint-prefix indexing to prune candidates
before expensive alignment. Decide this early; it constrains the near-dup design.

---

## 8. Actionability and output

A number is not a tool; the refactor is the value — and the *wrong* refactor
suggestion is worse than silence (see §4 generics).

- **Group + locate** (cargo-dupes does this): clone group, member locations,
  fingerprint. The minimum.
- **Suggest the mechanism**: extract-function vs introduce-generic vs macro vs
  trait-default. High value, high risk — suggesting "make this generic" where a
  generic was deliberately avoided is actively bad. Suggest conservatively or not
  at all.
- **Machine-readable output** (JSON) for the agent-loop and CI consumption.
- **Stable fingerprints** across runs so suppression and baselines survive edits —
  this is a design constraint on the fingerprint scheme, not an afterthought.

---

## 9. Scale, performance, incrementality

- **Workspace / cross-crate** is the interesting scope for a monorepo; single-file
  duplication is the boring case. Cross-crate duplication in a workspace is where
  real redundancy hides.
- **Incremental analysis** is mandatory for the agent-loop mode: re-analyzing the
  whole tree on every turn is too slow. Needs per-file fingerprint caching keyed
  on content hash, and a persisted fingerprint index.
- **Latency budget** differs by 3+ orders of magnitude between CI (seconds-to-
  minutes acceptable) and agent-loop (sub-second to be worth running inline). One
  codebase probably cannot serve both without a fast incremental core and a
  batch wrapper over it.

---

## 10. The strategic wedge — where "best possible" actually lives

Given §0, competing on batch AST duplication is a marginal game. The unoccupied,
defensible, and *self-relevant* ground:

**An agent-oriented semantic-redundancy detector.**

The insight: when an LLM agent reimplements an existing function because it wasn't
in context, the result is **not a copy-paste (Type 1–3) — it's a Type 4 semantic
clone**. Structurally different, functionally identical. Every existing Rust tool,
cargo-dupes included, is blind to it, because they detect syntactic similarity and
this is semantic. The very failure mode that motivates these tools is the one they
cannot catch.

A tool aimed there would:

- Detect **functional** redundancy, not just structural — "you have two functions
  that *do the same thing* written differently." This needs embeddings or an
  LLM-in-the-loop, accepting that it is a *suggestion* engine, not a hard gate.
- Run **in the agent loop or as an MCP tool**: before/while an agent writes a
  function, answer "does an equivalent already exist as `foo::bar`?" — turning
  anti-copy from post-hoc detection into pre-hoc prevention.
- Maintain a **semantic index of the existing codebase** (embeddings of functions
  by behavior) that the agent queries, directly attacking the root cause (missing
  context) rather than the symptom (duplicate output).
- Stay **Rust-native and idiom-aware** so it discounts the intentional repetition
  of §4 instead of drowning in it.

This is coherent with your broader thesis about agentic overproduction: the
bottleneck in agent-driven development is redundant regeneration under limited
context, and the tool that prevents it — rather than merely auditing for it after
the fact — is the one worth building. It is also a natural RTAS-adjacent or blog-
adjacent artifact, and it is genuinely novel: nobody has a production Rust
semantic-redundancy tool wired into the agent loop.

**Caveat, stated plainly:** this is materially harder than a `syn` fingerprint
gate. Type 4 detection is an open research problem with unsolved precision and no
Rust benchmark. LLM/embedding approaches bring opaque false positives and infra
cost. The honest framing is a *ranked suggestion* engine with human/agent
confirmation, never a silent hard gate — the same discipline as your other
tooling.

---

## 11. Targets and success criteria

A best-in-class tool should be judged against these, not against raw recall:

1. **False-positive rate on idiomatic Rust** — the adoption-governing metric.
   Target: near-zero flags on required trait impls, derives, cfg variants.
2. **Actionability** — every finding maps to a specific, *correct* refactor, or is
   not shown.
3. **Latency fit** — sub-second incremental for agent-loop; whole-workspace in
   CI-acceptable time for batch.
4. **Stability** — fingerprints and suppressions survive unrelated edits.
5. **Idiom-awareness** — the tool understands Rust's intentional repetition rather
   than fighting it.
6. **(Wedge) Semantic reach** — catches functional redundancy that syntactic tools
   miss, at a precision high enough to be trusted as a suggestion.

Explicit non-goals worth committing to up front:
- Not a plagiarism/copyright detector (different problem, different tuning).
- Not a compiled-bloat analyzer (that's `duplicate-function-checker`'s job).
- Not a file-level dedup tool.
- Not (for the gate) a Type 4 hard gate — Type 4 is suggestion-only.

---

## 12. Decisions to make before writing line one

A checklist, each item a real fork with no default:

- [ ] **Product mode**: batch-CI gate, ratchet gate, or agent-loop assistant? (§3.5, §10)
- [ ] **Clone-type scope**: 1–2, 1–3, or 1–3 + semantic-suggestion? (§2, §3.4)
- [ ] **The gate metric**: ratio or finding-count; which ratio? (§3.1)
- [ ] **Unit granularity**: function-level or sub-function block-level? (§3.2)
- [ ] **Representation**: token / AST / AST+alignment / PDG / embedding? (§7)
- [ ] **Normalization dial defaults**: identifiers/literals/types/control-flow? (§3.3)
- [ ] **Macro policy**: pre- or post-expansion; how to treat macro-generated impls? (§4)
- [ ] **Idiom discounting**: which Rust idioms are auto-excluded? (§4, §5)
- [ ] **Suppression & baseline model**: fingerprint-ignore, ratchet, both? (§5)
- [ ] **Near-dup scaling strategy**: LSH / vectors / prefix-index to avoid O(n²)? (§7)
- [ ] **Output contract**: human report + JSON; refactor suggestions or locations-only? (§8)
- [ ] **Incrementality & caching**: content-hashed per-file fingerprint index? (§9)
- [ ] **Evaluation corpus**: build a labeled Rust (agent-generated) clone set? (§6)
- [ ] **Relationship to the family**: does it share `crap4rust`'s gate conventions
      (single scalar, ratchet, documented suppression, CI verdict)? (§0, §3.1)

---

## 13. One-paragraph recommendation

A second AST-fingerprint duplication gate for Rust is not worth building;
`cargo-dupes` already holds that ground competently and recently. The defensible,
novel, and self-consistent target is a **Rust-native, idiom-aware semantic-
redundancy detector wired into the agent loop** — one that catches the Type 4
"reimplemented the same function differently" failure that every syntactic tool
misses and that agentic development produces constantly. Scope it as a ranked
suggestion engine with confirmation, never a silent hard gate; bootstrap its
evaluation on your own clean repos; and hold it to a false-positive-first standard
rather than the recall-first standard the research benchmarks reward. If instead
the goal is only a fast batch gate to complement `crap4rust`, adopt cargo-dupes
rather than rebuild it, and spend the effort on the semantic layer no one has.
