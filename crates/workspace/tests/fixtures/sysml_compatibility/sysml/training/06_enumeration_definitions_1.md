# META
~~~ini
description=SysML Training 06 (Enumeration Definitions): Enumeration Definitions-1
type=file
~~~
# SOURCE
~~~sysml
package 'Enumeration Definitions-1' {
	private import ScalarValues::Real;
	
	enum def TrafficLightColor {
		enum green;
		enum yellow;
		enum red;
	}
	
	part def TrafficLight {
		attribute currentColor : TrafficLightColor;
	}
	
	part def TrafficLightGo specializes TrafficLight {
		attribute redefines currentColor = TrafficLightColor::green;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwEnum,KwDef,Ident,OpenCurly,
KwEnum,Ident,Semicolon,
KwEnum,Ident,Semicolon,
KwEnum,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwAttribute,KwRedefines,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Enumeration Definitions-1''
    (import_decl private 'ScalarValues::Real')
    (enum_def 'TrafficLightColor'
      (enum_value 'green')
      (enum_value 'yellow')
      (enum_value 'red'))
    (part_def 'TrafficLight'
      (attribute_usage 'currentColor' : 'TrafficLightColor'))
    (part_def 'TrafficLightGo' :> 'TrafficLight'
      (attribute_usage :>> 'currentColor' value))))
~~~
# FORMAT
~~~sysml
package 'Enumeration Definitions-1' {
    private import ScalarValues::Real;

    enum def TrafficLightColor {
        enum green;
        enum yellow;
        enum red;
    }

    part def TrafficLight {
        attribute currentColor : TrafficLightColor;
    }

    part def TrafficLightGo specializes TrafficLight {
        attribute redefines currentColor = TrafficLightColor::green;
    }
}

~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Enumeration Definitions-1"))) (name "Enumeration Definitions-1") (declared-name "Enumeration Definitions-1")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Enumeration Definitions-1::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLight"))) (name "TrafficLight") (declared-name "TrafficLight") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLight::currentColor"))) (name "currentColor") (declared-name "currentColor") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLight")))))
          )
        )
        (element (kind "enum def") (id (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor"))) (name "TrafficLightColor") (declared-name "TrafficLightColor")
          (contains
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor::green"))) (name "green") (declared-name "green") (effective (featuring-type (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor::red"))) (name "red") (declared-name "red") (effective (featuring-type (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor::yellow"))) (name "yellow") (declared-name "yellow") (effective (featuring-type (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightGo"))) (name "TrafficLightGo") (declared-name "TrafficLightGo") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightGo::currentColor"))) (name "currentColor") (declared-name "currentColor") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "featureReference") (reference "TrafficLightColor::green")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightGo"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightGo::currentColor"))) (role feature-value))))
          )
        )
      )
    )
  )
  (relationships
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightGo::currentColor"))) (to (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLight::currentColor"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightGo"))) (to (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLight"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLight::currentColor"))) (to (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/06_enumeration_definitions_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 35))
      )
    )
  )
)
~~~
