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

    part engine4cyl {
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
(model
  (namespace
    (package 'RiskMetadataExample'
      (namespace_import private -> 'RiskMetadata'[unresolved])
      (namespace_import private -> 'RiskLevelEnum'[unresolved])
      (part_usage 'engine4cyl'
        (metadata_usage :> 'Risk'[unresolved]
          (feature_def 'totalRisk'
            (feature_value (=)))
          (feature_def 'technicalRisk'
            (feature_value (=)))
          (feature_def 'scheduleRisk'
            (feature_value (=))))
        (metadata_usage :> 'Risk'[unresolved]
          (feature_def 'totalRisk'
            (feature_def 'probability'
              (feature_value (=)))
            (feature_def 'impact'
              (feature_value (=)))))))))
~~~
