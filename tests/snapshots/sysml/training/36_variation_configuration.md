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
        (severity warning)
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
        (range (start 4 26) (end 4 46))
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
        (range (start 5 32) (end 5 64))
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
        (range (start 9 26) (end 9 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 17) (end 10 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 32) (end 10 64))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:1fc1ed9e10b55746436e3dbaa6a85460ae737847da3eb1b49d9c1011bfd1215f"))
  (declarations
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Variation Usages") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration::vehicle4Cyl"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicleFamily")))))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (effective-identification (name unresolved) (short-name unresolved) (provenance first-redefinition)) (feature-value (kind bind) (value (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "engine")))))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1))))) (kind part) (membership (kind feature) (visibility default)) (effective-identification (name unresolved) (short-name unresolved) (provenance first-redefinition)) (feature-value (kind bind) (value (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "transmission")))))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "engine::4cylEngine")))))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "transmission::manualTransmission")))))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration::vehicle6Cyl"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicleFamily")))))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (effective-identification (name unresolved) (short-name unresolved) (provenance first-redefinition)) (feature-value (kind bind) (value (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "engine")))))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1))))) (kind part) (membership (kind feature) (visibility default)) (effective-identification (name unresolved) (short-name unresolved) (provenance first-redefinition)) (feature-value (kind bind) (value (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "transmission")))))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "engine::6cylEngine")))))
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "transmission::manualTransmission")))))
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
    (reference (id (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "engine::4cylEngine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "transmission::manualTransmission")
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
    (reference (id (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "engine::6cylEngine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "transmission::manualTransmission")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration::vehicle4Cyl"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1))))) (target (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration::vehicle4Cyl"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration::vehicle4Cyl"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration::vehicle4Cyl"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration::vehicle6Cyl"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1))))) (target (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration::vehicle6Cyl"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration::vehicle6Cyl"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration::vehicle6Cyl"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (state unresolved-operand))
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
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration::vehicle4Cyl")))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/36_variation_configuration.md") (qualified-name "Variation Configuration::vehicle6Cyl")))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)))))
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
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (result-typing (status unresolved)) (feature-reference "engine::4cylEngine" (target unresolved)))
  (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (result-typing (status unresolved)) (feature-reference "transmission::manualTransmission" (target unresolved)))
  (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (result-typing (status unresolved)) (feature-reference "engine::6cylEngine" (target unresolved)))
  (declaration (id (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (result-typing (status unresolved)) (feature-reference "transmission::manualTransmission" (target unresolved)))
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
  (query (document "memory://snapshot/36_variation_configuration.md") (range (start 4 26) (end 4 46)) (probe (position 4 26))
    (reference (id (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "engine::4cylEngine")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/36_variation_configuration.md") (range (start 5 32) (end 5 64)) (probe (position 5 32))
    (reference (id (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle4Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "transmission::manualTransmission")
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
  (query (document "memory://snapshot/36_variation_configuration.md") (range (start 9 26) (end 9 46)) (probe (position 9 26))
    (reference (id (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "engine::6cylEngine")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/36_variation_configuration.md") (range (start 10 32) (end 10 64)) (probe (position 10 32))
    (reference (id (source (node (document "memory://snapshot/36_variation_configuration.md") (path (named (kind package) (name "Variation Configuration")) (named (kind part) (name "vehicle6Cyl")) (anonymous (kind part) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "transmission::manualTransmission")
      (outcome (status unresolved)))
    )
  )
)
~~~
