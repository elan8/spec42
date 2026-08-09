# META
~~~ini
description=Fuzzer crash: malformed package with unclosed braces
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
		wtes A;

	simpleMass + sum(subcomackage eMassponents import Numeric.totalMassFpackage 'Metadata Example-1' {

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
KwPart,Ident,Colon,Ident,OpenCurly,
Ident,At,Ident,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Ident,Ident,Semicolon,
Ident,ColonGtGt,Ident,KwDefault,
Ident,Plus,Ident,OpenParen,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
KwPart,KwFilter,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
Ident,ColonGtGt,Ident,Eq,
Ident,Ident,Semicolon,
Ident,Plus,Ident,OpenParen,Ident,Ident,KwImport,Ident,Dot,Ident,UnrestrictedName,OpenCurly,
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
		wtes A;

	simpleMass + sum(subcomackage eMassponents import Numeric.totalMassFpackage 'Metadata Example-1' {

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
      (part_usage 'composicomackagteThing' : 'MassRollup2::MassedThing'[part_def]
        (not_implemented 'malformed')
        (reference_usage reference 'arValuete' :>> 'MassRollup2::MassedThing::totalMass'[attribute_usage]
          (feature_value (default =))))
      (not_implemented 'malformed'))))
~~~
