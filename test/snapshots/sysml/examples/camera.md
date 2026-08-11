# META
~~~ini
description=SysML Example (Camera): Camera
type=file
~~~
# SOURCE
~~~sysml
part def Camera {
	private import PictureTaking::*;
	
	perform action takePicture[*] :> PictureTaking::takePicture;
	
	part focusingSubsystem {
		perform takePicture.focus;
	}
	
	part imagingSubsystem {
		perform takePicture.shoot;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "camera.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "sysml")
        (range (start 3 1) (end 3 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 1) (end 5 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 1) (end 9 56))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
part def Camera {
    private import PictureTaking::*;

    perform action takePicture[*] :> PictureTaking::takePicture;

    part focusingSubsystem {
        perform takePicture.focus;
    }

    part imagingSubsystem {
        perform takePicture.shoot;
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "2b5d836d254f62c2255d5cb1f0011736b166da63af27110833e1cbd0ad3134cc") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Camera"))) (kind "part def") (name "Camera") (declared-name "Camera"))
    (element (id (node (document "d0") (qualified-name "Camera::focusingSubsystem"))) (kind "part") (name "focusingSubsystem") (declared-name "focusingSubsystem") (parent (node (document "d0") (qualified-name "Camera"))) (authored (membership (kind Feature)) (relationships (perform (reference "Camera::focusingSubsystem::takePicture::focus")))))
    (element (id (node (document "d0") (qualified-name "Camera::focusingSubsystem::takePicture.focus"))) (kind "action") (name "takePicture.focus") (declared-name "takePicture.focus") (parent (node (document "d0") (qualified-name "Camera::focusingSubsystem"))))
    (element (id (node (document "d0") (qualified-name "Camera::imagingSubsystem"))) (kind "part") (name "imagingSubsystem") (declared-name "imagingSubsystem") (parent (node (document "d0") (qualified-name "Camera"))) (authored (membership (kind Feature)) (relationships (perform (reference "Camera::imagingSubsystem::takePicture::shoot")))))
    (element (id (node (document "d0") (qualified-name "Camera::imagingSubsystem::takePicture.shoot"))) (kind "action") (name "takePicture.shoot") (declared-name "takePicture.shoot") (parent (node (document "d0") (qualified-name "Camera::imagingSubsystem"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Camera::focusingSubsystem"))) (kind performSource) (ordinal 0)) (authored-target "Camera::focusingSubsystem::takePicture::focus") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Camera::imagingSubsystem"))) (kind performSource) (ordinal 0)) (authored-target "Camera::imagingSubsystem::takePicture::shoot") (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
