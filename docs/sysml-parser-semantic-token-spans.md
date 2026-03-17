# sysml-parser: Fine-grained spans for semantic tokens

This document specifies optional span fields to add to the sysml-parser AST so that the language server can emit one (range, role) per semantic piece for LSP semantic token highlighting, instead of one role per whole node.

## Background

Today each AST node has a single `span` (on `Node<T>` or the inner struct). For usages like `part propulsion : PropulsionUnit { }`, the whole declaration is one span, so the server cannot distinguish the usage name (`propulsion`) from the type reference (`PropulsionUnit`). The language server needs:

- **Usage name** → PROPERTY (or variable) role  
- **Type reference** → TYPE role  

So the parser should expose optional sub-spans for the name and the type reference where the grammar has both.

## Span type

Use the existing `Span` type (offset, line, column, len; 1-based line/column). All new fields are **optional** (`Option<Span>`) so that:

1. Parsers can add the fields and set them to `None` until the parser is updated to fill them.
2. The language server can compile against the updated AST and use the spans when present.

## Structs and fields to add

Add these fields to the **inner** structs (the `T` in `Node<T>`). The node’s existing `.span` remains the span of the whole construct.

### PartUsage

**Syntax:** `part` name `:` type_name multiplicity? ordered? body

| Field           | Type         | Description |
|----------------|--------------|-------------|
| `name_span`    | `Option<Span>` | Span of the usage name (identifier after `part`). |
| `type_ref_span`| `Option<Span>` | Span of the type reference (identifier or qualified name after `:`). |

### PortUsage

**Syntax:** `port` name `:` type? multiplicity? `:>` subsets? redefines? body

| Field           | Type         | Description |
|----------------|--------------|-------------|
| `name_span`    | `Option<Span>` | Span of the usage name (identifier after `port`). |
| `type_ref_span`| `Option<Span>` | Span of the type reference after `:`, if present. |

### AttributeUsage

**Syntax:** `attribute` name redefines? value? body

| Field    | Type         | Description |
|----------|--------------|-------------|
| `name_span`     | `Option<Span>` | Span of the usage name (identifier after `attribute`). |
| `redefines_span`| `Option<Span>` | Span of the redefines target (qualified name after `redefines`), if present. Optional; name + type_ref are the priority for semantic tokens. |

### ActionUsage

**Syntax:** `action` name `:` type_name (`accept` param_name `:` param_type)? body

| Field           | Type         | Description |
|----------------|--------------|-------------|
| `name_span`    | `Option<Span>` | Span of the usage name (identifier after `action`). |
| `type_ref_span`| `Option<Span>` | Span of the type reference after `:`. |

### AttributeDef (optional but useful)

**Syntax:** `attribute` name (`:>` type)? body

| Field           | Type         | Description |
|----------------|--------------|-------------|
| `name_span`    | `Option<Span>` | Span of the defined name. |
| `typing_span`  | `Option<Span>` | Span of the type after `:>`, if present. |

### EndDecl / RefDecl (interface body)

**EndDecl:** `end` name `:` type `;`  
**RefDecl:** `ref` name `:` type body  

| Struct   | Field           | Type         | Description |
|----------|-----------------|--------------|-------------|
| EndDecl  | `name_span`     | `Option<Span>` | Span of the name. |
| EndDecl  | `type_ref_span` | `Option<Span>` | Span of the type after `:`. |
| RefDecl  | `name_span`     | `Option<Span>` | Span of the name. |
| RefDecl  | `type_ref_span` | `Option<Span>` | Span of the type after `:`. |

## Parser implementation notes

- When building each node, after parsing the name token and the type token (if any), record their source positions (offset, line, column, length) and construct a `Span` for each. Store them in the new optional fields.
- Use the same input/location type you already use for the node’s main span (e.g. from `nom_locate::LocatedSpan` or equivalent) so that line/column and offset are consistent.
- If a field is not yet implemented, use `None`. The language server will still work and will use these spans when they are `Some`.

## Consumer (language server)

The sysml-language-server will:

- For PartUsage, PortUsage, AttributeUsage, ActionUsage: if `name_span` is `Some`, push (name_span, PROPERTY); if `type_ref_span` is `Some`, push (type_ref_span, TYPE). It will **not** push the whole node span with a single role for these usages when fine-grained spans are present.
- For AttributeDef, EndDecl, RefDecl: use name_span and typing_span/type_ref_span when present to emit separate ranges for name vs type.
- Continue to use the node’s main `.span` for definitions (PartDef, PortDef, etc.) where a single span per node is still appropriate, unless/until the parser adds name-only spans for defs.

## Summary table

| AST struct   | New fields                                      |
|-------------|--------------------------------------------------|
| PartUsage   | name_span, type_ref_span                         |
| PortUsage   | name_span, type_ref_span                         |
| AttributeUsage | name_span, redefines_span (optional)           |
| ActionUsage | name_span, type_ref_span                         |
| AttributeDef | name_span, typing_span (optional)              |
| EndDecl     | name_span, type_ref_span                         |
| RefDecl     | name_span, type_ref_span                         |

All fields: `Option<Span>`, default `None` until the parser fills them.

## Language server integration

After the parser exposes these fields, the sysml-language-server will use them in `server/src/semantic_tokens.rs`. The collectors already have placeholder comments at each usage site; replace them with:

- **PartUsage** (package body and part usage body):  
  `if let Some(ref s) = pu_node.value.name_span { out.push((span_to_source_range(s), TYPE_PROPERTY)); }`  
  `if let Some(ref s) = pu_node.value.type_ref_span { out.push((span_to_source_range(s), TYPE_TYPE)); }`  
  and remove any fallback that pushes the whole node span.

- **PortUsage** (part def body, part usage body, port def body):  
  Same pattern with `n.value.name_span` and `n.value.type_ref_span`.

- **ActionUsage** (package body):  
  Same with `au_node.value.name_span` and `au_node.value.type_ref_span`.

- **AttributeUsage** (part usage body):  
  `if let Some(ref s) = n.value.name_span { out.push((span_to_source_range(s), TYPE_PROPERTY)); }`.

Until the parser adds the fields, the server does not push any range for these usage nodes, so the lexer’s heuristics (e.g. identifier after `:` = TYPE) apply and type references are not overridden to PROPERTY.
