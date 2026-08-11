# META
~~~ini
description=SysML Training 14 (Action Definitions): Action Succession Example-1
type=file
~~~
# SOURCE
~~~sysml
package 'Action Succession Example-1' {
	item def Scene;
	item def Image;
	item def Picture;
	
	action def Focus { in scene : Scene; out image : Image; }
	action def Shoot { in image: Image; out picture : Picture; }	
				
	action def TakePicture {
		in item scene : Scene;
		out item picture : Picture;
		
		bind focus.scene = scene;
		
		action focus: Focus { in scene; out image; }
		
		flow from focus.image to shoot.image;
		
		first focus then shoot;
		
		action shoot: Shoot { in image; out picture; }
		
		bind shoot.picture = picture;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "14_action_succession_example_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "connection_context_invalid")
        (source "semantic")
        (range (start 12 7) (end 12 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 24) (end 14 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 34) (end 14 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 24) (end 20 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 34) (end 20 46))
      )
      (diagnostic
        (severity warning)
        (code "connection_context_invalid")
        (source "semantic")
        (range (start 22 7) (end 22 20))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwItem,KwDef,Ident,Semicolon,
KwItem,KwDef,Ident,Semicolon,
KwItem,KwDef,Ident,Semicolon,
KwAction,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwOut,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwOut,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,Ident,OpenCurly,
KwIn,KwItem,Ident,Colon,Ident,Semicolon,
KwOut,KwItem,Ident,Colon,Ident,Semicolon,
KwBind,Ident,Dot,Ident,Eq,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,KwIn,Ident,Semicolon,KwOut,Ident,Semicolon,CloseCurly,
KwFlow,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwFirst,Ident,KwThen,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,KwIn,Ident,Semicolon,KwOut,Ident,Semicolon,CloseCurly,
KwBind,Ident,Dot,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Action Succession Example-1''
    (item_def 'Scene')
    (item_def 'Image')
    (item_def 'Picture')
    (action_def 'Focus'
      (default_ref_usage in 'scene' : 'Scene')
      (default_ref_usage out 'image' : 'Image'))
    (action_def 'Shoot'
      (default_ref_usage in 'image' : 'Image')
      (default_ref_usage out 'picture' : 'Picture'))
    (action_def 'TakePicture'
      (item_usage in 'scene' : 'Scene')
      (item_usage out 'picture' : 'Picture')
      (binding_as_usage
        (connector_end)
        (connector_end))
      (action_usage 'focus' : 'Focus'
        (default_ref_usage in 'scene')
        (default_ref_usage out 'image'))
      (flow_usage
        (connector_end)
        (connector_end))
      (succession_as_usage
        (connector_end)
        (connector_end))
      (action_usage 'shoot' : 'Shoot'
        (default_ref_usage in 'image')
        (default_ref_usage out 'picture'))
      (binding_as_usage
        (connector_end)
        (connector_end)))))
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# FORMAT
~~~sysml
package 'Action Succession Example-1' {
    item def Scene;
    item def Image;
    item def Picture;

    action def Focus { in scene : Scene; out image : Image; }
    action def Shoot { in image: Image; out picture : Picture; }

    action def TakePicture {
        in item scene : Scene;
        out item picture : Picture;

        bind focus.scene = scene;

        action focus: Focus { in scene; out image; }

        flow from focus.image to shoot.image;

        first focus then shoot;

        action shoot: Shoot { in image; out picture; }

        bind shoot.picture = picture;
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "e2384668817074c362790529b8dd6d56c72e480b663cf9c327204b50c26a2df5") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1"))) (kind "package") (name "Action Succession Example-1") (declared-name "Action Succession Example-1") (range (start (line 0) (character 0)) (end (line 0) (character 549))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::Focus"))) (kind "action def") (name "Focus") (declared-name "Focus") (range (start (line 5) (character 1)) (end (line 5) (character 58))) (parent (node (document "d0") (qualified-name "Action Succession Example-1"))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::Focus::image"))) (kind "in out parameter") (name "image") (declared-name "image") (range (start (line 5) (character 38)) (end (line 5) (character 56))) (parent (node (document "d0") (qualified-name "Action Succession Example-1::Focus"))) (authored (relationships (typing (reference "Image") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::Focus::scene"))) (kind "in out parameter") (name "scene") (declared-name "scene") (range (start (line 5) (character 20)) (end (line 5) (character 37))) (parent (node (document "d0") (qualified-name "Action Succession Example-1::Focus"))) (authored (relationships (typing (reference "Scene") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::Image"))) (kind "item def") (name "Image") (declared-name "Image") (range (start (line 2) (character 1)) (end (line 2) (character 16))) (parent (node (document "d0") (qualified-name "Action Succession Example-1"))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::Picture"))) (kind "item def") (name "Picture") (declared-name "Picture") (range (start (line 3) (character 1)) (end (line 3) (character 18))) (parent (node (document "d0") (qualified-name "Action Succession Example-1"))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::Scene"))) (kind "item def") (name "Scene") (declared-name "Scene") (range (start (line 1) (character 1)) (end (line 1) (character 16))) (parent (node (document "d0") (qualified-name "Action Succession Example-1"))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::Shoot"))) (kind "action def") (name "Shoot") (declared-name "Shoot") (range (start (line 6) (character 1)) (end (line 6) (character 61))) (parent (node (document "d0") (qualified-name "Action Succession Example-1"))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::Shoot::image"))) (kind "in out parameter") (name "image") (declared-name "image") (range (start (line 6) (character 20)) (end (line 6) (character 36))) (parent (node (document "d0") (qualified-name "Action Succession Example-1::Shoot"))) (authored (relationships (typing (reference "Image") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::Shoot::picture"))) (kind "in out parameter") (name "picture") (declared-name "picture") (range (start (line 6) (character 37)) (end (line 6) (character 59))) (parent (node (document "d0") (qualified-name "Action Succession Example-1::Shoot"))) (authored (relationships (typing (reference "Picture") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind "action def") (name "TakePicture") (declared-name "TakePicture") (range (start (line 8) (character 1)) (end (line 8) (character 323))) (parent (node (document "d0") (qualified-name "Action Succession Example-1"))) (authored (membership (kind Owning)) (relationships (perform (reference "Action Succession Example-1::TakePicture::focus") (range none)) (perform (reference "Action Succession Example-1::TakePicture::shoot") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus"))) (kind "action") (name "focus") (declared-name "focus") (range (start (line 14) (character 2)) (end (line 14) (character 46))) (parent (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Focus") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus::image"))) (kind "in out parameter") (name "image") (declared-name "image") (range (start (line 14) (character 34)) (end (line 14) (character 44))) (parent (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus::scene"))) (kind "in out parameter") (name "scene") (declared-name "scene") (range (start (line 14) (character 24)) (end (line 14) (character 33))) (parent (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::from"))) (kind "flow") (name "from") (declared-name "from") (range (start (line 16) (character 2)) (end (line 16) (character 39))) (parent (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::picture"))) (kind "item") (name "picture") (declared-name "picture") (range (start (line 10) (character 2)) (end (line 10) (character 29))) (parent (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Picture") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::scene"))) (kind "item") (name "scene") (declared-name "scene") (range (start (line 9) (character 2)) (end (line 9) (character 24))) (parent (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Scene") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot"))) (kind "action") (name "shoot") (declared-name "shoot") (range (start (line 20) (character 2)) (end (line 20) (character 48))) (parent (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Shoot") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot::image"))) (kind "in out parameter") (name "image") (declared-name "image") (range (start (line 20) (character 24)) (end (line 20) (character 33))) (parent (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot::picture"))) (kind "in out parameter") (name "picture") (declared-name "picture") (range (start (line 20) (character 34)) (end (line 20) (character 46))) (parent (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot"))) (authored (relationships (typing (reference "") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::Focus::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::Focus::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "Scene") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::Scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::Shoot::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::Shoot::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::Picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind bindSource) (ordinal 0)) (authored-target "focus::scene") (range (start (line 12) (character 7)) (end (line 12) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus::scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind bindSource) (ordinal 3)) (authored-target "shoot::picture") (range (start (line 22) (character 7)) (end (line 22) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot::picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind bindTarget) (ordinal 0)) (authored-target "scene") (range (start (line 12) (character 21)) (end (line 12) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind bindTarget) (ordinal 3)) (authored-target "picture") (range (start (line 22) (character 23)) (end (line 22) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind flowSource) (ordinal 1)) (authored-target "focus::image") (range (start (line 16) (character 12)) (end (line 16) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus::image")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind flowSource) (ordinal 2)) (authored-target "focus") (range (start (line 18) (character 8)) (end (line 18) (character 13))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind flowTarget) (ordinal 1)) (authored-target "shoot::image") (range (start (line 16) (character 27)) (end (line 16) (character 38))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot::image")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind flowTarget) (ordinal 2)) (authored-target "shoot") (range (start (line 18) (character 19)) (end (line 18) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind performSource) (ordinal 0)) (authored-target "Action Succession Example-1::TakePicture::focus") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind performSource) (ordinal 1)) (authored-target "Action Succession Example-1::TakePicture::shoot") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus"))) (kind featureTyping) (ordinal 0)) (authored-target "Focus") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::Focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus::image"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::Picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "Scene") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::Scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot"))) (kind featureTyping) (ordinal 0)) (authored-target "Shoot") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Succession Example-1::Shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot::image"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Succession Example-1::Focus::image"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::Focus::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Succession Example-1::Focus::scene"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::Scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::Focus::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Succession Example-1::Shoot::image"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::Shoot::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Succession Example-1::Shoot::picture"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::Picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::Shoot::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind performSource) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::Focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind flowSource) (ordinal 2)) (expression (kind flow) (source "focus") (target "shoot") (source-range (start (line 18) (character 8)) (end (line 18) (character 13))) (target-range (start (line 18) (character 19)) (end (line 18) (character 24)))))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus::image"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot::image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind flowSource) (ordinal 1)) (expression (kind flow) (source "focus::image") (target "shoot::image") (source-range (start (line 16) (character 12)) (end (line 16) (character 23))) (target-range (start (line 16) (character 27)) (end (line 16) (character 38)))))
    (relationship (kind bind) (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::focus::scene"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind bindSource) (ordinal 0)) (expression (kind bind) (source "focus::scene") (target "scene") (source-range (start (line 12) (character 7)) (end (line 12) (character 18))) (target-range (start (line 12) (character 21)) (end (line 12) (character 26)))))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::picture"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::Picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::scene"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::Scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::Shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind bind) (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::shoot::picture"))) (target (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture::picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Succession Example-1::TakePicture"))) (kind bindSource) (ordinal 3)) (expression (kind bind) (source "shoot::picture") (target "picture") (source-range (start (line 22) (character 7)) (end (line 22) (character 20))) (target-range (start (line 22) (character 23)) (end (line 22) (character 30)))))
  )
  (evaluation
  )
)
~~~
