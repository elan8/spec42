# META
~~~ini
description=SysML Training 14 (Action Definitions): Action Definition Example
type=file
~~~
# SOURCE
~~~sysml
package 'Action Definition Example' {
	item def Scene;
	item def Image;
	item def Picture;
	
	action def Focus { in scene : Scene; out image : Image; }
	action def Shoot { in image: Image; out picture : Picture; }	
		
	action def TakePicture { in scene : Scene; out picture : Picture;
		bind focus.scene = scene;
		
		action focus: Focus { in scene; out image; }
		
		flow from focus.image to shoot.image;
		
		action shoot: Shoot { in image; out picture; }
		
		bind shoot.picture = picture;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "14_action_definition_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "connection_context_invalid")
        (source "semantic")
        (range (start 9 7) (end 9 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 24) (end 11 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 34) (end 11 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 24) (end 15 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 34) (end 15 46))
      )
      (diagnostic
        (severity warning)
        (code "connection_context_invalid")
        (source "semantic")
        (range (start 17 7) (end 17 20))
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
KwAction,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwOut,Ident,Colon,Ident,Semicolon,
KwBind,Ident,Dot,Ident,Eq,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,KwIn,Ident,Semicolon,KwOut,Ident,Semicolon,CloseCurly,
KwFlow,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,KwIn,Ident,Semicolon,KwOut,Ident,Semicolon,CloseCurly,
KwBind,Ident,Dot,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Action Definition Example''
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
      (default_ref_usage in 'scene' : 'Scene')
      (default_ref_usage out 'picture' : 'Picture')
      (binding_as_usage
        (connector_end)
        (connector_end))
      (action_usage 'focus' : 'Focus'
        (default_ref_usage in 'scene')
        (default_ref_usage out 'image'))
      (flow_usage
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
package 'Action Definition Example' {
    item def Scene;
    item def Image;
    item def Picture;

    action def Focus { in scene : Scene; out image : Image; }
    action def Shoot { in image: Image; out picture : Picture; }

    action def TakePicture { in scene : Scene; out picture : Picture;
        bind focus.scene = scene;

        action focus: Focus { in scene; out image; }

        flow from focus.image to shoot.image;

        action shoot: Shoot { in image; out picture; }

        bind shoot.picture = picture;
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "48407516b525129bd2b6149c49fa247c5441a2727b5eb586ceed14eff8398bbc") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Action Definition Example"))) (kind "package") (name "Action Definition Example") (declared-name "Action Definition Example") (range (start (line 0) (character 0)) (end (line 0) (character 499))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::Focus"))) (kind "action def") (name "Focus") (declared-name "Focus") (range (start (line 5) (character 1)) (end (line 5) (character 58))) (parent (node (document "d0") (qualified-name "Action Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::Focus::image"))) (kind "in out parameter") (name "image") (declared-name "image") (range (start (line 5) (character 38)) (end (line 5) (character 56))) (parent (node (document "d0") (qualified-name "Action Definition Example::Focus"))) (authored (relationships (typing (reference "Image") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::Focus::scene"))) (kind "in out parameter") (name "scene") (declared-name "scene") (range (start (line 5) (character 20)) (end (line 5) (character 37))) (parent (node (document "d0") (qualified-name "Action Definition Example::Focus"))) (authored (relationships (typing (reference "Scene") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::Image"))) (kind "item def") (name "Image") (declared-name "Image") (range (start (line 2) (character 1)) (end (line 2) (character 16))) (parent (node (document "d0") (qualified-name "Action Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::Picture"))) (kind "item def") (name "Picture") (declared-name "Picture") (range (start (line 3) (character 1)) (end (line 3) (character 18))) (parent (node (document "d0") (qualified-name "Action Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::Scene"))) (kind "item def") (name "Scene") (declared-name "Scene") (range (start (line 1) (character 1)) (end (line 1) (character 16))) (parent (node (document "d0") (qualified-name "Action Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::Shoot"))) (kind "action def") (name "Shoot") (declared-name "Shoot") (range (start (line 6) (character 1)) (end (line 6) (character 61))) (parent (node (document "d0") (qualified-name "Action Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::Shoot::image"))) (kind "in out parameter") (name "image") (declared-name "image") (range (start (line 6) (character 20)) (end (line 6) (character 36))) (parent (node (document "d0") (qualified-name "Action Definition Example::Shoot"))) (authored (relationships (typing (reference "Image") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::Shoot::picture"))) (kind "in out parameter") (name "picture") (declared-name "picture") (range (start (line 6) (character 37)) (end (line 6) (character 59))) (parent (node (document "d0") (qualified-name "Action Definition Example::Shoot"))) (authored (relationships (typing (reference "Picture") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind "action def") (name "TakePicture") (declared-name "TakePicture") (range (start (line 8) (character 1)) (end (line 8) (character 277))) (parent (node (document "d0") (qualified-name "Action Definition Example"))) (authored (membership (kind Owning)) (relationships (perform (reference "Action Definition Example::TakePicture::focus") (range none)) (perform (reference "Action Definition Example::TakePicture::shoot") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus"))) (kind "action") (name "focus") (declared-name "focus") (range (start (line 11) (character 2)) (end (line 11) (character 46))) (parent (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Focus") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus::image"))) (kind "in out parameter") (name "image") (declared-name "image") (range (start (line 11) (character 34)) (end (line 11) (character 44))) (parent (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus::scene"))) (kind "in out parameter") (name "scene") (declared-name "scene") (range (start (line 11) (character 24)) (end (line 11) (character 33))) (parent (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture::from"))) (kind "flow") (name "from") (declared-name "from") (range (start (line 13) (character 2)) (end (line 13) (character 39))) (parent (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture::picture"))) (kind "in out parameter") (name "picture") (declared-name "picture") (range (start (line 8) (character 44)) (end (line 8) (character 66))) (parent (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (authored (relationships (typing (reference "Picture") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture::scene"))) (kind "in out parameter") (name "scene") (declared-name "scene") (range (start (line 8) (character 26)) (end (line 8) (character 43))) (parent (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (authored (relationships (typing (reference "Scene") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot"))) (kind "action") (name "shoot") (declared-name "shoot") (range (start (line 15) (character 2)) (end (line 15) (character 48))) (parent (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (authored (membership (kind Feature)) (relationships (typing (reference "Shoot") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot::image"))) (kind "in out parameter") (name "image") (declared-name "image") (range (start (line 15) (character 24)) (end (line 15) (character 33))) (parent (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot::picture"))) (kind "in out parameter") (name "picture") (declared-name "picture") (range (start (line 15) (character 34)) (end (line 15) (character 46))) (parent (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot"))) (authored (relationships (typing (reference "") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::Focus::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::Focus::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "Scene") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::Scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::Shoot::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::Shoot::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::Picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind bindSource) (ordinal 0)) (authored-target "focus::scene") (range (start (line 9) (character 7)) (end (line 9) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus::scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind bindSource) (ordinal 2)) (authored-target "shoot::picture") (range (start (line 17) (character 7)) (end (line 17) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot::picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind bindTarget) (ordinal 0)) (authored-target "scene") (range (start (line 9) (character 21)) (end (line 9) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::TakePicture::scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind bindTarget) (ordinal 2)) (authored-target "picture") (range (start (line 17) (character 23)) (end (line 17) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::TakePicture::picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind flowSource) (ordinal 1)) (authored-target "focus::image") (range (start (line 13) (character 12)) (end (line 13) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus::image")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind flowTarget) (ordinal 1)) (authored-target "shoot::image") (range (start (line 13) (character 27)) (end (line 13) (character 38))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot::image")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind performSource) (ordinal 0)) (authored-target "Action Definition Example::TakePicture::focus") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind performSource) (ordinal 1)) (authored-target "Action Definition Example::TakePicture::shoot") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus"))) (kind featureTyping) (ordinal 0)) (authored-target "Focus") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::Focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus::image"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::Picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "Scene") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::Scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot"))) (kind featureTyping) (ordinal 0)) (authored-target "Shoot") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Definition Example::Shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot::image"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Definition Example::Focus::image"))) (target (node (document "d0") (qualified-name "Action Definition Example::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Definition Example::Focus::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Definition Example::Focus::scene"))) (target (node (document "d0") (qualified-name "Action Definition Example::Scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Definition Example::Focus::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Definition Example::Shoot::image"))) (target (node (document "d0") (qualified-name "Action Definition Example::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Definition Example::Shoot::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Definition Example::Shoot::picture"))) (target (node (document "d0") (qualified-name "Action Definition Example::Picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Definition Example::Shoot::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (target (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (target (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind performSource) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus"))) (target (node (document "d0") (qualified-name "Action Definition Example::Focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus::image"))) (target (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot::image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind flowSource) (ordinal 1)) (expression (kind flow) (source "focus::image") (target "shoot::image") (source-range (start (line 13) (character 12)) (end (line 13) (character 23))) (target-range (start (line 13) (character 27)) (end (line 13) (character 38)))))
    (relationship (kind bind) (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::focus::scene"))) (target (node (document "d0") (qualified-name "Action Definition Example::TakePicture::scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind bindSource) (ordinal 0)) (expression (kind bind) (source "focus::scene") (target "scene") (source-range (start (line 9) (character 7)) (end (line 9) (character 18))) (target-range (start (line 9) (character 21)) (end (line 9) (character 26)))))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::picture"))) (target (node (document "d0") (qualified-name "Action Definition Example::Picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::scene"))) (target (node (document "d0") (qualified-name "Action Definition Example::Scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot"))) (target (node (document "d0") (qualified-name "Action Definition Example::Shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind bind) (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture::shoot::picture"))) (target (node (document "d0") (qualified-name "Action Definition Example::TakePicture::picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Action Definition Example::TakePicture"))) (kind bindSource) (ordinal 2)) (expression (kind bind) (source "shoot::picture") (target "picture") (source-range (start (line 17) (character 7)) (end (line 17) (character 20))) (target-range (start (line 17) (character 23)) (end (line 17) (character 30)))))
  )
  (evaluation
  )
)
~~~
