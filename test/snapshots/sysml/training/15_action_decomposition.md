# META
~~~ini
description=SysML Training 15 (Actions): Action Decomposition
type=file
~~~
# SOURCE
~~~sysml
package 'Action Decomposition' {
	part def Scene;
	part def Image;
	part def Picture;
	
	action def Focus { in scene : Scene; out image : Image; }
	action def Shoot { in image: Image; out picture : Picture; }	
	action def TakePicture { in scene : Scene; out picture : Picture; }
		
	action takePicture : TakePicture {
		in item scene;
		out item picture;
		
		action focus : Focus {
			in item scene = takePicture::scene; 
			out item image;
		}
		
		flow from focus.image to shoot.image;

		action shoot : Shoot {
			in item; 
			out item picture = takePicture::picture;
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/15_action_decomposition.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 18 2) (end 18 39))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 21 3) (end 22 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:14d4c1946452dfb045a84b0e50fe40918ac1df8967d1a71bc634f5d4487a5d1a") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Focus"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Focus::image"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Image") (direction out))))
    (declaration (id (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Focus::scene"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Scene") (direction in))))
    (declaration (id (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Image"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Picture"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Scene"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Shoot"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Shoot::image"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Image") (direction in))))
    (declaration (id (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Shoot::picture"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Picture") (direction out))))
    (declaration (id (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::TakePicture"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::TakePicture::picture"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Picture") (direction out))))
    (declaration (id (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::TakePicture::scene"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Scene") (direction in))))
    (declaration (id (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::takePicture"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TakePicture"))))
    (declaration (id (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::takePicture::focus"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Focus"))))
    (declaration (id (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::takePicture::focus::image"))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::takePicture::focus::scene"))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::takePicture::picture"))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::takePicture::scene"))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::takePicture::shoot"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Shoot"))))
    (declaration (id (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::takePicture::shoot::picture"))) (kind item) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Focus::image"))) (kind featureTyping) (ordinal 0))
      (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Image")))))
    (reference (id (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Focus::scene"))) (kind featureTyping) (ordinal 0))
      (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Scene")))))
    (reference (id (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Shoot::image"))) (kind featureTyping) (ordinal 0))
      (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Image")))))
    (reference (id (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Shoot::picture"))) (kind featureTyping) (ordinal 0))
      (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Picture")))))
    (reference (id (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::TakePicture::picture"))) (kind featureTyping) (ordinal 0))
      (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Picture")))))
    (reference (id (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::TakePicture::scene"))) (kind featureTyping) (ordinal 0))
      (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Scene")))))
    (reference (id (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::takePicture"))) (kind featureTyping) (ordinal 0))
      (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::TakePicture")))))
    (reference (id (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::takePicture::focus"))) (kind featureTyping) (ordinal 0))
      (authored-target "Focus")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Focus")))))
    (reference (id (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::takePicture::shoot"))) (kind featureTyping) (ordinal 0))
      (authored-target "Shoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Shoot")))))
  )
  (relationships
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Focus::image"))) (target (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Image"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Focus::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Focus::scene"))) (target (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Scene"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Focus::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Shoot::image"))) (target (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Image"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Shoot::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Shoot::picture"))) (target (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Picture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Shoot::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::TakePicture::picture"))) (target (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Picture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::TakePicture::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::TakePicture::scene"))) (target (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Scene"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::TakePicture::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::takePicture"))) (target (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::TakePicture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::takePicture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::takePicture::focus"))) (target (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Focus"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::takePicture::focus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::takePicture::shoot"))) (target (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Shoot"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::takePicture::shoot"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/15_action_decomposition.md") (range (start 5 50) (end 5 55)) (probe (position 5 50))
    (reference (id (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Focus::image"))) (kind featureTyping) (ordinal 0) (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Image")))))
  )
  (query (document "memory://snapshot/15_action_decomposition.md") (range (start 5 31) (end 5 36)) (probe (position 5 31))
    (reference (id (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Focus::scene"))) (kind featureTyping) (ordinal 0) (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Scene")))))
  )
  (query (document "memory://snapshot/15_action_decomposition.md") (range (start 6 30) (end 6 35)) (probe (position 6 30))
    (reference (id (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Shoot::image"))) (kind featureTyping) (ordinal 0) (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Image")))))
  )
  (query (document "memory://snapshot/15_action_decomposition.md") (range (start 6 51) (end 6 58)) (probe (position 6 51))
    (reference (id (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Shoot::picture"))) (kind featureTyping) (ordinal 0) (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Picture")))))
  )
  (query (document "memory://snapshot/15_action_decomposition.md") (range (start 7 58) (end 7 65)) (probe (position 7 58))
    (reference (id (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::TakePicture::picture"))) (kind featureTyping) (ordinal 0) (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Picture")))))
  )
  (query (document "memory://snapshot/15_action_decomposition.md") (range (start 7 37) (end 7 42)) (probe (position 7 37))
    (reference (id (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::TakePicture::scene"))) (kind featureTyping) (ordinal 0) (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Scene")))))
  )
  (query (document "memory://snapshot/15_action_decomposition.md") (range (start 9 22) (end 9 33)) (probe (position 9 22))
    (reference (id (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::takePicture"))) (kind featureTyping) (ordinal 0) (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::TakePicture")))))
  )
  (query (document "memory://snapshot/15_action_decomposition.md") (range (start 13 17) (end 13 22)) (probe (position 13 17))
    (reference (id (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::takePicture::focus"))) (kind featureTyping) (ordinal 0) (authored-target "Focus")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Focus")))))
  )
  (query (document "memory://snapshot/15_action_decomposition.md") (range (start 20 17) (end 20 22)) (probe (position 20 17))
    (reference (id (source (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::takePicture::shoot"))) (kind featureTyping) (ordinal 0) (authored-target "Shoot")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_action_decomposition.md") (qualified-name "Action Decomposition::Shoot")))))
  )
)
~~~
