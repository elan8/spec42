# META
~~~ini
description=Fuzzer crash #5: malformed SysML input
type=file
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

	simpleMass + sum(subcomackage eMassponents.totalMassFpackage 'Metadata Example-1 {
	
	metadata def SatyFeature;
	m@ata def Securi
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
Ident,Plus,Ident,OpenParen,Ident,Ident,Dot,Ident,MalformedUnrestrictedName,
KwMetadata,KwDef,Ident,MalformedUnknownToken,Ident,Semicolon,
Ident,At,Ident,KwDef,Ident,EndOfFile,
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
        attribute totalMass :> ISQ::mass default = sLmpleMass;
    }

    part composicomackagteThing : MassedThing {
        @rt subcomponents: MassedThing[*]ature redefin;
        arValuete :>> totalMass default = simleMass + sum(subcomponents.totalMass);
    }

    ssThing :> compositeThing {
		attribute minMass :> ISQ::mass;
		atribute :>> totalMass =
		ates A;

	simpleMass + sum(subcomackage eMassponents.totalMassFpackage 'Metadata Example-1 {
	
	metadata def SatyFeature;
	m@ata def Securi
    }
}
~~~
# EXPECTED
~~~
tokenize.UnclosedUnrestrictedName
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_close_curly
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
~~~
# PROBLEMS
~~~
tokenize.UnclosedUnrestrictedName
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_close_curly
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
~~~
# SMG
~~~
(semantic-graph
  (status (skip (code "SMG-EMPTY-RECOVERY") (reason "parser recovery for non-empty source produced no typed semantic graph facts")))
  (containment
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
