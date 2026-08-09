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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "VerificationMetadataExample"))) (name "VerificationMetadataExample") (declared-name "VerificationMetadataExample")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "VerificationMetadataExample::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VerificationMetadataExample::*#import"))) (name "*") (declared-name "*"))
        (element (kind "verification def") (id (node (document "d0") (qualified-name "VerificationMetadataExample::MassTest"))) (name "MassTest") (declared-name "MassTest"))
        (element (kind "verification") (id (node (document "d0") (qualified-name "VerificationMetadataExample::massTests"))) (name "massTests") (declared-name "massTests")
          (contains
            (element (kind "metadata usage") (id (node (document "d0") (qualified-name "VerificationMetadataExample::massTests::VerificationMethod"))) (name "VerificationMethod") (declared-name "VerificationMethod") (effective (featuring-type (node (document "d0") (qualified-name "VerificationMetadataExample::MassTest"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "VerificationMetadataExample::massTests::VerificationMethod::kind"))) (name "kind") (declared-name "kind") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "VerificationMetadataExample::MassTest")))))
              )
            )
            (element (kind "objective") (id (node (document "d0") (qualified-name "VerificationMetadataExample::massTests::objective"))) (name "objective") (declared-name "objective") (effective (featuring-type (node (document "d0") (qualified-name "VerificationMetadataExample::MassTest")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "VerificationMetadataExample::massTests::weighVehicle"))) (name "weighVehicle") (declared-name "weighVehicle") (declared) (effective (featuring-type (node (document "d0") (qualified-name "VerificationMetadataExample::MassTest"))))
              (contains
                (element (kind "metadata usage") (id (node (document "d0") (qualified-name "VerificationMetadataExample::massTests::weighVehicle::VerificationMethod"))) (name "VerificationMethod") (declared-name "VerificationMethod") (effective (featuring-type (node (document "d0") (qualified-name "VerificationMetadataExample::MassTest"))))
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "VerificationMetadataExample::massTests::weighVehicle::VerificationMethod::kind"))) (name "kind") (declared-name "kind") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "VerificationMetadataExample::MassTest")))))
                  )
                )
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VerificationMetadataExample::massTests::VerificationMethod"))) (to (node (document "d0") (qualified-name "VerificationMetadataExample::massTests"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VerificationMetadataExample::massTests::weighVehicle::VerificationMethod"))) (to (node (document "d0") (qualified-name "VerificationMetadataExample::massTests::weighVehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VerificationMetadataExample::massTests"))) (to (node (document "d0") (qualified-name "VerificationMetadataExample::MassTest"))))
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
  (document "sysml/examples/verification_metadata_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 1) (end 2 42))
      )
      (diagnostic
        (severity warning)
        (code "metadata_annotation_unresolved")
        (source "semantic")
        (range (start 6 8) (end 6 50))
      )
      (diagnostic
        (severity warning)
        (code "objective_binding_unresolved")
        (source "semantic")
        (range (start 7 8) (end 7 29))
      )
      (diagnostic
        (severity warning)
        (code "metadata_annotation_unresolved")
        (source "semantic")
        (range (start 10 9) (end 10 47))
      )
    )
  )
)
~~~
