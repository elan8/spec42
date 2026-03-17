# Gap Analysis: Replacing kerml-parser with elan8/sysml-parser

> **Status:** Migration complete. kerml-parser has been removed. The server uses sysml-parser (in-repo at `sysml-parser/`). The validation suite has been migrated to sysml-parser.

This document compares the in-repo **kerml-parser** (removed) with the external **[elan8/sysml-parser](https://github.com/elan8/sysml-parser)** and outlines what would need to change to make the replacement feasible.

---

## 1. High-level comparison

| Aspect | kerml-parser (in-repo) | elan8/sysml-parser |
|--------|------------------------|--------------------|
| **Parser engine** | Pest 2.x (grammar.pest) | nom 7 + nom_locate |
| **Root type** | `SysMLDocument` (imports + packages) | `RootNamespace` (elements) |
| **Position model** | `SourcePosition` / `SourceRange` (line, character, length) | `Span` (offset, line, column, len) + `Node<T>` wrapper |
| **Structure** | Flat `doc.imports` + `doc.packages`; packages have `members: Vec<Member>` | `RootNamespace.elements: Vec<Node<PackageBodyElement>>` (packages, imports, part defs, etc. are siblings) |
| **Serde** | Yes (Serialize/Deserialize on AST) | No |
| **Error** | `ParseError` (PestError with optional LSP position, Message, Io) | `ParseError` (message, optional offset/line/column) |

---

## 2. API surface the server depends on

### 2.1 Parsing entry points

- **`kerml_parser::parse_sysml(source: &str) -> Result<SysMLDocument>`**  
  Used everywhere: main.rs, language.rs, semantic_model.rs, model.rs.  
  **elan8:** `sysml_parser::parse(input) -> Result<RootNamespace, ParseError>`.  
  **Gap:** Different return type; no `SysMLDocument`. Need an adapter that maps `RootNamespace` → something the server can use, or change all server code to work with `RootNamespace`.

- **`kerml_parser::parse_sysml_collect_errors(text: &str) -> (Result<SysMLDocument>, Vec<ParseError>)`**  
  Used in main.rs for diagnostics (multiple parse errors per file).  
  **elan8:** Only `parse()`; no “collect multiple errors” API.  
  **Gap:** elan8/sysml-parser does not provide error recovery / multi-error collection. We would need to implement similar logic on top of nom (mask error region and re-parse), or only report the first error.

### 2.2 Document / root structure

- **`SysMLDocument`**  
  - `imports: Vec<Import>`  
  - `packages: Vec<Package>`  

- **`Package`**  
  - `name`, `name_position`, `range`, `is_library`, `imports`, **`members: Vec<Member>`**

- **`RootNamespace` (elan8)**  
  - `elements: Vec<Node<PackageBodyElement>>`  
  - `PackageBodyElement` = Package | Import | PartDef | PartUsage | PortDef | InterfaceDef | AliasDef | AttributeDef | ActionDef | ActionUsage  

**Gap:**  
- elan8 has no top-level “document” with separate imports and packages; imports and packages are siblings in `elements`.  
- elan8’s `Package` has `identification: Identification`, `body: PackageBody` (Semicolon | Brace { elements }). So “members” are inside `body`, and the element types inside a package body are again `PackageBodyElement` (not a single `Member` enum).  
- We need either a **conversion layer** RootNamespace → SysMLDocument-like shape, or we need to **refactor the server** to work with RootNamespace + PackageBodyElement (and possibly a unified “member” view built from that).

---

## 3. AST types the server uses (and elan8 equivalents)

### 3.1 Used in server

From grep and code inspection the server uses:

- **`SysMLDocument`** – root
- **`Package`** – name, members
- **`Member`** – big enum: PartDef, PartUsage, PortDef, PortUsage, InterfaceDef, ConnectionUsage, ItemDef, ItemUsage, RequirementDef, RequirementUsage, AttributeDef, AttributeUsage, ActionDef, Package, DocComment, BindStatement, AllocateStatement, ProvidesStatement, RequiresStatement, InStatement, EndStatement, StateDef, ExhibitState, TransitionStatement, UseCase, ActorDef
- **`PartDef`** – name, name_position, range, is_abstract, specializes, type_ref, multiplicity, ordered, metadata, members
- **`PartUsage`** – name (Option), name_position, range, specializes, type_ref, multiplicity, ordered, redefines, subsets, value, metadata, members
- **`PortDef`** / **`PortUsage`** – name, positions, type_ref, specializes, metadata, members
- **`AttributeDef`** / **`AttributeUsage`** – name, type_ref, positions, visibility, etc.
- **`InterfaceDef`** – name, members
- **`ConnectionUsage`** – name, source, target
- **`ItemDef`** / **`ItemUsage`**
- **`RequirementDef`** / **`RequirementUsage`**
- **`ActionDef`** – body: **`Vec<Statement>`**
- **`Statement`** – Assignment, Call, PerformAction
- **`Expression`** – Literal, Variable, FunctionCall, ValueWithUnit, QualifiedName, Index
- **`SourcePosition`** / **`SourceRange`** – line, character, length (0-based)
- **`SemanticRole`** – Type, Namespace, Class, Interface, Property, Function
- **`collect_semantic_ranges(doc) -> Vec<(SourceRange, SemanticRole)>**
- **`collect_type_ref_ranges(doc) -> Vec<SourceRange>`**
- **`ParseError::position() -> Option<(u32, u32)>`** for LSP diagnostics

### 3.2 elan8 AST (from their ast.rs)

- **RootNamespace**, **Package**, **PackageBodyElement** (Package, Import, PartDef, PartUsage, PortDef, InterfaceDef, AliasDef, AttributeDef, ActionDef, ActionUsage).
- **PartDef** – identification, specializes, body (PartDefBody with elements: AttributeDef, PortUsage).
- **PartUsage** – name, type_name, multiplicity, ordered, subsets, body (PartUsageBody with AttributeUsage, PartUsage, PortUsage, Bind, InterfaceUsage, Connect).
- **PortDef** / **PortUsage**, **InterfaceDef**, **AttributeDef** / **AttributeUsage**, **ActionDef** / **ActionUsage**, **Bind**, **Connect**, **Flow**, **FirstStmt**, **MergeStmt**, etc.
- **Expression** – LiteralInteger, LiteralReal, LiteralString, LiteralBoolean, FeatureRef, MemberAccess, Index, Bracket, LiteralWithUnit.
- **Span** – offset, line, column, len (no Serde).
- **Node&lt;T&gt;** – span + value (no Serde).
- **No** RequirementDef/RequirementUsage, StateDef, ExhibitState, TransitionStatement, UseCase, ActorDef, DocComment, ConnectionUsage (they have Connect/ConnectStmt), AllocateStatement, ProvidesStatement, RequiresStatement, InStatement, EndStatement in the same form. Some may exist under different names (e.g. EndDecl, RefDecl).

**Gaps:**

1. **Position model**  
   - kerml: 0-based line/character + length; direct `SourceRange`.  
   - elan8: `Span` (offset, line, column, len); everything wrapped in `Node<T>`.  
   We need a **conversion**: Span ↔ (line, character, length) and optionally to a single `SourceRange` type the server uses.

2. **No `Member` enum**  
   elan8 uses different enums at different levels (PackageBodyElement, PartDefBodyElement, PartUsageBodyElement, etc.). The server’s single `Member` enum and recursive `members: Vec<Member>` don’t exist. We’d need a **unified “member” view** or to change the server to walk elan8’s structure (multiple enums and body types).

3. **Missing or different constructs**  
   - **RequirementDef / RequirementUsage** – not present in elan8 ast.rs.  
   - **StateDef, ExhibitState, TransitionStatement** – not present.  
   - **UseCase, ActorDef** – not present.  
   - **DocComment** – not present.  
   - **ConnectionUsage** (source/target strings) – elan8 has Connect/ConnectStmt with `from`/`to` as `Node<>`; structure differs.  
   - **AllocateStatement, ProvidesStatement, RequiresStatement** – not in elan8 ast; may need to be added there or emulated.  
   - **InStatement, EndStatement** – elan8 has EndDecl, RefDecl; naming and structure differ.

4. **Action body / Statement**  
   - kerml: `ActionDef.body: Vec<Statement>` with Statement = Assignment | Call | PerformAction; Expression with Literal, Variable, FunctionCall, etc.  
   - elan8: ActionDef has `body: ActionDefBody` (InOutDecl elements); ActionUsage has `body: ActionUsageBody` with InOutDecl, Bind, Flow, FirstStmt, MergeStmt, ActionUsage. So no direct “list of statements” like kerml. **model.rs** (activity/sequence diagram extraction) depends on walking `action.body` (Statements). We’d need to map elan8’s action body model to something with “statements” or refactor that code.

5. **Semantic highlighting**  
   - kerml: `collect_semantic_ranges(doc)` and `collect_type_ref_ranges(doc)` returning (SourceRange, SemanticRole).  
   - elan8: No such API. We’d need to **implement** semantic range collection by walking the elan8 AST and mapping Spans to (range, role), and to define SemanticRole (or reuse the server’s enum).

6. **ParseError**  
   - kerml: `ParseError::position() -> Option<(u32, u32)>` (0-based line, character).  
   - elan8: `ParseError` has optional offset, line (1-based?), column. We need to **expose** a 0-based (line, character) for LSP and possibly adapt Display.

---

## 4. What to add to make replacement feasible

### Option A: Adapter in sysml-language-server (keep server AST-centric)

1. **Add dependency**: `sysml-parser = { git = "https://github.com/elan8/sysml-parser" }` (or publish to crates.io).
2. **Conversion layer (new crate or module)**:
   - **RootNamespace → SysMLDocument-like**  
     - Flatten `elements` into top-level imports + packages (e.g. collect consecutive Import into `doc.imports`, wrap Package and other top-level elements into a single or multiple packages with `members`).
   - **Map elan8 types → current server types**  
     - PartDef, PartUsage, PortDef, PortUsage, AttributeDef, AttributeUsage, InterfaceDef, ActionDef, etc.: field-by-field, converting `Node<T>` → (value, SourcePosition/SourceRange) and building a `Member` enum.
   - **Span → SourcePosition / SourceRange**  
     - Use span.line, span.column, span.len to build 0-based (line, character) and SourceRange.
3. **Semantic ranges**  
   - Implement `collect_semantic_ranges(root: &RootNamespace) -> Vec<(SourceRange, SemanticRole)>` (and optionally `collect_type_ref_ranges`) by walking elan8 AST and classifying identifiers (namespace, type, class, property, function, etc.).
4. **Errors**  
   - Wrap or adapt elan8’s `ParseError` to expose `position() -> Option<(u32, u32)>` (0-based).  
   - Implement **parse_sysml_collect_errors** in the server or adapter: on parse failure, use error offset/line to mask region and re-call `parse()` to collect more errors (similar to current kerml logic).
5. **Missing constructs**  
   - Where elan8 has no equivalent (RequirementDef/Usage, StateDef, UseCase, Actor, DocComment, AllocateStatement, Provides/RequiresStatement, etc.), either:
     - Add them to elan8/sysml-parser and upstream, or  
     - Represent them as “unknown” or omit them in the adapter and accept limited functionality for those constructs.

6. **Action body / model.rs**  
   - Map elan8’s ActionDefBody / ActionUsageBody (InOutDecl, Bind, Flow, FirstStmt, MergeStmt, ActionUsage) into a list of “statements” or a small internal enum so that `extract_activity_diagrams` and sequence extraction can still work, or refactor those functions to work directly on elan8’s action body.

### Option B: Contribute to elan8/sysml-parser and refactor server

1. **Upstream (elan8/sysml-parser)**:
   - Add **RequirementDef/RequirementUsage**, **StateDef**, **ExhibitState**, **TransitionStatement**, **UseCase**, **ActorDef** if the grammar supports them.
   - Add **AllocateStatement**, **ProvidesStatement**, **RequiresStatement**, **InStatement**, **EndStatement** (or align with EndDecl/RefDecl).
   - Add **DocComment** and **ConnectionUsage**-style connection if needed.
   - Expose **0-based** line/character in errors and optionally a `position() -> (line, char)` for LSP.
   - Add **collect_semantic_ranges** (and optionally **collect_type_ref_ranges**) in the crate, returning a simple (range, role) type.
   - Consider **parse_collect_errors** that returns `(Result<RootNamespace>, Vec<ParseError>)` with error recovery.
   - Optionally add **Serde** to AST types for debugging/API use.

2. **Server refactor**:
   - Replace dependency on kerml-parser with sysml-parser.
   - Change all `SysMLDocument`/`Package`/`Member` usage to **RootNamespace** and elan8’s enums (PackageBodyElement, PartDefBodyElement, etc.), or introduce a minimal “view” type that the server uses and that is built from RootNamespace.
   - Use elan8’s **Span** and **Node<T>** everywhere for positions, or convert at the boundary to the server’s range type.
   - Use elan8’s **Expression** and action body structures in **model.rs** (activity/sequence) instead of Statement/Call/Assignment.

---

## 5. Recommendation summary

- **Minimal path (Option A)**  
  - Add **sysml-parser** as dependency.  
  - Implement an **adapter** that converts **RootNamespace** → **SysMLDocument** (and **Member**-style tree) and **Span** → **SourceRange**; implement **collect_semantic_ranges** and **collect_type_ref_ranges** on the elan8 AST; adapt **ParseError** and implement **parse_sysml_collect_errors** on top of `parse()`.  
  - For constructs elan8 doesn’t have (requirements, state machines, use cases, actors, allocate/provides/requires, doc comments), either extend elan8 or leave those as “unsupported” in the adapter and accept reduced feature set for those.

- **Larger path (Option B)**  
  - Contribute missing AST nodes and APIs to elan8/sysml-parser, then refactor the server to use **RootNamespace** and elan8 types directly (or a thin view layer). This reduces duplication and keeps a single AST definition.

- **Critical gaps to resolve either way**  
  1. Document/root shape (RootNamespace vs SysMLDocument).  
  2. Position model (Span vs SourceRange; 0-based for LSP).  
  3. Single **Member**-like recursion vs multiple body element enums.  
  4. Semantic range collection API.  
  5. Multi-error collection for diagnostics.  
  6. Action body and **Statement**/Expression for activity/sequence extraction.  
  7. Missing constructs (requirements, states, use cases, actors, allocate/provides/requires, etc.) if we need full feature parity.

Once the chosen option is implemented, we can remove **kerml-parser** from the workspace and point the server at **sysml-parser** plus the adapter or refactored code.
