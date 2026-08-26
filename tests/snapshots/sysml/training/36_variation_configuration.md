# META
~~~ini
description=SysML Training 36 (Variability): Variation Configuration
type=file
~~~
# SOURCE
~~~sysml
package 'Variation Configuration' {
	private import 'Variation Usages'::*;
	
	part vehicle4Cyl :> vehicleFamily {
		part redefines engine = engine::'4cylEngine';
		part redefines transmission = transmission::manualTransmission;
	}
	
	part vehicle6Cyl :> vehicleFamily {
		part redefines engine = engine::'6cylEngine';
		part redefines transmission = transmission::manualTransmission;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/36_variation_configuration.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 21) (end 3 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 17) (end 4 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 17) (end 5 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 21) (end 8 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 17) (end 9 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 17) (end 10 29))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:520967a999dd3459bbfee27b1d5fbf00ee77d027cdd478d0bacea1c76675944b") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Variation Usages") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration::vehicle4Cyl"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicleFamily")))))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "engine")))))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1))))) (kind part) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "transmission")))))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration::vehicle6Cyl"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicleFamily")))))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "engine")))))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1))))) (kind part) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "transmission")))))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Variation Usages")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration::vehicle4Cyl"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicleFamily")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "engine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "transmission")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration::vehicle6Cyl"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicleFamily")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "engine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "transmission")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration::vehicle4Cyl"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1))))) (target (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration::vehicle4Cyl"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration::vehicle6Cyl"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1))))) (target (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration::vehicle6Cyl"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration::vehicle4Cyl")))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration::vehicle4Cyl")))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration::vehicle6Cyl")))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration::vehicle6Cyl")))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/36_variation_configuration.md") (range (start 1 16) (end 1 37)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Variation Usages")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/36_variation_configuration.md") (range (start 3 21) (end 3 34)) (probe (position 3 21))
    (reference (id (source (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration::vehicle4Cyl"))) (kind subsetting) (ordinal 0) (authored-target "vehicleFamily")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/36_variation_configuration.md") (range (start 4 17) (end 4 23)) (probe (position 4 17))
    (reference (id (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "engine")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/36_variation_configuration.md") (range (start 5 17) (end 5 29)) (probe (position 5 17))
    (reference (id (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "transmission")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/36_variation_configuration.md") (range (start 8 21) (end 8 34)) (probe (position 8 21))
    (reference (id (source (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration::vehicle6Cyl"))) (kind subsetting) (ordinal 0) (authored-target "vehicleFamily")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/36_variation_configuration.md") (range (start 9 17) (end 9 23)) (probe (position 9 17))
    (reference (id (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "engine")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/36_variation_configuration.md") (range (start 10 17) (end 10 29)) (probe (position 10 17))
    (reference (id (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "transmission")
      (outcome (status unresolved)))
    )
  )
)
~~~
