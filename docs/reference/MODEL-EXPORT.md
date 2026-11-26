# Model export reference

Notes for tools that read the JSON written by `spec42 model-export` and `spec42 diagram`. For how to
run the commands, see [AI assistants](../user/AI-ASSISTANTS.md).

## Export contents and schema

Model-export schema 2 adds invocation, sequence and meta-cast node kinds; consumers must accept
these operations or report them as unsupported. `membership.role` identifies entry/do/exit
state subactions for both inline and reference forms. A trigger invocation's `trigger_kind`
is `when`, `at` or `after`; its `expression` tree carries the condition, instant or duration.
Read connector topology from `projection.connectors`, whose ordered `ends` retain each
feature-chain endpoint's `root`, `terminal` and full `path` with per-hop resolution outcomes.
For example, `connect left.output to right.input;` retains the `left` and `right` occurrence
roots even when both ports are inherited from shared definitions. A named end, including an
`end port` declared in the connector body, carries a `declaration` token that identifies its
entry in `projection.elements`. An unwired declared end has endpoint kind `unconnected`.
Generic element relationships alone do not describe that topology. End order does not by itself
imply directed signal flow.

Elements retain authored `name` and `qualified_name` separately from
`effective_name` and `effective_qualified_name`. Use the effective fields to find unnamed
redefinitions; `null` means no complete effective name is settled. `multiplicity_expressions`
links to the range and its authored lower/upper expression elements (a single-bound range has
only an upper expression). `exposure` retains namespace, recursive and filtered exposure flags.
`evaluation` publishes the settled status and optional value; `analysis` publishes a verdict,
computed result or an explicit unsettled status. An unsupported or nonconstant expression is
never a failing verdict. Expression nodes distinguish invocations, sequences and meta casts;
this does not imply that the evaluator executes those operations. Generator ABI v7 continues to
represent these newer operators as explicitly unsupported nodes with their operands retained.

Expression `result_typing` reports static result types separately from evaluated values.
The strict conformance diagnostic covers bound scalar datatype values and assignments; defaults
retain the existing declaration reporting policy, while structural and unit bindings need their
own intersection/measurement rules. Computed Integer results do not become Natural merely
because their value is nonnegative.
Operator signatures come from admitted standard-library specializations; named invocations
retain their declared return type; sequence size can refine to Positive when a direct feature
operand has a proven positive lower multiplicity. Positive sums and products use static
operand-domain guarantees. Unresolved, ambiguous and unsupported typings
remain explicit.
`contextual_evaluations` identifies the requested, effective and authored elements for each
inherited expression evaluated in a usage. A false contextual constraint reports at that usage
and links to its authored declaration. Derived values use the same evaluator with contextual
bindings; cycles and nonconstant inputs remain explicit. Quantity comparisons currently fold
matching unit spellings only; unit conversion and arbitrary user calculation execution remain
unsupported. Generator ABI v7 does not expose these additional static/contextual fields.

## `spec42 diagram`

`spec42 diagram` emits `warning[empty_diagram]` on stderr when a selected view has no
exposed roots. Check its expose and filter declarations: `filter @SysML::PartUsage;`, for
example, excludes an exposed part definition. A fully resolved empty selection still reports
completeness `complete`; the warning does not alter the JSON written to stdout.

## Library elements

`model-export` returns workspace elements by default. Use `--library-elements referenced` to
include the transitive closure of typing, specialization, subsetting, and redefinition targets,
and the members owned by those library definitions. Unrelated library definitions remain out of
the export. `--library-elements all` includes every admitted library and external element;
`--library-elements none` explicitly selects the default workspace scope. Standard-library
targets may appear in `referenced`, but their owned members are not expanded and `all` omits
standard-library elements. `--max-nodes` still limits the returned element count.
