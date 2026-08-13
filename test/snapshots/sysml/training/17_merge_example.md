# META
~~~ini
description=SysML Training 17 (Control): Merge Example
type=file
~~~
# SOURCE
~~~sysml
package 'Merge Example' {
	part def Scene;
	part def Image;
	part def Picture;
	
	action def Focus { in item scene : Scene; out item image : Image; }
	action def Shoot { in item image : Image; out item picture : Picture; }
	action def Display { in item picture : Picture; }
	action def TakePicture;
	
	action takePicture : TakePicture {
		first start;
		
		then merge continue;
			
		then action trigger {
			out item scene : Scene;
		}
		
		flow from trigger.scene to focus.scene;
		
		then action focus : Focus {
			in item scene;
			out item image;
		}
		
		flow from focus.image to shoot.image;
		
		then action shoot : Shoot {
			in item image ;
			out item picture;
		}
		
		flow from shoot.picture to display.picture;
		
		then action display : Display {
			in item picture;
		}
		
		then continue;	
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/17_merge_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 8) (end 11 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 13 2) (end 13 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 15 2) (end 17 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 19 2) (end 19 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 21 2) (end 24 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 26 2) (end 26 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 28 2) (end 31 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 33 2) (end 33 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 35 2) (end 37 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 39 2) (end 39 16))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:14f9be9806a3c8307ecc8c0e471a4568d737d693247c43184b4e77774cebcd82") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Display"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Display::picture"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Picture"))))
    (declaration (id (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Focus"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Focus::image"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Image"))))
    (declaration (id (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Focus::scene"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Scene"))))
    (declaration (id (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Image"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Picture"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Scene"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Shoot"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Shoot::image"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Image"))))
    (declaration (id (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Shoot::picture"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Picture"))))
    (declaration (id (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::TakePicture"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::takePicture"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TakePicture"))))
    (declaration (id (node (document "memory://snapshot/17_merge_example.md") (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "start"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Display::picture"))) (kind featureTyping) (ordinal 0))
      (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Picture")))))
    (reference (id (source (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Focus::image"))) (kind featureTyping) (ordinal 0))
      (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Image")))))
    (reference (id (source (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Focus::scene"))) (kind featureTyping) (ordinal 0))
      (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Scene")))))
    (reference (id (source (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Shoot::image"))) (kind featureTyping) (ordinal 0))
      (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Image")))))
    (reference (id (source (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Shoot::picture"))) (kind featureTyping) (ordinal 0))
      (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Picture")))))
    (reference (id (source (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::takePicture"))) (kind featureTyping) (ordinal 0))
      (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::TakePicture")))))
    (reference (id (source (node (document "memory://snapshot/17_merge_example.md") (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "start")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Display::picture"))) (target (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Picture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Display::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Focus::image"))) (target (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Image"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Focus::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Focus::scene"))) (target (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Scene"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Focus::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Shoot::image"))) (target (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Image"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Shoot::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Shoot::picture"))) (target (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Picture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Shoot::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::takePicture"))) (target (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::TakePicture"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::takePicture"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/17_merge_example.md") (range (start 7 40) (end 7 47)) (probe (position 7 40))
    (reference (id (source (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Display::picture"))) (kind featureTyping) (ordinal 0) (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Picture")))))
  )
  (query (document "memory://snapshot/17_merge_example.md") (range (start 5 60) (end 5 65)) (probe (position 5 60))
    (reference (id (source (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Focus::image"))) (kind featureTyping) (ordinal 0) (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Image")))))
  )
  (query (document "memory://snapshot/17_merge_example.md") (range (start 5 36) (end 5 41)) (probe (position 5 36))
    (reference (id (source (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Focus::scene"))) (kind featureTyping) (ordinal 0) (authored-target "Scene")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Scene")))))
  )
  (query (document "memory://snapshot/17_merge_example.md") (range (start 6 36) (end 6 41)) (probe (position 6 36))
    (reference (id (source (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Shoot::image"))) (kind featureTyping) (ordinal 0) (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Image")))))
  )
  (query (document "memory://snapshot/17_merge_example.md") (range (start 6 62) (end 6 69)) (probe (position 6 62))
    (reference (id (source (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Shoot::picture"))) (kind featureTyping) (ordinal 0) (authored-target "Picture")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::Picture")))))
  )
  (query (document "memory://snapshot/17_merge_example.md") (range (start 10 22) (end 10 33)) (probe (position 10 22))
    (reference (id (source (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::takePicture"))) (kind featureTyping) (ordinal 0) (authored-target "TakePicture")
      (outcome (status resolved) (target (node (document "memory://snapshot/17_merge_example.md") (qualified-name "Merge Example::TakePicture")))))
  )
  (query (document "memory://snapshot/17_merge_example.md") (range (start 11 8) (end 11 13)) (probe (position 11 8))
    (reference (id (source (node (document "memory://snapshot/17_merge_example.md") (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "start")
      (outcome (status unresolved)))
  )
)
~~~
