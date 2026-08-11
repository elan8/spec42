# META
~~~ini
description=SysML Training 16 (Conditional Succession): Conditional Succession Example-1
type=file
~~~
# SOURCE
~~~sysml
package 'Conditional Succession Example-1' {
	part def Scene;
	part def Image {
		isWellFocused: ScalarValues::Boolean;
	}
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
				
		first focus 
			if focus.image.isWellFocused then shoot;
		
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
  (document "16_conditional_succession_example_1.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 16 3) (end 16 38))
      )
      (diagnostic
        (severity error)
        (code "missing_semicolon")
        (source "sysml")
        (range (start 20 2) (end 20 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 23 27) (end 23 38))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "sysml")
        (range (start 26 3) (end 26 16))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 27 3) (end 27 43))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwAction,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwOut,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwOut,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwOut,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,KwItem,Ident,Semicolon,
KwOut,KwItem,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,KwItem,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwOut,KwItem,Ident,Semicolon,
CloseCurly,
KwFirst,Ident,
KwIf,Ident,Dot,Ident,Dot,Ident,KwThen,Ident,Semicolon,
KwFlow,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,KwItem,Semicolon,
KwOut,KwItem,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Conditional Succession Example-1''
    (part_def 'Scene')
    (part_def 'Image'
      (default_ref_usage 'isWellFocused' : 'ScalarValues::Boolean'))
    (part_def 'Picture')
    (action_def 'Focus'
      (default_ref_usage in 'scene' : 'Scene')
      (default_ref_usage out 'image' : 'Image'))
    (action_def 'Shoot'
      (default_ref_usage in 'image' : 'Image')
      (default_ref_usage out 'picture' : 'Picture'))
    (action_def 'TakePicture'
      (default_ref_usage in 'scene' : 'Scene')
      (default_ref_usage out 'picture' : 'Picture'))
    (action_usage 'takePicture' : 'TakePicture'
      (item_usage in 'scene')
      (item_usage out 'picture')
      (action_usage 'focus' : 'Focus'
        (item_usage in 'scene' value)
        (item_usage out 'image'))
      (initial_node focus)
      (if_node)
      (source_succession
        (default_ref_usage 'shoot'))
      (flow_usage
        (connector_end)
        (connector_end))
      (action_usage 'shoot' : 'Shoot'
        (item_usage in)
        (item_usage out 'picture' value)))))
