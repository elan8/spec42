# META
~~~ini
description=SysML Training 14 (Action Definitions): Action Shorthand Example
type=file
~~~
# SOURCE
~~~sysml
package 'Action Shorthand Example' {
	item def Scene;
	item def Image;
	item def Picture;
	
	action def Focus { in scene : Scene; out image : Image; }
	action def Shoot { in image: Image; out picture : Picture; }	
				
	action def TakePicture {
		in item scene : Scene;
		out item picture : Picture;
		
		action focus: Focus {
			in item scene = TakePicture::scene;
			out item image;
		}
		
		flow from focus.image to shoot.image;
		
		then action shoot: Shoot {
			in item;
			out item picture = TakePicture::picture;
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/14_action_shorthand_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 27) (end 17 38))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:6409babd991a20b892be47396abad82d5e8da9f8e41b377e6a10f72342e58bb0"))
  (declarations
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus::image"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Image") (direction out)))))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus::scene"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Scene") (direction in)))))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Image"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Picture"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Scene"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot::image"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Image") (direction in)))))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot::picture"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Picture") (direction out)))))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (anonymous (kind flow) (ordinal 0))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "focus::image")) (flowTarget (reference "shoot::image")))))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Focus")))))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus::image"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus::scene"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)) (feature-value (kind bind) (value (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (named (kind action) (name "focus")) (named (kind item) (name "scene")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (named (kind action) (name "focus")) (named (kind item) (name "scene")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (named (kind action) (name "focus")) (named (kind item) (name "scene")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (named (kind action) (name "focus")) (named (kind item) (name "scene")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (named (kind action) (name "focus")) (named (kind item) (name "scene")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::picture"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Picture")))))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::scene"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Scene")))))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::shoot"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Shoot")))))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (named (kind action) (name "shoot")) (anonymous (kind item) (ordinal 0))))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::shoot::picture"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)) (feature-value (kind bind) (value (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (named (kind action) (name "shoot")) (named (kind item) (name "picture")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (named (kind action) (name "shoot")) (named (kind item) (name "picture")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (named (kind action) (name "shoot")) (named (kind item) (name "picture")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (named (kind action) (name "shoot")) (named (kind item) (name "picture")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (named (kind action) (name "shoot")) (named (kind item) (name "picture")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus::image"))) (kind featureTyping) (ordinal 0))
      (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Image")))))
    (reference (id (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus::scene"))) (kind featureTyping) (ordinal 0))
      (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Scene")))))
    (reference (id (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot::image"))) (kind featureTyping) (ordinal 0))
      (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Image")))))
    (reference (id (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot::picture"))) (kind featureTyping) (ordinal 0))
      (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Picture")))))
    (reference (id (source (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0))
      (authored-target "focus::image")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus::image")))))
    (reference (id (source (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0))
      (authored-target "shoot::image")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus"))) (kind featureTyping) (ordinal 0))
      (authored-target "Focus")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus")))))
    (reference (id (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::picture"))) (kind featureTyping) (ordinal 0))
      (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Picture")))))
    (reference (id (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::scene"))) (kind featureTyping) (ordinal 0))
      (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Scene")))))
    (reference (id (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::shoot"))) (kind featureTyping) (ordinal 0))
      (authored-target "Shoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot")))))
  )
  (relationships
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus::image"))) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Image"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus::scene"))) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Scene"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot::image"))) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Image"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot::picture"))) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Picture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus::image"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus"))) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::picture"))) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Picture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::scene"))) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Scene"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::shoot"))) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::shoot"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus::image"))) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus::scene"))) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot::image"))) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot::picture"))) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus"))) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus::image"))) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus::scene"))) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (named (kind action) (name "focus")) (named (kind item) (name "scene")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (named (kind action) (name "focus")) (named (kind item) (name "scene")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::picture"))) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::scene"))) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::shoot"))) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (named (kind action) (name "shoot")) (anonymous (kind item) (ordinal 0))))) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::shoot"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::shoot::picture"))) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::shoot"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (named (kind action) (name "shoot")) (named (kind item) (name "picture")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (named (kind action) (name "shoot")) (named (kind item) (name "picture")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus")))
      (subtype (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus::image")))
      (featured-by (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus")))
      (type (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Image")) (provenance authored))
      (effective-type (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Image")) (source direct))
      (supertype (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Image")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus::scene")))
      (featured-by (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus")))
      (type (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Scene")) (provenance authored))
      (effective-type (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Scene")) (source direct))
      (supertype (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Scene")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Image")))
      (subtype (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus::image")) (scopes any))
      (subtype (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot::image")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Picture")))
      (subtype (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot::picture")) (scopes any))
      (subtype (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::picture")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Scene")))
      (subtype (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus::scene")) (scopes any))
      (subtype (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::scene")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot")))
      (subtype (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::shoot")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot::image")))
      (featured-by (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot")))
      (type (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Image")) (provenance authored))
      (effective-type (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Image")) (source direct))
      (supertype (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Image")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot::picture")))
      (featured-by (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot")))
      (type (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Picture")) (provenance authored))
      (effective-type (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Picture")) (source direct))
      (supertype (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Picture")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (anonymous (kind flow) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture")))
    )
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus")))
      (featured-by (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture")))
      (type (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus")) (provenance authored))
      (effective-type (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus")) (source direct))
      (supertype (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus::image")))
      (featured-by (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus")))
    )
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus::scene")))
      (featured-by (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus")))
    )
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (named (kind action) (name "focus")) (named (kind item) (name "scene")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (named (kind action) (name "focus")) (named (kind item) (name "scene")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::picture")))
      (featured-by (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture")))
      (type (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Picture")) (provenance authored))
      (effective-type (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Picture")) (source direct))
      (supertype (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Picture")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::scene")))
      (featured-by (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture")))
      (type (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Scene")) (provenance authored))
      (effective-type (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Scene")) (source direct))
      (supertype (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Scene")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::shoot")))
      (featured-by (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture")))
      (type (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot")) (provenance authored))
      (effective-type (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot")) (source direct))
      (supertype (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (named (kind action) (name "shoot")) (anonymous (kind item) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::shoot")))
    )
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::shoot::picture")))
      (featured-by (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::shoot")))
    )
    (declaration (id (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (named (kind action) (name "shoot")) (named (kind item) (name "picture")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (named (kind action) (name "shoot")) (named (kind item) (name "picture")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/14_action_shorthand_example.md") (range (start 5 50) (end 5 55)) (probe (position 5 50))
    (reference (id (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus::image"))) (kind featureTyping) (ordinal 0) (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Image")))))
    )
  )
  (query (document "memory://snapshot/14_action_shorthand_example.md") (range (start 5 31) (end 5 36)) (probe (position 5 31))
    (reference (id (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus::scene"))) (kind featureTyping) (ordinal 0) (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Scene")))))
    )
  )
  (query (document "memory://snapshot/14_action_shorthand_example.md") (range (start 6 30) (end 6 35)) (probe (position 6 30))
    (reference (id (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot::image"))) (kind featureTyping) (ordinal 0) (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Image")))))
    )
  )
  (query (document "memory://snapshot/14_action_shorthand_example.md") (range (start 6 51) (end 6 58)) (probe (position 6 51))
    (reference (id (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot::picture"))) (kind featureTyping) (ordinal 0) (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Picture")))))
    )
  )
  (query (document "memory://snapshot/14_action_shorthand_example.md") (range (start 17 12) (end 17 23)) (probe (position 17 12))
    (reference (id (source (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0) (authored-target "focus::image")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus::image")))))
    )
  )
  (query (document "memory://snapshot/14_action_shorthand_example.md") (range (start 17 27) (end 17 38)) (probe (position 17 27))
    (reference (id (source (node (document "memory://snapshot/14_action_shorthand_example.md") (path (named (kind package) (name "Action Shorthand Example")) (named (kind action-def) (name "TakePicture")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0) (authored-target "shoot::image")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/14_action_shorthand_example.md") (range (start 12 16) (end 12 21)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::focus"))) (kind featureTyping) (ordinal 0) (authored-target "Focus")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Focus")))))
    )
  )
  (query (document "memory://snapshot/14_action_shorthand_example.md") (range (start 10 21) (end 10 28)) (probe (position 10 21))
    (reference (id (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::picture"))) (kind featureTyping) (ordinal 0) (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Picture")))))
    )
  )
  (query (document "memory://snapshot/14_action_shorthand_example.md") (range (start 9 18) (end 9 23)) (probe (position 9 18))
    (reference (id (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::scene"))) (kind featureTyping) (ordinal 0) (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Scene")))))
    )
  )
  (query (document "memory://snapshot/14_action_shorthand_example.md") (range (start 19 21) (end 19 26)) (probe (position 19 21))
    (reference (id (source (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::TakePicture::shoot"))) (kind featureTyping) (ordinal 0) (authored-target "Shoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/14_action_shorthand_example.md") (qualified-name "Action Shorthand Example::Shoot")))))
    )
  )
)
~~~
