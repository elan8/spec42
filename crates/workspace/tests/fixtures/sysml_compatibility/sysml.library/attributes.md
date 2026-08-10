# META
~~~ini
description=Standard Library: Systems Library/Attributes
type=file
~~~
# SOURCE
~~~sysml
standard library package Attributes {
    doc /*
 * This package defines the base types for attributes and related structural elements 
 * in the SysML language.
 */

    private import Base::DataValue;
    private import Base::dataValues;

    alias AttributeValue for DataValue {
        doc /*
		 * AttributeValue is the most general type of data values that represent qualities or characteristics 
		 * of a system or part of a system. AttributeValue is the base type of all AttributeDefinitions.
		 */
    }

    alias attributeValues for dataValues {
        doc /*
		 * attributeValues is the base feature for all AttributeUsages.
		 */
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
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwAlias,Ident,KwFor,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Attributes'
    (documentation)
    (import_decl private 'Base::DataValue')
    (import_decl private 'Base::dataValues')
    (alias_member 'AttributeValue' for 'DataValue'
      (documentation))
    (alias_member 'attributeValues' for 'dataValues'
      (documentation))))
~~~
# FORMAT
~~~sysml
standard library package Attributes {
    doc /*
 * This package defines the base types for attributes and related structural elements 
 * in the SysML language.
 */

    private import Base::DataValue;
    private import Base::dataValues;

    alias AttributeValue for DataValue {
        doc /*
		 * AttributeValue is the most general type of data values that represent qualities or characteristics 
		 * of a system or part of a system. AttributeValue is the base type of all AttributeDefinitions.
		 */
    }

    alias attributeValues for dataValues {
        doc /*
		 * attributeValues is the base feature for all AttributeUsages.
		 */
    }
}

~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Attributes"))) (name "Attributes") (declared-name "Attributes")
      (contains
        (element (kind "alias") (id (node (document "d0") (qualified-name "Attributes::AttributeValue"))) (name "AttributeValue") (declared-name "AttributeValue"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Attributes::DataValue"))) (name "DataValue") (declared-name "DataValue"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "Attributes::_documentation"))) (name ""))
        (element (kind "alias") (id (node (document "d0") (qualified-name "Attributes::attributeValues"))) (name "attributeValues") (declared-name "attributeValues"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Attributes::dataValues"))) (name "dataValues") (declared-name "dataValues"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Attributes::_documentation"))) (to (node (document "d0") (qualified-name "Attributes"))) (provenance authored))
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
  (document "sysml.library/attributes.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 19) (end 6 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 19) (end 7 35))
      )
    )
  )
)
~~~