~~~
# EXPECTED
~~~
semantic.duplicate_name 'shoot'
semantic.unresolved_name 'ScalarValues::Boolean'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'shoot'
semantic.unresolved_name 'ScalarValues::Boolean'
~~~
# FORMAT
~~~sysml
package 'Conditional Succession Example-1' {
    part def Scene;
    part def Image {
        isWellFocused: ScalarValues::Boolean;
    }
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

        first focus
        if focus.image.isWellFocused then shoot;

        flow from focus.image to shoot.image;

        action shoot : Shoot {
            in item;
            out item picture = takePicture::picture;
        }
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "91276d3d2ae1496ce56eab933fa8e6bfab651ab2f8010eeee370434c3b1ea489") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-1"))) (kind "package") (name "Conditional Succession Example-1") (declared-name "Conditional Succession Example-1") (range (start (line 0) (character 0)) (end (line 0) (character 701))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-1::Focus"))) (kind "action def") (name "Focus") (declared-name "Focus") (range (start (line 7) (character 1)) (end (line 7) (character 58))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-1"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-1::Focus::image"))) (kind "in out parameter") (name "image") (declared-name "image") (range (start (line 7) (character 38)) (end (line 7) (character 56))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-1::Focus"))) (authored (relationships (typing (reference "Image") (range none)))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-1::Focus::scene"))) (kind "in out parameter") (name "scene") (declared-name "scene") (range (start (line 7) (character 20)) (end (line 7) (character 37))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-1::Focus"))) (authored (relationships (typing (reference "Scene") (range none)))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-1::Image"))) (kind "part def") (name "Image") (declared-name "Image") (range (start (line 2) (character 1)) (end (line 2) (character 60))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-1"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-1::Picture"))) (kind "part def") (name "Picture") (declared-name "Picture") (range (start (line 5) (character 1)) (end (line 5) (character 18))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-1"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-1::Scene"))) (kind "part def") (name "Scene") (declared-name "Scene") (range (start (line 1) (character 1)) (end (line 1) (character 16))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-1"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-1::Shoot"))) (kind "action def") (name "Shoot") (declared-name "Shoot") (range (start (line 8) (character 1)) (end (line 8) (character 61))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-1"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-1::Shoot::image"))) (kind "in out parameter") (name "image") (declared-name "image") (range (start (line 8) (character 20)) (end (line 8) (character 36))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-1::Shoot"))) (authored (relationships (typing (reference "Image") (range none)))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-1::Shoot::picture"))) (kind "in out parameter") (name "picture") (declared-name "picture") (range (start (line 8) (character 37)) (end (line 8) (character 59))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-1::Shoot"))) (authored (relationships (typing (reference "Picture") (range none)))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-1::TakePicture"))) (kind "action def") (name "TakePicture") (declared-name "TakePicture") (range (start (line 9) (character 1)) (end (line 9) (character 68))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-1"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-1::TakePicture::picture"))) (kind "in out parameter") (name "picture") (declared-name "picture") (range (start (line 9) (character 44)) (end (line 9) (character 66))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-1::TakePicture"))) (authored (relationships (typing (reference "Picture") (range none)))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-1::TakePicture::scene"))) (kind "in out parameter") (name "scene") (declared-name "scene") (range (start (line 9) (character 26)) (end (line 9) (character 43))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-1::TakePicture"))) (authored (relationships (typing (reference "Scene") (range none)))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture"))) (kind "action") (name "takePicture") (declared-name "takePicture") (range (start (line 11) (character 1)) (end (line 11) (character 360))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-1"))) (authored (membership (kind Feature)) (relationships (typing (reference "TakePicture") (range none)) (perform (reference "Conditional Succession Example-1::takePicture::focus") (range none)) (perform (reference "Conditional Succession Example-1::takePicture::shoot") (range none)))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::focus"))) (kind "action") (name "focus") (declared-name "focus") (range (start (line 15) (character 2)) (end (line 15) (character 87))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Focus") (range none)))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::focus::image"))) (kind "item") (name "image") (declared-name "image") (range (start (line 17) (character 3)) (end (line 17) (character 18))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::focus"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::focus::scene"))) (kind "item") (name "scene") (declared-name "scene") (range (start (line 16) (character 3)) (end (line 16) (character 38))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::focus"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::from"))) (kind "flow") (name "from") (declared-name "from") (range (start (line 23) (character 2)) (end (line 23) (character 39))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::picture"))) (kind "item") (name "picture") (declared-name "picture") (range (start (line 13) (character 2)) (end (line 13) (character 19))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::scene"))) (kind "item") (name "scene") (declared-name "scene") (range (start (line 12) (character 2)) (end (line 12) (character 16))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture"))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::shoot"))) (kind "action") (name "shoot") (declared-name "shoot") (range (start (line 25) (character 2)) (end (line 25) (character 85))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Shoot") (range none)))))
    (element (id (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::shoot::picture"))) (kind "item") (name "picture") (declared-name "picture") (range (start (line 27) (character 3)) (end (line 27) (character 43))) (parent (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::shoot"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-1::Focus::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-1::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-1::Focus::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "Scene") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-1::Scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-1::Shoot::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-1::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-1::Shoot::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-1::Picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-1::TakePicture::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-1::Picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-1::TakePicture::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "Scene") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-1::Scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture"))) (kind featureTyping) (ordinal 0)) (authored-target "TakePicture") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-1::TakePicture")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture"))) (kind flowSource) (ordinal 0)) (authored-target "focus::image") (range (start (line 23) (character 12)) (end (line 23) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::focus::image")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture"))) (kind flowTarget) (ordinal 0)) (authored-target "shoot::image") (range (start (line 23) (character 27)) (end (line 23) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture"))) (kind performSource) (ordinal 0)) (authored-target "Conditional Succession Example-1::takePicture::focus") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture"))) (kind performSource) (ordinal 1)) (authored-target "Conditional Succession Example-1::takePicture::shoot") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::focus"))) (kind featureTyping) (ordinal 0)) (authored-target "Focus") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-1::Focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::shoot"))) (kind featureTyping) (ordinal 0)) (authored-target "Shoot") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Conditional Succession Example-1::Shoot")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Conditional Succession Example-1::Focus::image"))) (target (node (document "d0") (qualified-name "Conditional Succession Example-1::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Conditional Succession Example-1::Focus::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Conditional Succession Example-1::Focus::scene"))) (target (node (document "d0") (qualified-name "Conditional Succession Example-1::Scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Conditional Succession Example-1::Focus::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Conditional Succession Example-1::Shoot::image"))) (target (node (document "d0") (qualified-name "Conditional Succession Example-1::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Conditional Succession Example-1::Shoot::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Conditional Succession Example-1::Shoot::picture"))) (target (node (document "d0") (qualified-name "Conditional Succession Example-1::Picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Conditional Succession Example-1::Shoot::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Conditional Succession Example-1::TakePicture::picture"))) (target (node (document "d0") (qualified-name "Conditional Succession Example-1::Picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Conditional Succession Example-1::TakePicture::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Conditional Succession Example-1::TakePicture::scene"))) (target (node (document "d0") (qualified-name "Conditional Succession Example-1::Scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Conditional Succession Example-1::TakePicture::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture"))) (target (node (document "d0") (qualified-name "Conditional Succession Example-1::TakePicture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture"))) (target (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture"))) (target (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture"))) (kind performSource) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::focus"))) (target (node (document "d0") (qualified-name "Conditional Succession Example-1::Focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::focus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::shoot"))) (target (node (document "d0") (qualified-name "Conditional Succession Example-1::Shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::shoot"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::focus::scene")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "Conditional Succession Example-1::takePicture::shoot::picture")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
