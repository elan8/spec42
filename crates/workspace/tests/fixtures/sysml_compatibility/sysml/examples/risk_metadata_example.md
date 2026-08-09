# META
~~~ini
description=SysML Example (Metadata): RiskMetadataExample
type=file
~~~
# SOURCE
~~~sysml
package RiskMetadataExample {
	private import RiskMetadata::*;
	private import RiskLevelEnum::*;
	
    part engine4cyl{
        @Risk {
            totalRisk = high;
            technicalRisk = medium;
            scheduleRisk = medium;
        }
        @Risk {
        	totalRisk { 
        		probability = 0.3;
        		impact = 0.7;
        	}        	
        }
    }
        
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,Ident,OpenCurly,
At,Ident,OpenCurly,
Ident,Eq,Ident,Semicolon,
Ident,Eq,Ident,Semicolon,
Ident,Eq,Ident,Semicolon,
CloseCurly,
At,Ident,OpenCurly,
Ident,OpenCurly,
Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'RiskMetadataExample'
    (import_decl private 'RiskMetadata::*')
    (import_decl private 'RiskLevelEnum::*')
    (part_usage 'engine4cyl'
      (metadata_feature typed 'Risk'
        (feature_def 'totalRisk' value)
        (feature_def 'technicalRisk' value)
        (feature_def 'scheduleRisk' value))
      (metadata_feature typed 'Risk'
        (feature_def 'totalRisk'
          (feature_def 'probability' value)
          (feature_def 'impact' value))))))
~~~
# FORMAT
~~~sysml
package RiskMetadataExample {
    private import RiskMetadata::*;
    private import RiskLevelEnum::*;

    part engine4cyl{
        @Risk {
            totalRisk = high;
            technicalRisk = medium;
            scheduleRisk = medium;
        }
        @Risk {
            totalRisk {
                probability = 0.3;
                impact = 0.7;
            }
        }
    }

}

~~~
# EXPECTED
~~~
semantic.duplicate_name 'totalRisk'
semantic.unresolved_name 'Risk'
semantic.unresolved_name 'Risk'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'totalRisk'
semantic.unresolved_name 'Risk'
semantic.unresolved_name 'Risk'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "RiskMetadataExample"))) (name "RiskMetadataExample") (declared-name "RiskMetadataExample")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "RiskMetadataExample::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "RiskMetadataExample::*#import"))) (name "*") (declared-name "*"))
        (element (kind "part") (id (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl"))) (name "engine4cyl") (declared-name "engine4cyl") (declared (properties (composite true) (reference false) (ordered false)))
          (contains
            (element (kind "metadata usage") (id (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl::Risk"))) (name "Risk") (declared-name "Risk")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl::Risk::scheduleRisk"))) (name "scheduleRisk") (declared-name "scheduleRisk") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl::Risk::technicalRisk"))) (name "technicalRisk") (declared-name "technicalRisk") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl::Risk::totalRisk"))) (name "totalRisk") (declared-name "totalRisk") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
              )
            )
            (element (kind "metadata usage") (id (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl::Risk#metadata_usage"))) (name "Risk") (declared-name "Risk"))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl::Risk"))) (to (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl::Risk#metadata_usage"))) (to (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl"))))
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
  (document "sysml/examples/risk_metadata_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 1) (end 2 33))
      )
      (diagnostic
        (severity warning)
        (code "metadata_annotation_unresolved")
        (source "semantic")
        (range (start 5 8) (end 5 126))
      )
      (diagnostic
        (severity warning)
        (code "metadata_annotation_unresolved")
        (source "semantic")
        (range (start 10 8) (end 10 120))
      )
    )
  )
)
~~~
