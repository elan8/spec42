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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "attributes.md"
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "ba64be3c3af135ff38ece80726bf37d9c063a941aaeada3325ec3f32a7516faa") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Attributes"))) (kind "package") (name "Attributes") (declared-name "Attributes"))
    (element (id (node (document "d0") (qualified-name "Attributes::AttributeValue"))) (kind "alias") (name "AttributeValue") (declared-name "AttributeValue") (parent (node (document "d0") (qualified-name "Attributes"))))
    (element (id (node (document "d0") (qualified-name "Attributes::DataValue"))) (kind "import") (name "DataValue") (declared-name "DataValue") (parent (node (document "d0") (qualified-name "Attributes"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::DataValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Attributes::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Attributes"))))
    (element (id (node (document "d0") (qualified-name "Attributes::attributeValues"))) (kind "alias") (name "attributeValues") (declared-name "attributeValues") (parent (node (document "d0") (qualified-name "Attributes"))))
    (element (id (node (document "d0") (qualified-name "Attributes::dataValues"))) (kind "import") (name "dataValues") (declared-name "dataValues") (parent (node (document "d0") (qualified-name "Attributes"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::dataValues") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Attributes::DataValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::DataValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Attributes::dataValues"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::dataValues") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 6 19) (end 6 34)) (probe (position 6 19))
      (reference
        (source (document "d0") (qualified-name "Attributes::DataValue"))
        (kind membershipImport) (ordinal 0) (authored-target "Base::DataValue")
        (range (start 6 19) (end 6 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 19) (end 7 35)) (probe (position 7 19))
      (reference
        (source (document "d0") (qualified-name "Attributes::dataValues"))
        (kind membershipImport) (ordinal 0) (authored-target "Base::dataValues")
        (range (start 7 19) (end 7 35))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
