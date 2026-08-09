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

        first [0..1] startShot then [1] 'merge'::TakePicture_snapshots.'merge';
        first [1] 'merge'::TakePicture_snapshots.'merge' then [1] focus::TakePicture_snapshots.focus;
        first [1] focus::TakePicture_snapshots.focus then shoot::TakePicture_snapshots.shoot;
        first [1] shoot::TakePicture_snapshots.shoot then [1] 'decide'::TakePicture_snapshots.'decide';
        first [0..1] 'decide'::TakePicture_snapshots.'decide' then [0..1] 'merge'::TakePicture_snapshots.'merge';
        first [1] 'decide'::TakePicture_snapshots.'decide' then [0..1] endShot;
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
# SMG
~~~
(model
  (namespace
    (package 'TimeVaryingSteps'
      (behavior_def 'TakePicture'
        (step_def 'merge' : 'ControlPerformances::MergePerformance'[unresolved]
          (multiplicity_range [0..1])
          (feature_def 'TakePicture_snapshots' :>> 'Occurrences::Occurrence::snapshots'[unresolved]
            (membership_import public -> 'TimeVaryingSteps::TakePicture::merge'[step_def])))
        (step_def 'focus'
          (multiplicity_range [0..1])
          (feature_def 'TakePicture_snapshots' :>> 'Occurrences::Occurrence::snapshots'[unresolved]
            (membership_import public -> 'TimeVaryingSteps::TakePicture::focus'[step_def])))
        (step_def 'shoot'
          (multiplicity_range [0..1])
          (feature_def 'TakePicture_snapshots' :>> 'Occurrences::Occurrence::snapshots'[unresolved]
            (membership_import public -> 'TimeVaryingSteps::TakePicture::shoot'[step_def])))
        (step_def 'decide' : 'ControlPerformances::DecisionPerformance'[unresolved]
          (multiplicity_range [0..1])
          (feature_def 'TakePicture_snapshots' :>> 'Occurrences::Occurrence::snapshots'[unresolved]
            (membership_import public -> 'TimeVaryingSteps::TakePicture::decide'[step_def])))
        (succession_def
          (connector_end 'startShot')
          (connector_end ''merge'::TakePicture_snapshots.'merge''))
        (succession_def
          (connector_end ''merge'::TakePicture_snapshots.'merge'')
          (connector_end 'focus::TakePicture_snapshots.focus'))
        (succession_def
          (connector_end 'focus::TakePicture_snapshots.focus')
          (connector_end 'shoot::TakePicture_snapshots.shoot'))
        (succession_def
          (connector_end 'shoot::TakePicture_snapshots.shoot')
          (connector_end ''decide'::TakePicture_snapshots.'decide''))
        (succession_def
          (connector_end ''decide'::TakePicture_snapshots.'decide'')
          (connector_end ''merge'::TakePicture_snapshots.'merge''))
        (succession_def
          (connector_end ''decide'::TakePicture_snapshots.'decide'')
          (connector_end 'endShot')))
      (structure_def 'Camera'
        (step_def 'takePic' : 'TimeVaryingSteps::TakePicture'[behavior_def]
          (multiplicity_range [1])
          (feature_def 'Camera_snapshots' :>> 'Occurrences::Occurrence::snapshots'[unresolved])))
      (structure_def 'MultiCamera'
        (step_def 'takePics' : 'TimeVaryingSteps::TakePicture'[behavior_def]
          (multiplicity_range [0..*])
          (feature_def 'Camera_snapshots' :>> 'Occurrences::Occurrence::snapshots'[unresolved]))))))
~~~
