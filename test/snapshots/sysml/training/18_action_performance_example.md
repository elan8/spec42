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
  (document "memory://snapshot/18_action_performance_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 41))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "parser")
        (range (start 9 2) (end 12 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:4792da0794e77a0d60357e60c04671e31362262b5ed5dd78100aad24ae05af99") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/18_action_performance_example.md") (path (name "Action Performance Example") (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Action Decomposition") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example::AutoFocus"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example::Camera"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example::Imager"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example::camera"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Camera"))))
    (declaration (id (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example::camera::f"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AutoFocus"))))
    (declaration (id (node (document "memory://snapshot/18_action_performance_example.md") (path (name "Action Performance Example") (name "camera") (name "f") (anonymous (kind perform-action) (ordinal 0)))))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example::camera::i"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Imager"))))
    (declaration (id (node (document "memory://snapshot/18_action_performance_example.md") (path (name "Action Performance Example") (name "camera") (name "i") (anonymous (kind perform-action) (ordinal 0)))))) (kind perform-action) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/18_action_performance_example.md") (path (name "Action Performance Example") (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Action Decomposition")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example::camera"))) (kind featureTyping) (ordinal 0))
      (authored-target "Camera")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example::Camera")))))
    (reference (id (source (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example::camera::f"))) (kind featureTyping) (ordinal 0))
      (authored-target "AutoFocus")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example::AutoFocus")))))
    (reference (id (source (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example::camera::i"))) (kind featureTyping) (ordinal 0))
      (authored-target "Imager")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example::Imager")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example::camera"))) (target (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example::Camera"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example::camera"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example::camera::f"))) (target (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example::AutoFocus"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example::camera::f"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example::camera::i"))) (target (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example::Imager"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example::camera::i"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/18_action_performance_example.md") (range (start 1 16) (end 1 41)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/18_action_performance_example.md") (path (name "Action Performance Example") (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "Action Decomposition")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/18_action_performance_example.md") (range (start 7 15) (end 7 21)) (probe (position 7 15))
    (reference (id (source (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example::camera"))) (kind featureTyping) (ordinal 0) (authored-target "Camera")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example::Camera")))))
  )
  (query (document "memory://snapshot/18_action_performance_example.md") (range (start 12 11) (end 12 20)) (probe (position 12 11))
    (reference (id (source (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example::camera::f"))) (kind featureTyping) (ordinal 0) (authored-target "AutoFocus")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example::AutoFocus")))))
  )
  (query (document "memory://snapshot/18_action_performance_example.md") (range (start 16 11) (end 16 17)) (probe (position 16 11))
    (reference (id (source (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example::camera::i"))) (kind featureTyping) (ordinal 0) (authored-target "Imager")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_action_performance_example.md") (qualified-name "Action Performance Example::Imager")))))
  )
)
~~~
