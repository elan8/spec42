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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "15_02_basic_value_properties.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
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
        (range (start 13 5) (end 13 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 29) (end 13 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 8) (end 15 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 25) (end 15 32))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "aa86b8bc5ff2287f83c2b8418bc1c3b2425aa9952b0e18a9c699699feeac6cd6") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "15_02-Basic Value Properties"))) (kind "package") (name "15_02-Basic Value Properties") (declared-name "15_02-Basic Value Properties") (range (start (line 0) (character 0)) (end (line 0) (character 589))))
    (element (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "15_02-Basic Value Properties"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::LengthValue"))) (kind "attribute def") (name "LengthValue") (declared-name "LengthValue") (range (start (line 3) (character 4)) (end (line 3) (character 208))) (parent (node (document "d0") (qualified-name "15_02-Basic Value Properties"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::LengthValue::_documentation"))) (kind "documentation") (name "") (range (start (line 3) (character 4)) (end (line 3) (character 208))) (parent (node (document "d0") (qualified-name "15_02-Basic Value Properties::LengthValue"))))
    (element (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire"))) (kind "part def") (name "Tire") (declared-name "Tire") (range (start (line 12) (character 4)) (end (line 12) (character 140))) (parent (node (document "d0") (qualified-name "15_02-Basic Value Properties"))))
    (element (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter"))) (kind "attribute") (name "hubDiameter") (declared-name "hubDiameter") (range (start (line 14) (character 8)) (end (line 14) (character 43))) (parent (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)) (typing (reference "LengthValue") (range (start (line 14) (character 31)) (end (line 14) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::manufacturer"))) (kind "attribute") (name "manufacturer") (declared-name "manufacturer") (range (start (line 13) (character 5)) (end (line 13) (character 36))) (parent (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)) (typing (reference "String") (range (start (line 13) (character 29)) (end (line 13) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::width"))) (kind "attribute") (name "width") (declared-name "width") (range (start (line 15) (character 8)) (end (line 15) (character 33))) (parent (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire"))) (authored (membership (kind Feature)) (relationships (typing (reference "Integer") (range none)) (typing (reference "Integer") (range (start (line 15) (character 25)) (end (line 15) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire"))) (kind "part") (name "frenchTire") (declared-name "frenchTire") (range (start (line 18) (character 4)) (end (line 18) (character 150))) (parent (node (document "d0") (qualified-name "15_02-Basic Value Properties"))) (authored (membership (kind Feature)) (relationships (typing (reference "Tire") (range (start (line 18) (character 21)) (end (line 18) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::hubDiameter"))) (kind "attribute") (name "hubDiameter") (declared-name "hubDiameter") (range (start (line 20) (character 5)) (end (line 20) (character 38))) (parent (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "hubDiameter") (range (start (line 20) (character 19)) (end (line 20) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::manufacturer"))) (kind "attribute") (name "manufacturer") (declared-name "manufacturer") (range (start (line 19) (character 5)) (end (line 19) (character 45))) (parent (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "manufacturer") (range (start (line 19) (character 19)) (end (line 19) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::width"))) (kind "attribute") (name "width") (declared-name "width") (range (start (line 21) (character 5)) (end (line 21) (character 31))) (parent (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "width") (range (start (line 21) (character 19)) (end (line 21) (character 24)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::LengthValue"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_02-Basic Value Properties::LengthValue")))))
    (reference (id (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (range (start (line 14) (character 31)) (end (line 14) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_02-Basic Value Properties::LengthValue")))))
    (reference (id (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::manufacturer"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::manufacturer"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (range (start (line 13) (character 29)) (end (line 13) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::width"))) (kind featureTyping) (ordinal 0)) (authored-target "Integer") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::width"))) (kind featureTyping) (ordinal 1)) (authored-target "Integer") (range (start (line 15) (character 25)) (end (line 15) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire"))) (kind featureTyping) (ordinal 0)) (authored-target "Tire") (range (start (line 18) (character 21)) (end (line 18) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire")))))
    (reference (id (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::hubDiameter"))) (kind redefinition) (ordinal 0)) (authored-target "hubDiameter") (range (start (line 20) (character 19)) (end (line 20) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::hubDiameter")))))
    (reference (id (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::manufacturer"))) (kind redefinition) (ordinal 0)) (authored-target "manufacturer") (range (start (line 19) (character 19)) (end (line 19) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::manufacturer")))))
    (reference (id (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::width"))) (kind redefinition) (ordinal 0)) (authored-target "width") (range (start (line 21) (character 19)) (end (line 21) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::width")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter"))) (target (node (document "d0") (qualified-name "15_02-Basic Value Properties::LengthValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter"))) (target (node (document "d0") (qualified-name "15_02-Basic Value Properties::LengthValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire::hubDiameter"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire"))) (target (node (document "d0") (qualified-name "15_02-Basic Value Properties::Tire"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::hubDiameter"))) (target (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::hubDiameter"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::hubDiameter"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::manufacturer"))) (target (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::manufacturer"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::manufacturer"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::width"))) (target (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::width"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::width"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::hubDiameter")) (expression (status "ok") (value (integer 18))))
    (node (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::manufacturer")) (expression (status "ok") (value (string "Michelin"))))
    (node (node (document "d0") (qualified-name "15_02-Basic Value Properties::frenchTire::width")) (expression (status "ok") (value (integer 245))))
  )
)
~~~
