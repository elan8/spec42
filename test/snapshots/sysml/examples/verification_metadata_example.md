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
  (document "verification_metadata_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 38))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "822800e1d7f96b08660982aa7e650dd15ed67acf64ead1ec4e157dc5de7fcda1") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "VerificationMetadataExample"))) (kind "package") (name "VerificationMetadataExample") (declared-name "VerificationMetadataExample"))
    (element (id (node (document "d0") (qualified-name "VerificationMetadataExample::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VerificationMetadataExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "VerificationCases::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VerificationMetadataExample::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VerificationMetadataExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "VerificationMethodKind::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VerificationMetadataExample::MassTest"))) (kind "verification def") (name "MassTest") (declared-name "MassTest") (parent (node (document "d0") (qualified-name "VerificationMetadataExample"))))
    (element (id (node (document "d0") (qualified-name "VerificationMetadataExample::massTests"))) (kind "verification") (name "massTests") (declared-name "massTests") (parent (node (document "d0") (qualified-name "VerificationMetadataExample"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassTest")))))
    (element (id (node (document "d0") (qualified-name "VerificationMetadataExample::massTests::VerificationMethod"))) (kind "metadata usage") (name "VerificationMethod") (declared-name "VerificationMethod") (parent (node (document "d0") (qualified-name "VerificationMetadataExample::massTests"))))
    (element (id (node (document "d0") (qualified-name "VerificationMetadataExample::massTests::VerificationMethod::kind"))) (kind "attribute") (name "kind") (declared-name "kind") (parent (node (document "d0") (qualified-name "VerificationMetadataExample::massTests::VerificationMethod"))))
    (element (id (node (document "d0") (qualified-name "VerificationMetadataExample::massTests::objective"))) (kind "objective") (name "objective") (declared-name "objective") (parent (node (document "d0") (qualified-name "VerificationMetadataExample::massTests"))))
    (element (id (node (document "d0") (qualified-name "VerificationMetadataExample::massTests::weighVehicle"))) (kind "action") (name "weighVehicle") (declared-name "weighVehicle") (parent (node (document "d0") (qualified-name "VerificationMetadataExample::massTests"))))
    (element (id (node (document "d0") (qualified-name "VerificationMetadataExample::massTests::weighVehicle::VerificationMethod"))) (kind "metadata usage") (name "VerificationMethod") (declared-name "VerificationMethod") (parent (node (document "d0") (qualified-name "VerificationMetadataExample::massTests::weighVehicle"))))
    (element (id (node (document "d0") (qualified-name "VerificationMetadataExample::massTests::weighVehicle::VerificationMethod::kind"))) (kind "attribute") (name "kind") (declared-name "kind") (parent (node (document "d0") (qualified-name "VerificationMetadataExample::massTests::weighVehicle::VerificationMethod"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VerificationMetadataExample::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "VerificationCases::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VerificationMetadataExample::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "VerificationMethodKind::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VerificationMetadataExample::massTests"))) (kind featureTyping) (ordinal 0)) (authored-target "MassTest") (outcome (status resolved) (target (node (document "d0") (qualified-name "VerificationMetadataExample::MassTest")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VerificationMetadataExample::massTests"))) (target (node (document "d0") (qualified-name "VerificationMetadataExample::MassTest"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VerificationMetadataExample::massTests"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 1 16) (end 1 33)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "VerificationMetadataExample::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "VerificationCases::*")
        (range (start 1 16) (end 1 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 38)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "VerificationMetadataExample::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "VerificationMethodKind::*")
        (range (start 2 16) (end 2 38))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
