# META
~~~ini
description=Fuzzer crash #1: malformed SysML input
type=file
semantic_graph=skip
semantic_graph_skip_reason=parser recovery for non-empty source produced no typed semantic graph facts
~~~
# SOURCE
~~~sysml
package MassRkllup2 {
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

	simpleMass + sum(subcomackage eMassponents.totalMassFpackage 'Metadata Example-1' {
	
	metadata def SafetyFeature;
	metadata def Securi
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_crash_1.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "missing_closing_brace")
        (source "sysml")
        (range (start 22 20) (end 22 21))
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
Ident,Plus,Ident,OpenParen,Ident,Ident,Dot,Ident,UnrestrictedName,OpenCurly,
KwMetadata,KwDef,Ident,Semicolon,
KwMetadata,KwDef,Ident,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'MassRkllup2'
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
parse.expected_close_curly
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
~~~
# PROBLEMS
~~~
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_close_curly
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
~~~
# FORMAT
~~~sysml
package MassRkllup2 {
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

        simpleMass + sum(subcomackage eMassponents.totalMassFpackage 'Metadata Example-1' {

            metadata def SafetyFeature;
            metadata def Securi

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "a49f08491d5450050f0ba6e0ce4290b824df20c2167d1fb0273cff7f9b8dfd07") (contract-version "canonical-resolution-v1"))
  (structure
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
