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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,KwDefault,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
Ident,At,Ident,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Ident,Ident,Semicolon,
Ident,ColonGtGt,Ident,KwDefault,
Ident,Plus,Ident,OpenParen,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
KwPart,KwFilter,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
Ident,ColonGtGt,Ident,Eq,
Ident,Ident,Semicolon,
Ident,Plus,Ident,OpenParen,Ident,Ident,Dot,Ident,Question,OpenCurly,KwIn,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,Ident,GtEq,Ident,CloseCurly,CloseParen,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'MassRollup2'
    (import_decl private 'NumericalFunctions::*')
    (part_def 'MassedThing'
      (attribute_usage 'simpleMass' :> 'ISQ::mass')
      (attribute_usage 'totalMass' :> 'ISQ::mass' value))
    (part_usage 'composicomackagteThing' : 'MassedThing'
      (malformed)
      (default_ref_usage 'arValuete' :>> 'totalMass' value))
    (malformed)))
~~~
# EXPECTED
~~~
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
~~~
# PROBLEMS
~~~
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "57391fcf3aaa8b96355164d1f7d8c6f05158d08cbf1b823c48f5a5f74efde879") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "MassRollup2"))) (kind "package") (name "MassRollup2") (declared-name "MassRollup2") (range (start (line 0) (character 0)) (end (line 0) (character 570))))
    (element (id (node (document "d0") (qualified-name "MassRollup2::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 38))) (parent (node (document "d0") (qualified-name "MassRollup2"))) (authored (membership (kind Import) (visibility "private") (import (reference "NumericalFunctions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 34))))))
    (element (id (node (document "d0") (qualified-name "MassRollup2::MassedThing"))) (kind "part def") (name "MassedThing") (declared-name "MassedThing") (range (start (line 3) (character 1)) (end (line 3) (character 118))) (parent (node (document "d0") (qualified-name "MassRollup2"))))
    (element (id (node (document "d0") (qualified-name "MassRollup2::MassedThing::simpleMass"))) (kind "attribute") (name "simpleMass") (declared-name "simpleMass") (range (start (line 4) (character 2)) (end (line 4) (character 36))) (parent (node (document "d0") (qualified-name "MassRollup2::MassedThing"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass") (range (start (line 4) (character 26)) (end (line 4) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "MassRollup2::MassedThing::totalMass"))) (kind "attribute") (name "totalMass") (declared-name "totalMass") (range (start (line 5) (character 2)) (end (line 5) (character 54))) (parent (node (document "d0") (qualified-name "MassRollup2::MassedThing"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass") (range (start (line 5) (character 25)) (end (line 5) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "MassRollup2::composicomackagteThing"))) (kind "part") (name "composicomackagteThing") (declared-name "composicomackagteThing") (range (start (line 8) (character 1)) (end (line 8) (character 177))) (parent (node (document "d0") (qualified-name "MassRollup2"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassedThing") (range (start (line 8) (character 31)) (end (line 8) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "MassRollup2::filter"))) (kind "kermlDecl") (name "filter") (declared-name "filter") (range (start (line 14) (character 1)) (end (line 14) (character 206))) (parent (node (document "d0") (qualified-name "MassRollup2"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "MassRollup2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "NumericalFunctions::*") (range (start (line 1) (character 16)) (end (line 1) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup2::MassedThing::simpleMass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (range (start (line 4) (character 26)) (end (line 4) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup2::MassedThing::totalMass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (range (start (line 5) (character 25)) (end (line 5) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup2::composicomackagteThing"))) (kind featureTyping) (ordinal 0)) (authored-target "MassedThing") (range (start (line 8) (character 31)) (end (line 8) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MassRollup2::MassedThing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassRollup2::composicomackagteThing"))) (target (node (document "d0") (qualified-name "MassRollup2::MassedThing"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassRollup2::composicomackagteThing"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "MassRollup2::MassedThing::totalMass")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
