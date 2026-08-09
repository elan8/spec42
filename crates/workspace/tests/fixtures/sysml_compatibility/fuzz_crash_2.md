# META
~~~ini
description=Fuzzer crash #2: malformed SysML input
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

	part composicomackagteT€ing : MassedThing {
		p@rt subcomponents: MassedThing[*]ature redefin;
		arValuete slizes ClMass default
			simleMass + sum(subcomponents.totalMass);
	}

	part filter  ssThing :> compositeThing {
		attribute minMass :> ISQ::mass;
		atribute :>> totalMass =
		ates A;

	simpleMass + sum(subcomackage eMassponents.totalMassFpackage 'Metadata Example-1' {
	
	metadata def SafetyFeature;
	metadata def Securi
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,KwDefault,Ident,Semicolon,
CloseCurly,
KwPart,Ident,MalformedUnknownToken,Ident,Colon,Ident,OpenCurly,
Ident,At,Ident,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Ident,Ident,Semicolon,
Ident,Ident,Ident,KwDefault,
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
  (package_def 'MassRollup2'
    (import_decl private 'NumericalFunctions::*')
    (part_def 'MassedThing'
      (attribute_usage 'simpleMass' :> 'ISQ::mass')
      (attribute_usage 'totalMass' :> 'ISQ::mass' value))
    (malformed)
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

    €ing : MassedThing {
		p@rt subcomponents: MassedThing[*]ature redefin;
		arValuete slizes ClMass default
			simleMass + sum(subcomponents.totalMass);
	}

    ssThing :> compositeThing {
		attribute minMass :> ISQ::mass;
		atribute :>> totalMass =
		ates A;

	simpleMass + sum(subcomackage eMassponents.totalMassFpackage 'Metadata Example-1' {
	
	metadata def SafetyFeature;
	metadata def Securi
    }
    }
}
~~~
# EXPECTED
~~~
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_close_curly
semantic.ambiguous_member 'malformed'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
~~~
# PROBLEMS
~~~
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_close_curly
semantic.ambiguous_member 'malformed'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
~~~
# SMG
~~~
(model
  (namespace
    (package 'MassRollup2'
      (namespace_import private -> 'NumericalFunctions'[unresolved])
      (part_def 'MassedThing'
        (attribute_usage composite 'simpleMass' :> 'ISQ::mass'[unresolved])
        (attribute_usage composite 'totalMass' :> 'ISQ::mass'[unresolved]
          (feature_value (default =))))
      (not_implemented 'malformed')
      (not_implemented 'malformed'))))
~~~
