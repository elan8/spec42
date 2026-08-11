# META
~~~ini
description=KerML Enhancements: TimeVaryingSteps
type=file
~~~
# SOURCE
~~~kerml
package TimeVaryingSteps {
	behavior TakePicture {
 		// var step merge : MergePerformance [0..1];
 		member step 'merge' : ControlPerformances::MergePerformance [0..1] featured by TakePicture_snapshots {
 			member feature TakePicture_snapshots :>> Occurrences::Occurrence::snapshots featured by TakePicture {
 				public import 'merge';
 			}
 		}

		// var step focus [0..1];
 		member step focus [0..1] featured by TakePicture_snapshots {
 			member feature TakePicture_snapshots :>> Occurrences::Occurrence::snapshots featured by TakePicture {
 				public import focus;
 			}
 		}

 		// var step shoot [0..1];
 		member step shoot [0..1] featured by TakePicture_snapshots {
 			member feature TakePicture_snapshots :>> Occurrences::Occurrence::snapshots featured by TakePicture {
 				public import shoot;
 			}
 		}

 		// var step decide : DecisionPerformance [0..1];
 		member step 'decide' : ControlPerformances::DecisionPerformance [0..1] featured by TakePicture_snapshots {
 			member feature TakePicture_snapshots :>> Occurrences::Occurrence::snapshots featured by TakePicture {
 				public import 'decide';
 			}
 		}

 		succession first [0..1] startShot then  [1] 'merge'::TakePicture_snapshots.'merge';
 		succession first [1] 'merge'::TakePicture_snapshots.'merge' then [1] focus::TakePicture_snapshots.focus;
  		succession first [1] focus::TakePicture_snapshots.focus then shoot::TakePicture_snapshots.shoot;
  		succession first [1] shoot::TakePicture_snapshots.shoot then [1] 'decide'::TakePicture_snapshots.'decide';
  		succession first [0..1] 'decide'::TakePicture_snapshots.'decide' then [0..1] 'merge'::TakePicture_snapshots.'merge';
  		succession first [1] 'decide'::TakePicture_snapshots.'decide' then[0..1] endShot;
  	}
	
	struct Camera {
		// Is always taking a picture, one at a time.
		// var step takePic : TakePicture [1];		
		member step takePic : TakePicture [1] featured by Camera_snapshots {
			member feature Camera_snapshots :>> Occurrences::Occurrence::snapshots featured by Camera;
		}
	}

	struct MultiCamera {
		// Can take many pictures at one time.
		// var step takePics : TakePicture [0..*];		
		member step takePics : TakePicture [0..*] featured by Camera_snapshots {
			member feature Camera_snapshots :>> Occurrences::Occurrence::snapshots featured by Camera;
		}
	}

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "time_varying_steps.md"
    (diagnostics
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwBehavior,Ident,OpenCurly,
LineComment,
KwMember,KwStep,UnrestrictedName,Colon,Ident,ColonColon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeatured,KwBy,Ident,OpenCurly,
KwMember,KwFeature,Ident,ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,OpenCurly,
KwPublic,KwImport,UnrestrictedName,Semicolon,
CloseCurly,
CloseCurly,
LineComment,
KwMember,KwStep,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeatured,KwBy,Ident,OpenCurly,
KwMember,KwFeature,Ident,ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,OpenCurly,
KwPublic,KwImport,Ident,Semicolon,
CloseCurly,
CloseCurly,
LineComment,
KwMember,KwStep,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeatured,KwBy,Ident,OpenCurly,
KwMember,KwFeature,Ident,ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,OpenCurly,
KwPublic,KwImport,Ident,Semicolon,
CloseCurly,
CloseCurly,
LineComment,
KwMember,KwStep,UnrestrictedName,Colon,Ident,ColonColon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeatured,KwBy,Ident,OpenCurly,
KwMember,KwFeature,Ident,ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,OpenCurly,
KwPublic,KwImport,UnrestrictedName,Semicolon,
CloseCurly,
CloseCurly,
KwSuccession,KwFirst,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,KwThen,OpenSquare,DecimalValue,CloseSquare,UnrestrictedName,ColonColon,Ident,Dot,UnrestrictedName,Semicolon,
KwSuccession,KwFirst,OpenSquare,DecimalValue,CloseSquare,UnrestrictedName,ColonColon,Ident,Dot,UnrestrictedName,KwThen,OpenSquare,DecimalValue,CloseSquare,Ident,ColonColon,Ident,Dot,Ident,Semicolon,
KwSuccession,KwFirst,OpenSquare,DecimalValue,CloseSquare,Ident,ColonColon,Ident,Dot,Ident,KwThen,Ident,ColonColon,Ident,Dot,Ident,Semicolon,
KwSuccession,KwFirst,OpenSquare,DecimalValue,CloseSquare,Ident,ColonColon,Ident,Dot,Ident,KwThen,OpenSquare,DecimalValue,CloseSquare,UnrestrictedName,ColonColon,Ident,Dot,UnrestrictedName,Semicolon,
KwSuccession,KwFirst,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,UnrestrictedName,ColonColon,Ident,Dot,UnrestrictedName,KwThen,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,UnrestrictedName,ColonColon,Ident,Dot,UnrestrictedName,Semicolon,
KwSuccession,KwFirst,OpenSquare,DecimalValue,CloseSquare,UnrestrictedName,ColonColon,Ident,Dot,UnrestrictedName,KwThen,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
CloseCurly,
KwStruct,Ident,OpenCurly,
LineComment,
LineComment,
KwMember,KwStep,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwFeatured,KwBy,Ident,OpenCurly,
KwMember,KwFeature,Ident,ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwStruct,Ident,OpenCurly,
LineComment,
LineComment,
KwMember,KwStep,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwFeatured,KwBy,Ident,OpenCurly,
KwMember,KwFeature,Ident,ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'TimeVaryingSteps'
    (behavior_def
      (line_comment)
      (step_def
        (feature_def member 'TakePicture_snapshots' :>> 'Occurrences::Occurrence::snapshots' featured by 'TakePicture'
          (import_decl public ''merge'')))
      (line_comment)
      (step_def
        (feature_def member 'TakePicture_snapshots' :>> 'Occurrences::Occurrence::snapshots' featured by 'TakePicture'
          (import_decl public 'focus')))
      (line_comment)
      (step_def
        (feature_def member 'TakePicture_snapshots' :>> 'Occurrences::Occurrence::snapshots' featured by 'TakePicture'
          (import_decl public 'shoot')))
      (line_comment)
      (step_def
        (feature_def member 'TakePicture_snapshots' :>> 'Occurrences::Occurrence::snapshots' featured by 'TakePicture'
          (import_decl public ''decide'')))
      (succession_as_usage
        (connector_end)
        (connector_end))
      (succession_as_usage
        (connector_end)
        (connector_end))
      (succession_as_usage
        (connector_end)
        (connector_end))
      (succession_as_usage
        (connector_end)
        (connector_end))
      (succession_as_usage
        (connector_end)
        (connector_end))
      (succession_as_usage
        (connector_end)
        (connector_end)))
    (structure_def 'Camera'
      (line_comment)
      (line_comment)
      (step_def
        (feature_def member 'Camera_snapshots' :>> 'Occurrences::Occurrence::snapshots' featured by 'Camera')))
    (structure_def 'MultiCamera'
      (line_comment)
      (line_comment)
      (step_def
        (feature_def member 'Camera_snapshots' :>> 'Occurrences::Occurrence::snapshots' featured by 'Camera')))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'ControlPerformances::MergePerformance'
semantic.unresolved_name 'TakePicture_snapshots'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
semantic.unresolved_name 'TakePicture_snapshots'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
semantic.unresolved_name 'TakePicture_snapshots'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
semantic.unresolved_name 'ControlPerformances::DecisionPerformance'
semantic.unresolved_name 'TakePicture_snapshots'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
semantic.unresolved_name 'Camera_snapshots'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
semantic.unresolved_name 'Camera_snapshots'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ControlPerformances::MergePerformance'
semantic.unresolved_name 'TakePicture_snapshots'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
semantic.unresolved_name 'TakePicture_snapshots'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
semantic.unresolved_name 'TakePicture_snapshots'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
semantic.unresolved_name 'ControlPerformances::DecisionPerformance'
semantic.unresolved_name 'TakePicture_snapshots'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
semantic.unresolved_name 'Camera_snapshots'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
semantic.unresolved_name 'Camera_snapshots'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
~~~
# FORMAT
~~~sysml
package TimeVaryingSteps {
	behavior TakePicture {
 		// var step merge : MergePerformance [0..1];
 		member step 'merge' : ControlPerformances::MergePerformance [0..1] featured by TakePicture_snapshots {
 			member feature TakePicture_snapshots :>> Occurrences::Occurrence::snapshots featured by TakePicture {
 				public import 'merge';
 			}
 		}

		// var step focus [0..1];
 		member step focus [0..1] featured by TakePicture_snapshots {
 			member feature TakePicture_snapshots :>> Occurrences::Occurrence::snapshots featured by TakePicture {
 				public import focus;
 			}
 		}

 		// var step shoot [0..1];
 		member step shoot [0..1] featured by TakePicture_snapshots {
 			member feature TakePicture_snapshots :>> Occurrences::Occurrence::snapshots featured by TakePicture {
 				public import shoot;
 			}
 		}

 		// var step decide : DecisionPerformance [0..1];
 		member step 'decide' : ControlPerformances::DecisionPerformance [0..1] featured by TakePicture_snapshots {
 			member feature TakePicture_snapshots :>> Occurrences::Occurrence::snapshots featured by TakePicture {
 				public import 'decide';
 			}
 		}

 		succession first [0..1] startShot then  [1] 'merge'::TakePicture_snapshots.'merge';
 		succession first [1] 'merge'::TakePicture_snapshots.'merge' then [1] focus::TakePicture_snapshots.focus;
  		succession first [1] focus::TakePicture_snapshots.focus then shoot::TakePicture_snapshots.shoot;
  		succession first [1] shoot::TakePicture_snapshots.shoot then [1] 'decide'::TakePicture_snapshots.'decide';
  		succession first [0..1] 'decide'::TakePicture_snapshots.'decide' then [0..1] 'merge'::TakePicture_snapshots.'merge';
  		succession first [1] 'decide'::TakePicture_snapshots.'decide' then[0..1] endShot;
  	}
	
	struct Camera {
		// Is always taking a picture, one at a time.
		// var step takePic : TakePicture [1];		
		member step takePic : TakePicture [1] featured by Camera_snapshots {
			member feature Camera_snapshots :>> Occurrences::Occurrence::snapshots featured by Camera;
		}
	}

	struct MultiCamera {
		// Can take many pictures at one time.
		// var step takePics : TakePicture [0..*];		
		member step takePics : TakePicture [0..*] featured by Camera_snapshots {
			member feature Camera_snapshots :>> Occurrences::Occurrence::snapshots featured by Camera;
		}
	}

}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "78cac9099ca667ebf5a425c56631645d4b43a562ac2f8058f041691f843dd8bc") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "TimeVaryingSteps"))) (kind "package") (name "TimeVaryingSteps") (declared-name "TimeVaryingSteps") (range (start (line 0) (character 0)) (end (line 0) (character 2323))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingSteps::Camera"))) (kind "classifier decl") (name "Camera") (declared-name "Camera") (range (start (line 38) (character 1)) (end (line 38) (character 279))) (parent (node (document "d0") (qualified-name "TimeVaryingSteps"))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingSteps::MultiCamera"))) (kind "classifier decl") (name "MultiCamera") (declared-name "MultiCamera") (range (start (line 46) (character 1)) (end (line 46) (character 285))) (parent (node (document "d0") (qualified-name "TimeVaryingSteps"))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingSteps::TakePicture"))) (kind "kermlDecl") (name "TakePicture") (declared-name "TakePicture") (range (start (line 1) (character 1)) (end (line 1) (character 1724))) (parent (node (document "d0") (qualified-name "TimeVaryingSteps"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
