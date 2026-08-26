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
  (document "memory://snapshot/time_varying_steps.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 25) (end 3 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 82) (end 3 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 45) (end 4 79))
      )
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 5 19) (end 5 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 40) (end 10 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 45) (end 11 79))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 40) (end 17 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 45) (end 18 79))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 26) (end 24 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 24 86) (end 24 107))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 25 45) (end 25 79))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 30 3) (end 31 3))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 31 3) (end 32 4))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 32 4) (end 33 4))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 33 4) (end 34 4))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 34 4) (end 35 4))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 35 4) (end 36 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 41 52) (end 41 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 42 39) (end 42 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 49 56) (end 49 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 50 39) (end 50 73))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:0a36bc093b26843f7d8032d2797a7a2ea4143a798a653afffca0b1b0a908c3be") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::Camera"))) (kind kerml-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::Camera::takePic"))) (kind kerml-step) (membership (kind feature) (visibility default)) (facts (modifiers member) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TakePicture")) (typeFeaturing (reference "Camera_snapshots")))))
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::Camera::takePic::Camera_snapshots"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers member)) (authored (membership (kind feature) (visibility default)) (relationships (typeFeaturing (reference "Camera")) (redefinition (reference "Occurrences::Occurrence::snapshots")))))
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::MultiCamera"))) (kind kerml-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::MultiCamera::takePics"))) (kind kerml-step) (membership (kind feature) (visibility default)) (facts (modifiers member) (multiplicity (lower 0) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TakePicture")) (typeFeaturing (reference "Camera_snapshots")))))
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::MultiCamera::takePics::Camera_snapshots"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers member)) (authored (membership (kind feature) (visibility default)) (relationships (typeFeaturing (reference "Camera")) (redefinition (reference "Occurrences::Occurrence::snapshots")))))
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture"))) (kind kerml-behavior) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::decide"))) (kind kerml-step) (membership (kind feature) (visibility default)) (facts (modifiers member) (multiplicity (lower 0) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ControlPerformances::DecisionPerformance")) (typeFeaturing (reference "TakePicture_snapshots")))))
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::decide::TakePicture_snapshots"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers member)) (authored (membership (kind feature) (visibility default)) (relationships (typeFeaturing (reference "TakePicture")) (redefinition (reference "Occurrences::Occurrence::snapshots")))))
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (path (named (kind package) (name "TimeVaryingSteps")) (named (kind kerml-behavior) (name "TakePicture")) (named (kind kerml-step) (name "decide")) (named (kind kerml-feature) (name "TakePicture_snapshots")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "decide") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::focus"))) (kind kerml-step) (membership (kind feature) (visibility default)) (facts (modifiers member) (multiplicity (lower 0) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (typeFeaturing (reference "TakePicture_snapshots")))))
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::focus::TakePicture_snapshots"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers member)) (authored (membership (kind feature) (visibility default)) (relationships (typeFeaturing (reference "TakePicture")) (redefinition (reference "Occurrences::Occurrence::snapshots")))))
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (path (named (kind package) (name "TimeVaryingSteps")) (named (kind kerml-behavior) (name "TakePicture")) (named (kind kerml-step) (name "focus")) (named (kind kerml-feature) (name "TakePicture_snapshots")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "focus") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::merge"))) (kind kerml-step) (membership (kind feature) (visibility default)) (facts (modifiers member) (multiplicity (lower 0) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ControlPerformances::MergePerformance")) (typeFeaturing (reference "TakePicture_snapshots")))))
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::merge::TakePicture_snapshots"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers member)) (authored (membership (kind feature) (visibility default)) (relationships (typeFeaturing (reference "TakePicture")) (redefinition (reference "Occurrences::Occurrence::snapshots")))))
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (path (named (kind package) (name "TimeVaryingSteps")) (named (kind kerml-behavior) (name "TakePicture")) (named (kind kerml-step) (name "merge")) (named (kind kerml-feature) (name "TakePicture_snapshots")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "merge") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::shoot"))) (kind kerml-step) (membership (kind feature) (visibility default)) (facts (modifiers member) (multiplicity (lower 0) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (typeFeaturing (reference "TakePicture_snapshots")))))
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::shoot::TakePicture_snapshots"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers member)) (authored (membership (kind feature) (visibility default)) (relationships (typeFeaturing (reference "TakePicture")) (redefinition (reference "Occurrences::Occurrence::snapshots")))))
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (path (named (kind package) (name "TimeVaryingSteps")) (named (kind kerml-behavior) (name "TakePicture")) (named (kind kerml-step) (name "shoot")) (named (kind kerml-feature) (name "TakePicture_snapshots")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "shoot") (import (shape membership) (recursive false))))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::Camera::takePic"))) (kind featureTyping) (ordinal 0))
      (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::Camera::takePic"))) (kind typeFeaturing) (ordinal 0))
      (authored-target "Camera_snapshots")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::Camera::takePic::Camera_snapshots"))) (kind typeFeaturing) (ordinal 0))
      (authored-target "Camera")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::Camera")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::Camera::takePic::Camera_snapshots"))) (kind redefinition) (ordinal 0))
      (authored-target "Occurrences::Occurrence::snapshots")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::MultiCamera::takePics"))) (kind featureTyping) (ordinal 0))
      (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::MultiCamera::takePics"))) (kind typeFeaturing) (ordinal 0))
      (authored-target "Camera_snapshots")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::MultiCamera::takePics::Camera_snapshots"))) (kind typeFeaturing) (ordinal 0))
      (authored-target "Camera")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::Camera")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::MultiCamera::takePics::Camera_snapshots"))) (kind redefinition) (ordinal 0))
      (authored-target "Occurrences::Occurrence::snapshots")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::decide"))) (kind featureTyping) (ordinal 0))
      (authored-target "ControlPerformances::DecisionPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::decide"))) (kind typeFeaturing) (ordinal 0))
      (authored-target "TakePicture_snapshots")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::decide::TakePicture_snapshots"))) (kind typeFeaturing) (ordinal 0))
      (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::decide::TakePicture_snapshots"))) (kind redefinition) (ordinal 0))
      (authored-target "Occurrences::Occurrence::snapshots")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (path (named (kind package) (name "TimeVaryingSteps")) (named (kind kerml-behavior) (name "TakePicture")) (named (kind kerml-step) (name "decide")) (named (kind kerml-feature) (name "TakePicture_snapshots")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "decide")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::decide")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::focus"))) (kind typeFeaturing) (ordinal 0))
      (authored-target "TakePicture_snapshots")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::focus::TakePicture_snapshots"))) (kind typeFeaturing) (ordinal 0))
      (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::focus::TakePicture_snapshots"))) (kind redefinition) (ordinal 0))
      (authored-target "Occurrences::Occurrence::snapshots")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (path (named (kind package) (name "TimeVaryingSteps")) (named (kind kerml-behavior) (name "TakePicture")) (named (kind kerml-step) (name "focus")) (named (kind kerml-feature) (name "TakePicture_snapshots")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "focus")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::focus")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::merge"))) (kind featureTyping) (ordinal 0))
      (authored-target "ControlPerformances::MergePerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::merge"))) (kind typeFeaturing) (ordinal 0))
      (authored-target "TakePicture_snapshots")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::merge::TakePicture_snapshots"))) (kind typeFeaturing) (ordinal 0))
      (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::merge::TakePicture_snapshots"))) (kind redefinition) (ordinal 0))
      (authored-target "Occurrences::Occurrence::snapshots")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (path (named (kind package) (name "TimeVaryingSteps")) (named (kind kerml-behavior) (name "TakePicture")) (named (kind kerml-step) (name "merge")) (named (kind kerml-feature) (name "TakePicture_snapshots")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "merge")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::merge")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::shoot"))) (kind typeFeaturing) (ordinal 0))
      (authored-target "TakePicture_snapshots")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::shoot::TakePicture_snapshots"))) (kind typeFeaturing) (ordinal 0))
      (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::shoot::TakePicture_snapshots"))) (kind redefinition) (ordinal 0))
      (authored-target "Occurrences::Occurrence::snapshots")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (path (named (kind package) (name "TimeVaryingSteps")) (named (kind kerml-behavior) (name "TakePicture")) (named (kind kerml-step) (name "shoot")) (named (kind kerml-feature) (name "TakePicture_snapshots")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "shoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::shoot")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::Camera::takePic"))) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::Camera::takePic"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::Camera::takePic::Camera_snapshots"))) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::Camera"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::Camera::takePic::Camera_snapshots"))) (kind typeFeaturing) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::MultiCamera::takePics"))) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::MultiCamera::takePics"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::MultiCamera::takePics::Camera_snapshots"))) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::Camera"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::MultiCamera::takePics::Camera_snapshots"))) (kind typeFeaturing) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::decide::TakePicture_snapshots"))) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::decide::TakePicture_snapshots"))) (kind typeFeaturing) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::focus::TakePicture_snapshots"))) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::focus::TakePicture_snapshots"))) (kind typeFeaturing) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::merge::TakePicture_snapshots"))) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::merge::TakePicture_snapshots"))) (kind typeFeaturing) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::shoot::TakePicture_snapshots"))) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::shoot::TakePicture_snapshots"))) (kind typeFeaturing) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::Camera::takePic")))
      (type (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture")) (provenance authored))
      (effective-type (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture")) (source direct))
      (supertype (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::Camera::takePic::Camera_snapshots")))
      (featured-by (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::Camera")))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::MultiCamera::takePics")))
      (type (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture")) (provenance authored))
      (effective-type (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture")) (source direct))
      (supertype (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::MultiCamera::takePics::Camera_snapshots")))
      (featured-by (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::Camera")))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture")))
      (subtype (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::Camera::takePic")) (scopes any))
      (subtype (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::MultiCamera::takePics")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::decide::TakePicture_snapshots")))
      (featured-by (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture")))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::focus::TakePicture_snapshots")))
      (featured-by (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture")))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::merge::TakePicture_snapshots")))
      (featured-by (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture")))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::shoot::TakePicture_snapshots")))
      (featured-by (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/time_varying_steps.md") (range (start 41 24) (end 41 35)) (probe (position 41 24))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::Camera::takePic"))) (kind featureTyping) (ordinal 0) (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture")))))
    )
  )
  (query (document "memory://snapshot/time_varying_steps.md") (range (start 41 52) (end 41 68)) (probe (position 41 52))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::Camera::takePic"))) (kind typeFeaturing) (ordinal 0) (authored-target "Camera_snapshots")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_steps.md") (range (start 42 86) (end 42 92)) (probe (position 42 86))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::Camera::takePic::Camera_snapshots"))) (kind typeFeaturing) (ordinal 0) (authored-target "Camera")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::Camera")))))
    )
  )
  (query (document "memory://snapshot/time_varying_steps.md") (range (start 42 39) (end 42 73)) (probe (position 42 39))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::Camera::takePic::Camera_snapshots"))) (kind redefinition) (ordinal 0) (authored-target "Occurrences::Occurrence::snapshots")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_steps.md") (range (start 49 25) (end 49 36)) (probe (position 49 25))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::MultiCamera::takePics"))) (kind featureTyping) (ordinal 0) (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture")))))
    )
  )
  (query (document "memory://snapshot/time_varying_steps.md") (range (start 49 56) (end 49 72)) (probe (position 49 56))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::MultiCamera::takePics"))) (kind typeFeaturing) (ordinal 0) (authored-target "Camera_snapshots")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_steps.md") (range (start 50 86) (end 50 92)) (probe (position 50 86))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::MultiCamera::takePics::Camera_snapshots"))) (kind typeFeaturing) (ordinal 0) (authored-target "Camera")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::Camera")))))
    )
  )
  (query (document "memory://snapshot/time_varying_steps.md") (range (start 50 39) (end 50 73)) (probe (position 50 39))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::MultiCamera::takePics::Camera_snapshots"))) (kind redefinition) (ordinal 0) (authored-target "Occurrences::Occurrence::snapshots")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_steps.md") (range (start 24 26) (end 24 66)) (probe (position 24 26))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::decide"))) (kind featureTyping) (ordinal 0) (authored-target "ControlPerformances::DecisionPerformance")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_steps.md") (range (start 24 86) (end 24 107)) (probe (position 24 86))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::decide"))) (kind typeFeaturing) (ordinal 0) (authored-target "TakePicture_snapshots")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_steps.md") (range (start 25 92) (end 25 103)) (probe (position 25 92))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::decide::TakePicture_snapshots"))) (kind typeFeaturing) (ordinal 0) (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture")))))
    )
  )
  (query (document "memory://snapshot/time_varying_steps.md") (range (start 25 45) (end 25 79)) (probe (position 25 45))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::decide::TakePicture_snapshots"))) (kind redefinition) (ordinal 0) (authored-target "Occurrences::Occurrence::snapshots")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_steps.md") (range (start 26 19) (end 26 27)) (probe (position 26 19))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (path (named (kind package) (name "TimeVaryingSteps")) (named (kind kerml-behavior) (name "TakePicture")) (named (kind kerml-step) (name "decide")) (named (kind kerml-feature) (name "TakePicture_snapshots")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "decide")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::decide")))))
    )
  )
  (query (document "memory://snapshot/time_varying_steps.md") (range (start 10 40) (end 10 61)) (probe (position 10 40))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::focus"))) (kind typeFeaturing) (ordinal 0) (authored-target "TakePicture_snapshots")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_steps.md") (range (start 11 92) (end 11 103)) (probe (position 11 92))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::focus::TakePicture_snapshots"))) (kind typeFeaturing) (ordinal 0) (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture")))))
    )
  )
  (query (document "memory://snapshot/time_varying_steps.md") (range (start 11 45) (end 11 79)) (probe (position 11 45))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::focus::TakePicture_snapshots"))) (kind redefinition) (ordinal 0) (authored-target "Occurrences::Occurrence::snapshots")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_steps.md") (range (start 12 19) (end 12 24)) (probe (position 12 19))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (path (named (kind package) (name "TimeVaryingSteps")) (named (kind kerml-behavior) (name "TakePicture")) (named (kind kerml-step) (name "focus")) (named (kind kerml-feature) (name "TakePicture_snapshots")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "focus")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::focus")))))
    )
  )
  (query (document "memory://snapshot/time_varying_steps.md") (range (start 3 25) (end 3 62)) (probe (position 3 25))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::merge"))) (kind featureTyping) (ordinal 0) (authored-target "ControlPerformances::MergePerformance")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_steps.md") (range (start 3 82) (end 3 103)) (probe (position 3 82))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::merge"))) (kind typeFeaturing) (ordinal 0) (authored-target "TakePicture_snapshots")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_steps.md") (range (start 4 92) (end 4 103)) (probe (position 4 92))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::merge::TakePicture_snapshots"))) (kind typeFeaturing) (ordinal 0) (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture")))))
    )
  )
  (query (document "memory://snapshot/time_varying_steps.md") (range (start 4 45) (end 4 79)) (probe (position 4 45))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::merge::TakePicture_snapshots"))) (kind redefinition) (ordinal 0) (authored-target "Occurrences::Occurrence::snapshots")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_steps.md") (range (start 5 19) (end 5 26)) (probe (position 5 19))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (path (named (kind package) (name "TimeVaryingSteps")) (named (kind kerml-behavior) (name "TakePicture")) (named (kind kerml-step) (name "merge")) (named (kind kerml-feature) (name "TakePicture_snapshots")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "merge")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::merge")))))
    )
  )
  (query (document "memory://snapshot/time_varying_steps.md") (range (start 17 40) (end 17 61)) (probe (position 17 40))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::shoot"))) (kind typeFeaturing) (ordinal 0) (authored-target "TakePicture_snapshots")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_steps.md") (range (start 18 92) (end 18 103)) (probe (position 18 92))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::shoot::TakePicture_snapshots"))) (kind typeFeaturing) (ordinal 0) (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture")))))
    )
  )
  (query (document "memory://snapshot/time_varying_steps.md") (range (start 18 45) (end 18 79)) (probe (position 18 45))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::shoot::TakePicture_snapshots"))) (kind redefinition) (ordinal 0) (authored-target "Occurrences::Occurrence::snapshots")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_steps.md") (range (start 19 19) (end 19 24)) (probe (position 19 19))
    (reference (id (source (node (document "memory://snapshot/time_varying_steps.md") (path (named (kind package) (name "TimeVaryingSteps")) (named (kind kerml-behavior) (name "TakePicture")) (named (kind kerml-step) (name "shoot")) (named (kind kerml-feature) (name "TakePicture_snapshots")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "shoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_steps.md") (qualified-name "TimeVaryingSteps::TakePicture::shoot")))))
    )
  )
)
~~~
