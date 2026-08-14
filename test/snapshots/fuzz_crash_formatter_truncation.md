# META
~~~ini
description=Malformed part definitions with binary corruption handled gracefully
type=file
notes=Demonstrates handling of binary-corrupted input (fuzzer-generated null bytes). Formatter preserves malformed content as-is with sanitization for safety (null bytes → Unicode replacement char). Non-idempotent due to binary corruption, which is expected for malformed input. Diagnostics report structure errors.
~~~
# SOURCE
~~~sysml
package MassRollup2 {
	private import NumericalFunctions::*;

	part def MassedThing {
		attribute simpleMass :> ISQ::mass;
		attribute totalMass :> ISQ::mass default sLmpleMass;
	}

	part composicomackagteThing : MassedThing {
		p@rt subcomponents: MassedThing[*]ature redefin;
		arValuete :>> totalMass default
			simleMass + sum(subcomponents.totalMass);
	}

	part filter   ssThing :> compositeThing {
		attribute minMass :> ISQ::mass;
		atribute :>> totalMass =
		ates A;

	simpleMass + sum(subcomackage eMassponents.totalMassF?{in p:>ISQ::mass; p >= minMass});
	}

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/fuzz_crash_formatter_truncation.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 26) (end 4 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 25) (end 5 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 43) (end 5 53))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 9 2) (end 12 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 14 1) (end 20 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 14 1) (end 20 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:d01b79e670c8f3d078b774f56d534315ffd26e912cbd2c14601ae5e673801b1c") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (qualified-name "MassRollup2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (path (named (kind package) (name "MassRollup2")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "NumericalFunctions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (qualified-name "MassRollup2::MassedThing"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (qualified-name "MassRollup2::MassedThing::simpleMass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (qualified-name "MassRollup2::MassedThing::totalMass"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (default true) (operator false)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass")) (expressionOperand (reference "sLmpleMass"))))
    (declaration (id (node (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (qualified-name "MassRollup2::composicomackagteThing"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassedThing"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (path (named (kind package) (name "MassRollup2")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (qualified-name "MassRollup2::MassedThing::simpleMass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (qualified-name "MassRollup2::MassedThing::totalMass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (qualified-name "MassRollup2::MassedThing::totalMass"))) (kind expressionOperand) (ordinal 0))
      (authored-target "sLmpleMass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (qualified-name "MassRollup2::composicomackagteThing"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (qualified-name "MassRollup2::MassedThing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (qualified-name "MassRollup2::composicomackagteThing"))) (target (node (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (qualified-name "MassRollup2::MassedThing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (qualified-name "MassRollup2::composicomackagteThing"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (qualified-name "MassRollup2::MassedThing::totalMass"))) (state unresolved-operand))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (qualified-name "MassRollup2::composicomackagteThing")))
      (supertype (node (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (qualified-name "MassRollup2::MassedThing")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (range (start 1 16) (end 1 37)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (path (named (kind package) (name "MassRollup2")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (range (start 4 26) (end 4 35)) (probe (position 4 26))
    (reference (id (source (node (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (qualified-name "MassRollup2::MassedThing::simpleMass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (range (start 5 25) (end 5 34)) (probe (position 5 25))
    (reference (id (source (node (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (qualified-name "MassRollup2::MassedThing::totalMass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (range (start 5 43) (end 5 53)) (probe (position 5 43))
    (reference (id (source (node (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (qualified-name "MassRollup2::MassedThing::totalMass"))) (kind expressionOperand) (ordinal 0) (authored-target "sLmpleMass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (range (start 8 31) (end 8 42)) (probe (position 8 31))
    (reference (id (source (node (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (qualified-name "MassRollup2::composicomackagteThing"))) (kind featureTyping) (ordinal 0) (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/fuzz_crash_formatter_truncation.md") (qualified-name "MassRollup2::MassedThing")))))
  )
)
~~~
