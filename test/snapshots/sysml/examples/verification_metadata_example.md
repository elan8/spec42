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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwVerification,KwDef,Ident,Semicolon,
KwVerification,Ident,Colon,Ident,OpenCurly,
At,Ident,OpenCurly,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
KwObjective,OpenCurly,
CloseCurly,
KwAction,Ident,OpenCurly,
At,Ident,OpenCurly,Ident,Eq,Ident,Semicolon,CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'VerificationMetadataExample'
    (import_decl private 'VerificationCases::*')
    (import_decl private 'VerificationMethodKind::*')
    (verification_case_def 'MassTest')
    (sysml_decl 'massTests' : 'MassTest'
      (metadata_feature typed 'VerificationMethod'
        (feature_def 'kind' value))
      (objective_member)
      (action_usage 'weighVehicle'
        (metadata_feature typed 'VerificationMethod'
          (feature_def 'kind' value))))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'VerificationMethod'
semantic.unresolved_name 'VerificationMethod'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'VerificationMethod'
semantic.unresolved_name 'VerificationMethod'
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "822800e1d7f96b08660982aa7e650dd15ed67acf64ead1ec4e157dc5de7fcda1") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "VerificationMetadataExample"))) (kind "package") (name "VerificationMetadataExample") (declared-name "VerificationMetadataExample") (range (start (line 0) (character 0)) (end (line 0) (character 368))))
    (element (id (node (document "d0") (qualified-name "VerificationMetadataExample::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 37))) (parent (node (document "d0") (qualified-name "VerificationMetadataExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "VerificationCases::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 33))))))
    (element (id (node (document "d0") (qualified-name "VerificationMetadataExample::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 42))) (parent (node (document "d0") (qualified-name "VerificationMetadataExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "VerificationMethodKind::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 38))))))
    (element (id (node (document "d0") (qualified-name "VerificationMetadataExample::MassTest"))) (kind "verification def") (name "MassTest") (declared-name "MassTest") (range (start (line 4) (character 4)) (end (line 4) (character 30))) (parent (node (document "d0") (qualified-name "VerificationMetadataExample"))))
    (element (id (node (document "d0") (qualified-name "VerificationMetadataExample::massTests"))) (kind "verification") (name "massTests") (declared-name "massTests") (range (start (line 5) (character 4)) (end (line 5) (character 212))) (parent (node (document "d0") (qualified-name "VerificationMetadataExample"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassTest") (range none)))))
    (element (id (node (document "d0") (qualified-name "VerificationMetadataExample::massTests::VerificationMethod"))) (kind "metadata usage") (name "VerificationMethod") (declared-name "VerificationMethod") (range (start (line 6) (character 8)) (end (line 6) (character 50))) (parent (node (document "d0") (qualified-name "VerificationMetadataExample::massTests"))))
    (element (id (node (document "d0") (qualified-name "VerificationMetadataExample::massTests::VerificationMethod::kind"))) (kind "attribute") (name "kind") (declared-name "kind") (range (start (line 6) (character 29)) (end (line 6) (character 48))) (parent (node (document "d0") (qualified-name "VerificationMetadataExample::massTests::VerificationMethod"))))
    (element (id (node (document "d0") (qualified-name "VerificationMetadataExample::massTests::objective"))) (kind "objective") (name "objective") (declared-name "objective") (range (start (line 7) (character 8)) (end (line 7) (character 29))) (parent (node (document "d0") (qualified-name "VerificationMetadataExample::massTests"))))
    (element (id (node (document "d0") (qualified-name "VerificationMetadataExample::massTests::weighVehicle"))) (kind "action") (name "weighVehicle") (declared-name "weighVehicle") (range (start (line 9) (character 8)) (end (line 9) (character 87))) (parent (node (document "d0") (qualified-name "VerificationMetadataExample::massTests"))))
    (element (id (node (document "d0") (qualified-name "VerificationMetadataExample::massTests::weighVehicle::VerificationMethod"))) (kind "metadata usage") (name "VerificationMethod") (declared-name "VerificationMethod") (range (start (line 10) (character 9)) (end (line 10) (character 47))) (parent (node (document "d0") (qualified-name "VerificationMetadataExample::massTests::weighVehicle"))))
    (element (id (node (document "d0") (qualified-name "VerificationMetadataExample::massTests::weighVehicle::VerificationMethod::kind"))) (kind "attribute") (name "kind") (declared-name "kind") (range (start (line 10) (character 30)) (end (line 10) (character 45))) (parent (node (document "d0") (qualified-name "VerificationMetadataExample::massTests::weighVehicle::VerificationMethod"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VerificationMetadataExample::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "VerificationCases::*") (range (start (line 1) (character 16)) (end (line 1) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VerificationMetadataExample::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "VerificationMethodKind::*") (range (start (line 2) (character 16)) (end (line 2) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VerificationMetadataExample::massTests"))) (kind featureTyping) (ordinal 0)) (authored-target "MassTest") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VerificationMetadataExample::MassTest")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VerificationMetadataExample::massTests"))) (target (node (document "d0") (qualified-name "VerificationMetadataExample::MassTest"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VerificationMetadataExample::massTests"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
