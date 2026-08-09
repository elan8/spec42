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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Enumeration Definitions-2"))) (name "Enumeration Definitions-2") (declared-name "Enumeration Definitions-2")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Enumeration Definitions-2::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Enumeration Definitions-2::*#import"))) (name "*") (declared-name "*"))
        (element (kind "enum def") (id (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationKind"))) (name "ClassificationKind") (declared-name "ClassificationKind")
          (contains
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationKind::confidential"))) (name "confidential") (declared-name "confidential") (effective (featuring-type (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationKind")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationKind::secret"))) (name "secret") (declared-name "secret") (effective (featuring-type (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationKind")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationKind::unclassified"))) (name "unclassified") (declared-name "unclassified") (effective (featuring-type (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationKind")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationLevel"))) (name "ClassificationLevel") (declared-name "ClassificationLevel") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationLevel::code"))) (name "code") (declared-name "code") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationLevel")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationLevel::color"))) (name "color") (declared-name "color") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationLevel")))))
          )
        )
        (element (kind "enum def") (id (node (document "d0") (qualified-name "Enumeration Definitions-2::GradePoints"))) (name "GradePoints") (declared-name "GradePoints")
          (contains
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "Enumeration Definitions-2::GradePoints::A"))) (name "A") (declared-name "A") (effective (featuring-type (node (document "d0") (qualified-name "Enumeration Definitions-2::GradePoints")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "Enumeration Definitions-2::GradePoints::B"))) (name "B") (declared-name "B") (effective (featuring-type (node (document "d0") (qualified-name "Enumeration Definitions-2::GradePoints")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "Enumeration Definitions-2::GradePoints::C"))) (name "C") (declared-name "C") (effective (featuring-type (node (document "d0") (qualified-name "Enumeration Definitions-2::GradePoints")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "Enumeration Definitions-2::GradePoints::D"))) (name "D") (declared-name "D") (effective (featuring-type (node (document "d0") (qualified-name "Enumeration Definitions-2::GradePoints")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "Enumeration Definitions-2::GradePoints::F"))) (name "F") (declared-name "F") (effective (featuring-type (node (document "d0") (qualified-name "Enumeration Definitions-2::GradePoints")))))
          )
        )
      )
    )
  )
  (relationships
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationKind"))) (to (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationLevel"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
