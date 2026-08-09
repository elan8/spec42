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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "MassRollup2"))) (name "MassRollup2") (declared-name "MassRollup2")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "MassRollup2::*"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "MassRollup2::MassedThing"))) (name "MassedThing") (declared-name "MassedThing") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MassRollup2::MassedThing::simpleMass"))) (name "simpleMass") (declared-name "simpleMass") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MassRollup2::MassedThing")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MassRollup2::MassedThing::totalMass"))) (name "totalMass") (declared-name "totalMass") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind default) (expression (kind "featureReference") (reference "sLmpleMass")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MassRollup2::MassedThing")))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "MassRollup2::composicomackagteThing"))) (name "composicomackagteThing") (declared-name "composicomackagteThing") (declared (properties (composite true) (reference false) (ordered false))))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "MassRollup2::filter"))) (name "filter") (declared-name "filter"))
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "MassRollup2::composicomackagteThing"))) (to (node (document "d0") (qualified-name "MassRollup2::MassedThing"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
