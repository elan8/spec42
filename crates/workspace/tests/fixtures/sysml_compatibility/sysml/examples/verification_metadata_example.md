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
    verification massTests : MassTest {
        @VerificationMethod {
            kind = (test,demo);
        }
        objective { }
        action weighVehicle {
            @VerificationMethod {
                kind = analyze;
            }
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
(model
  (namespace
    (package 'VerificationMetadataExample'
      (namespace_import private -> 'VerificationCases'[unresolved])
      (namespace_import private -> 'VerificationMethodKind'[unresolved])
      (verification_case_def 'MassTest')
      (verification_case_usage 'massTests' : 'VerificationMetadataExample::MassTest'[verification_case_def]
        (metadata_usage :> 'VerificationMethod'[unresolved]
          (feature_def 'kind'
            (feature_value (=))))
        (objective_membership composite)
        (action_usage composite 'weighVehicle'
          (metadata_usage :> 'VerificationMethod'[unresolved]
            (feature_def 'kind'
              (feature_value (=)))))))))
~~~
