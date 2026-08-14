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
  (document "memory://snapshot/attributes.md"
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
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 29) (end 9 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 30) (end 16 40))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:cada5bf0e4466e5bac58e24d7c47b92a4742f379f473bae01e84dd27744dd34c") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/attributes.md") (qualified-name "Attributes"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n * This package defines the base types for attributes and related structural elements \n * in the SysML language.\n "))))
    (declaration (id (node (document "memory://snapshot/attributes.md") (path (name "Attributes") (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Base::DataValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/attributes.md") (path (name "Attributes") (anonymous (kind import) (ordinal 1)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Base::dataValues") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/attributes.md") (qualified-name "Attributes::AttributeValue"))) (kind alias) (membership (kind alias) (visibility default)) (documentation (doc (text "\n\t\t * AttributeValue is the most general type of data values that represent qualities or characteristics \n\t\t * of a system or part of a system. AttributeValue is the base type of all AttributeDefinitions.\n\t\t "))) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "DataValue"))))
    (declaration (id (node (document "memory://snapshot/attributes.md") (qualified-name "Attributes::attributeValues"))) (kind alias) (membership (kind alias) (visibility default)) (documentation (doc (text "\n\t\t * attributeValues is the base feature for all AttributeUsages.\n\t\t "))) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "dataValues"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/attributes.md") (path (name "Attributes") (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0))
      (authored-target "Base::DataValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/attributes.md") (path (name "Attributes") (anonymous (kind import) (ordinal 1)))))) (kind membershipImport) (ordinal 0))
      (authored-target "Base::dataValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/attributes.md") (qualified-name "Attributes::AttributeValue"))) (kind aliasBinding) (ordinal 0))
      (authored-target "DataValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/attributes.md") (qualified-name "Attributes::attributeValues"))) (kind aliasBinding) (ordinal 0))
      (authored-target "dataValues")
      (outcome (status unresolved)))
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
  (query (document "memory://snapshot/attributes.md") (range (start 6 19) (end 6 34)) (probe (position 6 19))
    (reference (id (source (node (document "memory://snapshot/attributes.md") (path (name "Attributes") (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0) (authored-target "Base::DataValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/attributes.md") (range (start 7 19) (end 7 35)) (probe (position 7 19))
    (reference (id (source (node (document "memory://snapshot/attributes.md") (path (name "Attributes") (anonymous (kind import) (ordinal 1)))))) (kind membershipImport) (ordinal 0) (authored-target "Base::dataValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/attributes.md") (range (start 9 29) (end 9 38)) (probe (position 9 29))
    (reference (id (source (node (document "memory://snapshot/attributes.md") (qualified-name "Attributes::AttributeValue"))) (kind aliasBinding) (ordinal 0) (authored-target "DataValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/attributes.md") (range (start 16 30) (end 16 40)) (probe (position 16 30))
    (reference (id (source (node (document "memory://snapshot/attributes.md") (qualified-name "Attributes::attributeValues"))) (kind aliasBinding) (ordinal 0) (authored-target "dataValues")
      (outcome (status unresolved)))
  )
)
~~~
