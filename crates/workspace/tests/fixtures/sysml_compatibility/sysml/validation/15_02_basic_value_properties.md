# META
~~~ini
description=SysML Validation (15-Properties-Values-Expressions): 15_02-Basic Value Properties
type=file
~~~
# SOURCE
~~~sysml
package '15_02-Basic Value Properties' {
	private import ScalarValues::*;
	
    attribute def LengthValue :> Real {
		doc
		/*
		 * Real world user models would use a quantity type
		 * from the library model. A attribute def is defined
		 * here to show that it is possible.
		 */
	}

    part def Tire {
    	attribute manufacturer: String;
        attribute hubDiameter: LengthValue;
        attribute width: Integer;
    }
    
    part frenchTire: Tire {
    	attribute :>> manufacturer = "Michelin";
    	attribute :>> hubDiameter = 18.0;
    	attribute :>> width = 245;
    }
    
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,StringValue,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''15_02-Basic Value Properties''
    (import_decl private 'ScalarValues::*')
    (attribute_def 'LengthValue' :> 'Real'
      (documentation))
    (part_def 'Tire'
      (attribute_usage 'manufacturer' : 'String')
      (attribute_usage 'hubDiameter' : 'LengthValue')
      (attribute_usage 'width' : 'Integer'))
    (part_usage 'frenchTire' : 'Tire'
      (attribute_usage :>> 'manufacturer' value)
      (attribute_usage :>> 'hubDiameter' value)
      (attribute_usage :>> 'width' value))))
~~~
# FORMAT
~~~sysml
package '15_02-Basic Value Properties' {
    private import ScalarValues::*;

    attribute def LengthValue :> Real {
        doc
        /*
		 * Real world user models would use a quantity type
		 * from the library model. A attribute def is defined
		 * here to show that it is possible.
		 */
    }

    part def Tire {
        attribute manufacturer: String;
        attribute hubDiameter: LengthValue;
        attribute width: Integer;
    }

    part frenchTire: Tire {
        attribute :>> manufacturer = "Michelin";
        attribute :>> hubDiameter = 18.0;
        attribute :>> width = 245;
    }

}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Integer'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Integer'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "15_02-Basic Value Properties"))) (name "15_02-Basic Value Properties") (declared-name "15_02-Basic Value Properties")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::*"))) (name "*") (declared-name "*"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::LengthValue"))) (name "LengthValue") (declared-name "LengthValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::LengthValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "15_02-Basic Value Properties::LengthValue")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire"))) (name "Tire") (declared-name "Tire") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter"))) (name "hubDiameter") (declared-name "hubDiameter") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::manufacturer"))) (name "manufacturer") (declared-name "manufacturer") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::width"))) (name "width") (declared-name "width") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire")))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire"))) (name "frenchTire") (declared-name "frenchTire") (declared (properties (ordered false)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::hubDiameter"))) (name "hubDiameter") (declared-name "hubDiameter") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "realLiteral") (literal "18.0")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::hubDiameter"))) (role feature-value))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::manufacturer"))) (name "manufacturer") (declared-name "manufacturer") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "stringLiteral") (literal "Michelin")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::manufacturer"))) (role feature-value))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::width"))) (name "width") (declared-name "width") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "integerLiteral") (literal 245)))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::width"))) (role feature-value))))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "15_02-Basic Value Properties::LengthValue::_documentation"))) (to (node (document "d0") (qualified-name "15_02-Basic Value Properties::LengthValue"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::hubDiameter"))) (to (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::manufacturer"))) (to (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::manufacturer"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::width"))) (to (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::width"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter"))) (to (node (document "d0") (qualified-name "15_02-Basic Value Properties::LengthValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire"))) (to (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire"))))
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
  (document "sysml/validation/15_02_basic_value_properties.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 4) (end 3 208))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 8) (end 15 33))
      )
    )
  )
)
~~~
