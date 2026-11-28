# OMG corpus diagnostic baseline

Nightly checks the `sysml/src/validation`, `sysml/src/examples`, and `sysml/src/training` trees from the OMG release pinned in `config/standard-library.json`. Each tree is checked as a workspace with the pinned standard library. The KerML examples appear in the report and are outside the gate.

The baseline identifies each finding by corpus, relative file, severity, stable code, and complete source range. It preserves repetitions. The report must account for every tracked model file in the clean pinned checkout. Nightly uploads the full report before comparing it with the baseline. A new occurrence or a resolved one requires review of the baseline change, so an earlier fix cannot hide a later recurrence.

Prepare the pinned library bundle and checkout, then reproduce the report locally.

```bash
cargo build --locked -p server --bin spec42
node scripts/report-omg-corpus.mjs target/debug/spec42 path/to/SysML-v2-Release .cache/omg-corpus-report
node scripts/omg-corpus-baseline.mjs config/omg-corpus-baseline.json .cache/omg-corpus-report/diagnostics.json
```

After reviewing the reported changes, regenerate the baseline with the same comparison command followed by `--record`. Include the reason for each accepted addition in the pull request and remove occurrences fixed by the change. A pin update requires a fresh review of the new corpus.

## Findings retained in the baseline

The inventory includes findings on examples as authored, incomplete semantic coverage, and diagnostics under investigation. It records them so that work on one code cannot mask a regression at another site.

| Codes | Evidence and disposition |
| --- | --- |
| `incompatible_quantity_dimensions` | `Dynamics.sysml:9` subtracts the product of `Cd: Real` and a speed from a power. The checker reads the declared types and reports power versus speed. SysML v2 §9.8.9.1. |
| `incompatible_unit_dimension` | `AnalysisIndividualExample.sysml:88` assigns mph to a fuel economy result. Annex A line 1264 assigns metres to `Vehicle::velocity`. The pinned measurement types distinguish speed, distance per volume, and length. SysML v2 §9.8.9. |
| `attribute_value_type_mismatch` | `15_12-Compound Value Type.sysml:28` binds a measured value to a local definition that specializes `Real`. `TimeVaryingAttribute.sysml:41` binds a calculation over the `Real` magnitude `elapseTime.num` to an `Integer` level. The examples declare the types that give rise to these reports. SysML v2 §§7.7, 9.8. |
| `ambiguous_reference`, `duplicate_root_package` | `13a-Model Containment.sysml:21` imports two packages that expose `vehicle1_c1`. Training contains parallel variants with shared top-level package names. These findings reflect the combined workspaces checked by this job. KerML §8.3.2.4.5. |
| `implicit_redefinition_without_operator` | The training example binds actors in the specialized use case. The applicability of implied parameter redefinition is tracked in [#542](https://github.com/elan8/spec42/issues/542). SysML v2 §§7.17.2, 7.22.2, 7.25.3. |
| `recovered_requirement_body_element` | Annex A uses feature chains as `require` and `verify` targets. [#544](https://github.com/elan8/spec42/issues/544) tracks their parser recovery. SysML v2 §§8.2.2.21.1, 8.2.2.24. |
| `recovered_connection_def_body_element`, `unexpected_keyword_in_scope` | [#545](https://github.com/elan8/spec42/issues/545) tracks the compound connection end in `ConnectionTest.sysml` and the nested constraint in Annex A. SysML v2 §§8.2.2.13.1, 8.2.2.17, 8.2.2.19. |
| `recovered_package_body_element`, `recovery_cascade_suppressed`, `connection_context_invalid`, `unsupported_attribute_member` | The cause-and-effect examples and validation files exercise semantic-metadata connection forms. [#543](https://github.com/elan8/spec42/issues/543) tracks their parsing and endpoint rules. The other `unsupported_attribute_member` occurrences concern variants. SysML v2 §§8.2.2.6, 9.5. |
| `redefinition_direction_mismatch`, `flow_direction_incompatible` | The reported redefinitions include members of conjugated ports. The Annex A warning links an outgoing ignition command to its matching incoming command. [#546](https://github.com/elan8/spec42/issues/546) tracks the effective directions. KerML §8.3.3.3.8 and SysML v2 §7.12. |
| `unsupported_filtered_import` | [#547](https://github.com/elan8/spec42/issues/547) tracks supported syntax whose filtered memberships are missing from the model. SysML v2 §7.5.4. |
| `unresolved_reference`, `unresolved_type_reference` | This set mixes dotted endpoints and feature values, variability members, and nested types in `AscentSample`, `CartSample`, and `EVSample`. [#541](https://github.com/elan8/spec42/issues/541) tracks the Arrowhead reproducer. Other cases still require separate scope or coverage checks under [#425](https://github.com/elan8/spec42/issues/425). KerML §§8.3.2.4.5, 8.3.3.3.5. |
| `analysis_evaluation_unresolved` | An evaluation result explicitly reports missing operands. Variability and other contextual cases belong to [#403](https://github.com/elan8/spec42/issues/403). SysML v2 §§7.19, 7.20. |
| `connection_endpoint_not_port`, `perform_target_invalid_kind`, `incompatible_subset_redefine_kind`, `subsetting_target_not_accessible`, `invocation_argument_redefines_no_parameter` | Inspect the resolved endpoint, effective action kind, metadata usage, featured type, or invocation parameter before deciding whether each occurrence is a model error. These sites remain under [#425](https://github.com/elan8/spec42/issues/425). SysML v2 §§7.13, 7.17.6, 7.19, 7.27 and KerML §8.3.3.3.10. |
| `unsupported_action_definition_member`, `unsupported_action_usage_member`, `unsupported_connection_definition_member`, `unsupported_interface_definition_member`, `unsupported_occurrence_definition_member`, `unsupported_package_member`, `unsupported_part_definition_member`, `unsupported_part_usage_member`, `unsupported_port_usage_member`, `unsupported_requirement_definition_member`, `unsupported_use_case_definition_member`, `unsupported_view_definition_member` | The parser recognizes these forms and the semantic publication reports missing members. The occurrences include variants, body imports and assignments, nested packages, binding connectors, view filters and renderings, and requirement, action, and use-case members. Per-form follow-up remains under [#425](https://github.com/elan8/spec42/issues/425). SysML v2 §8.2.2. |

The specifications and pinned source support these dispositions. [SysML v2](https://www.omg.org/spec/SysML/2.0/Language/PDF), [KerML](https://www.omg.org/spec/KerML/1.0/PDF), [OMG 2026-04 release](https://github.com/Systems-Modeling/SysML-v2-Release/tree/2026-04).
