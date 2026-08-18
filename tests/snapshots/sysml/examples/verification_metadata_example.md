# META
~~~ini
description=SysML Example (Metadata): VerificationMetadataExample
type=file
~~~
# SOURCE
~~~sysml
package VerificationMetadataExample {
	private import VerificationCases::*;
	private import VerificationMethodKind::*;
	
    verification def MassTest;
    verification massTests:MassTest {
        @VerificationMethod{ kind = (test,demo); }
        objective {
        }
        action weighVehicle {
        	@VerificationMethod{ kind = analyze; }
        }
    }
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/verification_metadata_example.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 9) (end 6 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 37) (end 6 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 42) (end 6 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 10) (end 10 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 37) (end 10 44))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:156dde3c7b698e68a879dce95ae4026bc5b91e7b745b8418f598ef0bfe690100") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_metadata_example.md") (path (named (kind package) (name "VerificationMetadataExample")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "VerificationCases") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/verification_metadata_example.md") (path (named (kind package) (name "VerificationMetadataExample")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "VerificationMethodKind") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample::MassTest"))) (kind verification-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample::massTests"))) (kind verification) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassTest")) (metadataAnnotation (reference "VerificationMethod")))))
    (declaration (id (node (document "memory://snapshot/verification_metadata_example.md") (path (named (kind package) (name "VerificationMetadataExample")) (named (kind verification) (name "massTests")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_metadata_example.md") (path (named (kind package) (name "VerificationMetadataExample")) (named (kind verification) (name "massTests")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "kind"))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "test")) (expressionOperand (reference "demo")))))
    (declaration (id (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample::massTests::objective"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample::massTests::weighVehicle"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "VerificationMethod")))))
    (declaration (id (node (document "memory://snapshot/verification_metadata_example.md") (path (named (kind package) (name "VerificationMetadataExample")) (named (kind verification) (name "massTests")) (named (kind action) (name "weighVehicle")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/verification_metadata_example.md") (path (named (kind package) (name "VerificationMetadataExample")) (named (kind verification) (name "massTests")) (named (kind action) (name "weighVehicle")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "kind"))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "analyze")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/verification_metadata_example.md") (path (named (kind package) (name "VerificationMetadataExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "VerificationCases")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_metadata_example.md") (path (named (kind package) (name "VerificationMetadataExample")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "VerificationMethodKind")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample::massTests"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassTest")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample::MassTest")))))
    (reference (id (source (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample::massTests"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "VerificationMethod")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_metadata_example.md") (path (named (kind package) (name "VerificationMetadataExample")) (named (kind verification) (name "massTests")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "kind"))))) (kind expressionOperand) (ordinal 0))
      (authored-target "test")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_metadata_example.md") (path (named (kind package) (name "VerificationMetadataExample")) (named (kind verification) (name "massTests")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "kind"))))) (kind expressionOperand) (ordinal 1))
      (authored-target "demo")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample::massTests::weighVehicle"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "VerificationMethod")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/verification_metadata_example.md") (path (named (kind package) (name "VerificationMetadataExample")) (named (kind verification) (name "massTests")) (named (kind action) (name "weighVehicle")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "kind"))))) (kind expressionOperand) (ordinal 0))
      (authored-target "analyze")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample::massTests"))) (target (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample::MassTest"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample::massTests"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/verification_metadata_example.md") (path (named (kind package) (name "VerificationMetadataExample")) (named (kind verification) (name "massTests")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "kind"))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/verification_metadata_example.md") (path (named (kind package) (name "VerificationMetadataExample")) (named (kind verification) (name "massTests")) (named (kind action) (name "weighVehicle")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "kind"))))) (state unresolved-operand))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample::MassTest")))
      (subtype (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample::massTests")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample::massTests")))
      (type (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample::MassTest")) (provenance authored))
      (effective-type (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample::MassTest")) (source direct))
      (supertype (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample::MassTest")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/verification_metadata_example.md") (path (named (kind package) (name "VerificationMetadataExample")) (named (kind verification) (name "massTests")) (anonymous (kind metadata) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample::massTests")))
    )
    (declaration (id (node (document "memory://snapshot/verification_metadata_example.md") (path (named (kind package) (name "VerificationMetadataExample")) (named (kind verification) (name "massTests")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "kind")))))
      (featured-by (node (document "memory://snapshot/verification_metadata_example.md") (path (named (kind package) (name "VerificationMetadataExample")) (named (kind verification) (name "massTests")) (anonymous (kind metadata) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample::massTests::objective")))
      (featured-by (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample::massTests")))
    )
    (declaration (id (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample::massTests::weighVehicle")))
      (featured-by (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample::massTests")))
    )
    (declaration (id (node (document "memory://snapshot/verification_metadata_example.md") (path (named (kind package) (name "VerificationMetadataExample")) (named (kind verification) (name "massTests")) (named (kind action) (name "weighVehicle")) (anonymous (kind metadata) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample::massTests::weighVehicle")))
    )
    (declaration (id (node (document "memory://snapshot/verification_metadata_example.md") (path (named (kind package) (name "VerificationMetadataExample")) (named (kind verification) (name "massTests")) (named (kind action) (name "weighVehicle")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "kind")))))
      (featured-by (node (document "memory://snapshot/verification_metadata_example.md") (path (named (kind package) (name "VerificationMetadataExample")) (named (kind verification) (name "massTests")) (named (kind action) (name "weighVehicle")) (anonymous (kind metadata) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/verification_metadata_example.md") (range (start 1 16) (end 1 36)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/verification_metadata_example.md") (path (named (kind package) (name "VerificationMetadataExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "VerificationCases")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_metadata_example.md") (range (start 2 16) (end 2 41)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/verification_metadata_example.md") (path (named (kind package) (name "VerificationMetadataExample")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "VerificationMethodKind")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_metadata_example.md") (range (start 5 27) (end 5 35)) (probe (position 5 27))
    (reference (id (source (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample::massTests"))) (kind featureTyping) (ordinal 0) (authored-target "MassTest")
      (outcome (status resolved) (target (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample::MassTest")))))
    )
  )
  (query (document "memory://snapshot/verification_metadata_example.md") (range (start 6 9) (end 6 27)) (probe (position 6 9))
    (reference (id (source (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample::massTests"))) (kind metadataAnnotation) (ordinal 0) (authored-target "VerificationMethod")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_metadata_example.md") (range (start 6 37) (end 6 41)) (probe (position 6 37))
    (reference (id (source (node (document "memory://snapshot/verification_metadata_example.md") (path (named (kind package) (name "VerificationMetadataExample")) (named (kind verification) (name "massTests")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "kind"))))) (kind expressionOperand) (ordinal 0) (authored-target "test")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_metadata_example.md") (range (start 6 42) (end 6 46)) (probe (position 6 42))
    (reference (id (source (node (document "memory://snapshot/verification_metadata_example.md") (path (named (kind package) (name "VerificationMetadataExample")) (named (kind verification) (name "massTests")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "kind"))))) (kind expressionOperand) (ordinal 1) (authored-target "demo")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_metadata_example.md") (range (start 10 10) (end 10 28)) (probe (position 10 10))
    (reference (id (source (node (document "memory://snapshot/verification_metadata_example.md") (qualified-name "VerificationMetadataExample::massTests::weighVehicle"))) (kind metadataAnnotation) (ordinal 0) (authored-target "VerificationMethod")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/verification_metadata_example.md") (range (start 10 37) (end 10 44)) (probe (position 10 37))
    (reference (id (source (node (document "memory://snapshot/verification_metadata_example.md") (path (named (kind package) (name "VerificationMetadataExample")) (named (kind verification) (name "massTests")) (named (kind action) (name "weighVehicle")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "kind"))))) (kind expressionOperand) (ordinal 0) (authored-target "analyze")
      (outcome (status unresolved)))
    )
  )
)
~~~
