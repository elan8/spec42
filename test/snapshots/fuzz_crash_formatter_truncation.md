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
  (document "fuzz_crash_formatter_truncation.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 26) (end 4 35))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 5 2) (end 5 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 25) (end 5 34))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 9 2) (end 9 131))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "57391fcf3aaa8b96355164d1f7d8c6f05158d08cbf1b823c48f5a5f74efde879") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "MassRollup2"))) (kind "package") (name "MassRollup2") (declared-name "MassRollup2"))
    (element (id (node (document "d0") (qualified-name "MassRollup2::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "MassRollup2"))) (authored (membership (kind Import) (visibility "private") (import (reference "NumericalFunctions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MassRollup2::MassedThing"))) (kind "part def") (name "MassedThing") (declared-name "MassedThing") (parent (node (document "d0") (qualified-name "MassRollup2"))))
    (element (id (node (document "d0") (qualified-name "MassRollup2::MassedThing::simpleMass"))) (kind "attribute") (name "simpleMass") (declared-name "simpleMass") (parent (node (document "d0") (qualified-name "MassRollup2::MassedThing"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "MassRollup2::MassedThing::totalMass"))) (kind "attribute") (name "totalMass") (declared-name "totalMass") (parent (node (document "d0") (qualified-name "MassRollup2::MassedThing"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "MassRollup2::composicomackagteThing"))) (kind "part") (name "composicomackagteThing") (declared-name "composicomackagteThing") (parent (node (document "d0") (qualified-name "MassRollup2"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassedThing")))))
    (element (id (node (document "d0") (qualified-name "MassRollup2::filter"))) (kind "kermlDecl") (name "filter") (declared-name "filter") (parent (node (document "d0") (qualified-name "MassRollup2"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "MassRollup2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "NumericalFunctions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup2::MassedThing::simpleMass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup2::MassedThing::totalMass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup2::composicomackagteThing"))) (kind featureTyping) (ordinal 0)) (authored-target "MassedThing") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassRollup2::MassedThing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassRollup2::composicomackagteThing"))) (target (node (document "d0") (qualified-name "MassRollup2::MassedThing"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassRollup2::composicomackagteThing"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "MassRollup2::MassedThing::totalMass")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 4 26) (end 4 35)) (probe (position 4 26))
      (reference
        (source (document "d0") (qualified-name "MassRollup2::MassedThing::simpleMass"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 4 26) (end 4 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 5 25) (end 5 34)) (probe (position 5 25))
      (reference
        (source (document "d0") (qualified-name "MassRollup2::MassedThing::totalMass"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 5 25) (end 5 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 31) (end 8 42)) (probe (position 8 31))
      (reference
        (source (document "d0") (qualified-name "MassRollup2::composicomackagteThing"))
        (kind featureTyping) (ordinal 0) (authored-target "MassedThing")
        (range (start 8 31) (end 8 42))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassRollup2::MassedThing") (range (start 3 1) (end 3 118)))
        )
      )
    )
    (query (range (start 1 16) (end 1 34)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "MassRollup2::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "NumericalFunctions::*")
        (range (start 1 16) (end 1 34))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
