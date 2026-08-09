# META
~~~ini
description=SysML Training 06 (Enumeration Definitions): Enumeration Definitions-2
type=file
~~~
# SOURCE
~~~sysml
package 'Enumeration Definitions-2' {
	private import ScalarValues::*;
	private import 'Enumeration Definitions-1'::*;
	
	attribute def ClassificationLevel {
		attribute code : String;
		attribute color : TrafficLightColor;
	}
	
	enum def ClassificationKind specializes ClassificationLevel {
		unclassified {
			:>> code = "uncl";
			:>> color = TrafficLightColor::green;
		}
		confidential {
			:>> code = "conf";
			:>> color = TrafficLightColor::yellow;
		}
		secret {
			:>> code = "secr";
			:>> color = TrafficLightColor::red;
		}
	}
	
	enum def GradePoints :> Real {
		A = 4.0;
		B = 3.0;
		C = 2.0;
		D = 1.0;
		F = 0.0;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwAttribute,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwEnum,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
Ident,OpenCurly,
ColonGtGt,Ident,Eq,StringValue,Semicolon,
ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
Ident,OpenCurly,
ColonGtGt,Ident,Eq,StringValue,Semicolon,
ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
Ident,OpenCurly,
ColonGtGt,Ident,Eq,StringValue,Semicolon,
ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwEnum,KwDef,Ident,ColonGt,Ident,OpenCurly,
Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Enumeration Definitions-2''
    (import_decl private 'ScalarValues::*')
    (import_decl private ''Enumeration Definitions-1'::*')
    (attribute_def 'ClassificationLevel'
      (attribute_usage 'code' : 'String')
      (attribute_usage 'color' : 'TrafficLightColor'))
    (enum_def 'ClassificationKind' :> 'ClassificationLevel'
      (enum_value 'unclassified'
        (default_ref_usage :>> 'code' value)
        (default_ref_usage :>> 'color' value))
      (enum_value 'confidential'
        (default_ref_usage :>> 'code' value)
        (default_ref_usage :>> 'color' value))
      (enum_value 'secret'
        (default_ref_usage :>> 'code' value)
        (default_ref_usage :>> 'color' value)))
    (enum_def 'GradePoints' :> 'Real'
      (enum_value 'A' value)
      (enum_value 'B' value)
      (enum_value 'C' value)
      (enum_value 'D' value)
      (enum_value 'F' value))))
~~~
# FORMAT
~~~sysml
package 'Enumeration Definitions-2' {
    private import ScalarValues::*;
    private import 'Enumeration Definitions-1'::*;

    attribute def ClassificationLevel {
        attribute code : String;
        attribute color : TrafficLightColor;
    }

    enum def ClassificationKind specializes ClassificationLevel {
        enum unclassified {
            :>> code = "uncl";
            :>> color = TrafficLightColor::green;
        }
        enum confidential {
            :>> code = "conf";
            :>> color = TrafficLightColor::yellow;
        }
        enum secret {
            :>> code = "secr";
            :>> color = TrafficLightColor::red;
        }
    }

    enum def GradePoints :> Real {
        enum A = 4.0;
        enum B = 3.0;
        enum C = 2.0;
        enum D = 1.0;
        enum F = 0.0;
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'String'
semantic.unresolved_name 'TrafficLightColor'
semantic.unresolved_name 'Real'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'String'
semantic.unresolved_name 'TrafficLightColor'
semantic.unresolved_name 'Real'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Enumeration Definitions-2'
      (namespace_import private -> 'ScalarValues'[unresolved])
      (namespace_import private -> 'Enumeration Definitions-1'[unresolved])
      (attribute_def 'ClassificationLevel'
        (attribute_usage composite 'code' : 'String'[unresolved])
        (attribute_usage composite 'color' : 'TrafficLightColor'[unresolved]))
      (enum_def 'ClassificationKind' :> 'Enumeration Definitions-2::ClassificationLevel'[attribute_def]
        (enum_usage composite 'unclassified'
          (reference_usage reference :>> 'Enumeration Definitions-2::ClassificationLevel::code'[attribute_usage]
            (feature_value (=)))
          (reference_usage reference :>> 'Enumeration Definitions-2::ClassificationLevel::color'[attribute_usage]
            (feature_value (=))))
        (enum_usage composite 'confidential'
          (reference_usage reference :>> 'Enumeration Definitions-2::ClassificationLevel::code'[attribute_usage]
            (feature_value (=)))
          (reference_usage reference :>> 'Enumeration Definitions-2::ClassificationLevel::color'[attribute_usage]
            (feature_value (=))))
        (enum_usage composite 'secret'
          (reference_usage reference :>> 'Enumeration Definitions-2::ClassificationLevel::code'[attribute_usage]
            (feature_value (=)))
          (reference_usage reference :>> 'Enumeration Definitions-2::ClassificationLevel::color'[attribute_usage]
            (feature_value (=)))))
      (enum_def 'GradePoints' :> 'Real'[unresolved]
        (enum_usage composite 'A'
          (feature_value (=)))
        (enum_usage composite 'B'
          (feature_value (=)))
        (enum_usage composite 'C'
          (feature_value (=)))
        (enum_usage composite 'D'
          (feature_value (=)))
        (enum_usage composite 'F'
          (feature_value (=)))))))
~~~
