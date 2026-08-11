# META
~~~ini
description=SysML Training 18 (Action Performance): Action Performance Example
type=file
~~~
# SOURCE
~~~sysml
package 'Action Performance Example' {
	private import 'Action Decomposition'::*;
	
	part def Camera;
	part def AutoFocus;
	part def Imager;
	
	part camera : Camera {
		
		perform action takePhoto[*] ordered 
			references takePicture;
		
		part f : AutoFocus {
			perform takePhoto.focus;			
		}
		
		part i : Imager {
			perform takePhoto.shoot;
		}		
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "18_action_performance_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 38))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 9 2) (end 9 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 2) (end 12 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 2) (end 16 51))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Action Performance Example' {
    private import 'Action Decomposition'::*;

    part def Camera;
    part def AutoFocus;
    part def Imager;

    part camera : Camera {

        perform action takePhoto[*] ordered
        references takePicture;

        part f : AutoFocus {
            perform takePhoto.focus;
        }

        part i : Imager {
            perform takePhoto.shoot;
        }
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "ef7f067945a959c77269de27c186a2cf1e9f24940f3a583d6d86adc63841258c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Action Performance Example"))) (kind "package") (name "Action Performance Example") (declared-name "Action Performance Example"))
    (element (id (node (document "d0") (qualified-name "Action Performance Example::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Action Performance Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Action Decomposition::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Action Performance Example::AutoFocus"))) (kind "part def") (name "AutoFocus") (declared-name "AutoFocus") (parent (node (document "d0") (qualified-name "Action Performance Example"))))
    (element (id (node (document "d0") (qualified-name "Action Performance Example::Camera"))) (kind "part def") (name "Camera") (declared-name "Camera") (parent (node (document "d0") (qualified-name "Action Performance Example"))))
    (element (id (node (document "d0") (qualified-name "Action Performance Example::Imager"))) (kind "part def") (name "Imager") (declared-name "Imager") (parent (node (document "d0") (qualified-name "Action Performance Example"))))
    (element (id (node (document "d0") (qualified-name "Action Performance Example::camera"))) (kind "part") (name "camera") (declared-name "camera") (parent (node (document "d0") (qualified-name "Action Performance Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Camera")))))
    (element (id (node (document "d0") (qualified-name "Action Performance Example::camera::f"))) (kind "part") (name "f") (declared-name "f") (parent (node (document "d0") (qualified-name "Action Performance Example::camera"))) (authored (membership (kind Feature)) (relationships (typing (reference "AutoFocus")) (perform (reference "Action Performance Example::camera::f::takePhoto::focus")))))
    (element (id (node (document "d0") (qualified-name "Action Performance Example::camera::f::takePhoto.focus"))) (kind "action") (name "takePhoto.focus") (declared-name "takePhoto.focus") (parent (node (document "d0") (qualified-name "Action Performance Example::camera::f"))))
    (element (id (node (document "d0") (qualified-name "Action Performance Example::camera::i"))) (kind "part") (name "i") (declared-name "i") (parent (node (document "d0") (qualified-name "Action Performance Example::camera"))) (authored (membership (kind Feature)) (relationships (typing (reference "Imager")) (perform (reference "Action Performance Example::camera::i::takePhoto::shoot")))))
    (element (id (node (document "d0") (qualified-name "Action Performance Example::camera::i::takePhoto.shoot"))) (kind "action") (name "takePhoto.shoot") (declared-name "takePhoto.shoot") (parent (node (document "d0") (qualified-name "Action Performance Example::camera::i"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Action Performance Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Action Decomposition::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Action Performance Example::camera"))) (kind featureTyping) (ordinal 0)) (authored-target "Camera") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Performance Example::Camera")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Performance Example::camera::f"))) (kind featureTyping) (ordinal 0)) (authored-target "AutoFocus") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Performance Example::AutoFocus")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Performance Example::camera::f"))) (kind performSource) (ordinal 0)) (authored-target "Action Performance Example::camera::f::takePhoto::focus") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Action Performance Example::camera::i"))) (kind featureTyping) (ordinal 0)) (authored-target "Imager") (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Performance Example::Imager")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Performance Example::camera::i"))) (kind performSource) (ordinal 0)) (authored-target "Action Performance Example::camera::i::takePhoto::shoot") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Performance Example::camera"))) (target (node (document "d0") (qualified-name "Action Performance Example::Camera"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Performance Example::camera"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Performance Example::camera::f"))) (target (node (document "d0") (qualified-name "Action Performance Example::AutoFocus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Performance Example::camera::f"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Performance Example::camera::i"))) (target (node (document "d0") (qualified-name "Action Performance Example::Imager"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Performance Example::camera::i"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 7 15) (end 7 21)) (probe (position 7 15))
      (reference
        (source (document "d0") (qualified-name "Action Performance Example::camera"))
        (kind featureTyping) (ordinal 0) (authored-target "Camera")
        (range (start 7 15) (end 7 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Action Performance Example::Camera") (range (start 3 1) (end 3 17)))
        )
      )
    )
    (query (range (start 16 11) (end 16 17)) (probe (position 16 11))
      (reference
        (source (document "d0") (qualified-name "Action Performance Example::camera::i"))
        (kind featureTyping) (ordinal 0) (authored-target "Imager")
        (range (start 16 11) (end 16 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Action Performance Example::Imager") (range (start 5 1) (end 5 17)))
        )
      )
    )
    (query (range (start 12 11) (end 12 20)) (probe (position 12 11))
      (reference
        (source (document "d0") (qualified-name "Action Performance Example::camera::f"))
        (kind featureTyping) (ordinal 0) (authored-target "AutoFocus")
        (range (start 12 11) (end 12 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Action Performance Example::AutoFocus") (range (start 4 1) (end 4 20)))
        )
      )
    )
    (query (range (start 1 16) (end 1 38)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Action Performance Example::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Action Decomposition::*")
        (range (start 1 16) (end 1 38))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
