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
  (document "17_merge_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 2) (end 11 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 2) (end 15 54))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwAction,KwDef,Ident,OpenCurly,KwIn,KwItem,Ident,Colon,Ident,Semicolon,KwOut,KwItem,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,Ident,OpenCurly,KwIn,KwItem,Ident,Colon,Ident,Semicolon,KwOut,KwItem,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,Ident,OpenCurly,KwIn,KwItem,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,
KwFirst,Ident,Semicolon,
KwThen,KwMerge,Ident,Semicolon,
KwThen,KwAction,Ident,OpenCurly,
KwOut,KwItem,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwFlow,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwThen,KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,KwItem,Ident,Semicolon,
KwOut,KwItem,Ident,Semicolon,
CloseCurly,
KwFlow,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwThen,KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,KwItem,Ident,Semicolon,
KwOut,KwItem,Ident,Semicolon,
CloseCurly,
KwFlow,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwThen,KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,KwItem,Ident,Semicolon,
CloseCurly,
KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Merge Example''
    (part_def 'Scene')
    (part_def 'Image')
    (part_def 'Picture')
    (action_def 'Focus'
      (item_usage in 'scene' : 'Scene')
      (item_usage out 'image' : 'Image'))
    (action_def 'Shoot'
      (item_usage in 'image' : 'Image')
      (item_usage out 'picture' : 'Picture'))
    (action_def 'Display'
      (item_usage in 'picture' : 'Picture'))
    (action_def 'TakePicture')
    (action_usage 'takePicture' : 'TakePicture'
      (initial_node start)
      (source_succession
        (sysml_decl 'continue'))
      (source_succession
        (action_usage 'trigger'
          (item_usage out 'scene' : 'Scene')))
      (flow_usage
        (connector_end)
        (connector_end))
      (source_succession
        (action_usage 'focus' : 'Focus'
          (item_usage in 'scene')
          (item_usage out 'image')))
      (flow_usage
        (connector_end)
        (connector_end))
      (source_succession
        (action_usage 'shoot' : 'Shoot'
          (item_usage in 'image')
          (item_usage out 'picture')))
      (flow_usage
        (connector_end)
        (connector_end))
      (source_succession
        (action_usage 'display' : 'Display'
          (item_usage in 'picture')))
      (source_succession
        (default_ref_usage 'continue')))))
~~~
# EXPECTED
~~~
semantic.duplicate_name 'continue'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'continue'
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "8e4f4d5713d47e75f2d2bf3c314b2e7800e583e7e22703fc58b0a013d1a5a10f") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Merge Example"))) (kind "package") (name "Merge Example") (declared-name "Merge Example") (range (start (line 0) (character 0)) (end (line 0) (character 811))))
    (element (id (node (document "d0") (qualified-name "Merge Example::Display"))) (kind "action def") (name "Display") (declared-name "Display") (range (start (line 7) (character 1)) (end (line 7) (character 50))) (parent (node (document "d0") (qualified-name "Merge Example"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::Display::picture"))) (kind "item") (name "picture") (declared-name "picture") (range (start (line 7) (character 22)) (end (line 7) (character 48))) (parent (node (document "d0") (qualified-name "Merge Example::Display"))) (authored (membership (kind Feature)) (relationships (typing (reference "Picture") (range none)))))
    (element (id (node (document "d0") (qualified-name "Merge Example::Focus"))) (kind "action def") (name "Focus") (declared-name "Focus") (range (start (line 5) (character 1)) (end (line 5) (character 68))) (parent (node (document "d0") (qualified-name "Merge Example"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::Focus::image"))) (kind "item") (name "image") (declared-name "image") (range (start (line 5) (character 43)) (end (line 5) (character 66))) (parent (node (document "d0") (qualified-name "Merge Example::Focus"))) (authored (membership (kind Feature)) (relationships (typing (reference "Image") (range none)))))
    (element (id (node (document "d0") (qualified-name "Merge Example::Focus::scene"))) (kind "item") (name "scene") (declared-name "scene") (range (start (line 5) (character 20)) (end (line 5) (character 42))) (parent (node (document "d0") (qualified-name "Merge Example::Focus"))) (authored (membership (kind Feature)) (relationships (typing (reference "Scene") (range none)))))
    (element (id (node (document "d0") (qualified-name "Merge Example::Image"))) (kind "part def") (name "Image") (declared-name "Image") (range (start (line 2) (character 1)) (end (line 2) (character 16))) (parent (node (document "d0") (qualified-name "Merge Example"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::Picture"))) (kind "part def") (name "Picture") (declared-name "Picture") (range (start (line 3) (character 1)) (end (line 3) (character 18))) (parent (node (document "d0") (qualified-name "Merge Example"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::Scene"))) (kind "part def") (name "Scene") (declared-name "Scene") (range (start (line 1) (character 1)) (end (line 1) (character 16))) (parent (node (document "d0") (qualified-name "Merge Example"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::Shoot"))) (kind "action def") (name "Shoot") (declared-name "Shoot") (range (start (line 6) (character 1)) (end (line 6) (character 72))) (parent (node (document "d0") (qualified-name "Merge Example"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::Shoot::image"))) (kind "item") (name "image") (declared-name "image") (range (start (line 6) (character 20)) (end (line 6) (character 42))) (parent (node (document "d0") (qualified-name "Merge Example::Shoot"))) (authored (membership (kind Feature)) (relationships (typing (reference "Image") (range none)))))
    (element (id (node (document "d0") (qualified-name "Merge Example::Shoot::picture"))) (kind "item") (name "picture") (declared-name "picture") (range (start (line 6) (character 43)) (end (line 6) (character 70))) (parent (node (document "d0") (qualified-name "Merge Example::Shoot"))) (authored (membership (kind Feature)) (relationships (typing (reference "Picture") (range none)))))
    (element (id (node (document "d0") (qualified-name "Merge Example::TakePicture"))) (kind "action def") (name "TakePicture") (declared-name "TakePicture") (range (start (line 8) (character 1)) (end (line 8) (character 24))) (parent (node (document "d0") (qualified-name "Merge Example"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind "action") (name "takePicture") (declared-name "takePicture") (range (start (line 10) (character 1)) (end (line 10) (character 508))) (parent (node (document "d0") (qualified-name "Merge Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "TakePicture") (range none)) (perform (reference "Merge Example::takePicture::trigger") (range none)) (perform (reference "Merge Example::takePicture::focus") (range none)) (perform (reference "Merge Example::takePicture::shoot") (range none)) (perform (reference "Merge Example::takePicture::display") (range none)))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::_initial"))) (kind "initial") (name "_initial") (range (start (line 11) (character 2)) (end (line 11) (character 14))) (parent (node (document "d0") (qualified-name "Merge Example::takePicture"))) (authored (relationships (flow (reference "Merge Example::takePicture::start") (range none)))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::continue"))) (kind "merge") (name "merge") (declared-name "merge") (range (start (line 13) (character 2)) (end (line 13) (character 22))) (parent (node (document "d0") (qualified-name "Merge Example::takePicture"))) (authored (relationships (flow (reference "Merge Example::takePicture::trigger") (range none)))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::display"))) (kind "action") (name "display") (declared-name "display") (range (start (line 35) (character 2)) (end (line 35) (character 57))) (parent (node (document "d0") (qualified-name "Merge Example::takePicture"))) (authored (relationships (typing (reference "Display") (range none)) (flow (reference "Merge Example::takePicture::continue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::display::picture"))) (kind "item") (name "picture") (declared-name "picture") (range (start (line 36) (character 3)) (end (line 36) (character 19))) (parent (node (document "d0") (qualified-name "Merge Example::takePicture::display"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::focus"))) (kind "action") (name "focus") (declared-name "focus") (range (start (line 21) (character 2)) (end (line 21) (character 70))) (parent (node (document "d0") (qualified-name "Merge Example::takePicture"))) (authored (relationships (typing (reference "Focus") (range none)) (flow (reference "Merge Example::takePicture::shoot") (range none)))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::focus::image"))) (kind "item") (name "image") (declared-name "image") (range (start (line 23) (character 3)) (end (line 23) (character 18))) (parent (node (document "d0") (qualified-name "Merge Example::takePicture::focus"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::focus::scene"))) (kind "item") (name "scene") (declared-name "scene") (range (start (line 22) (character 3)) (end (line 22) (character 17))) (parent (node (document "d0") (qualified-name "Merge Example::takePicture::focus"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::from"))) (kind "flow") (name "from") (declared-name "from") (range (start (line 19) (character 2)) (end (line 19) (character 41))) (parent (node (document "d0") (qualified-name "Merge Example::takePicture"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::from#flow"))) (kind "flow") (name "from") (declared-name "from") (range (start (line 26) (character 2)) (end (line 26) (character 39))) (parent (node (document "d0") (qualified-name "Merge Example::takePicture"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::from#flow2"))) (kind "flow") (name "from") (declared-name "from") (range (start (line 33) (character 2)) (end (line 33) (character 45))) (parent (node (document "d0") (qualified-name "Merge Example::takePicture"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::shoot"))) (kind "action") (name "shoot") (declared-name "shoot") (range (start (line 28) (character 2)) (end (line 28) (character 73))) (parent (node (document "d0") (qualified-name "Merge Example::takePicture"))) (authored (relationships (typing (reference "Shoot") (range none)) (flow (reference "Merge Example::takePicture::display") (range none)))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::shoot::image"))) (kind "item") (name "image") (declared-name "image") (range (start (line 29) (character 3)) (end (line 29) (character 18))) (parent (node (document "d0") (qualified-name "Merge Example::takePicture::shoot"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::shoot::picture"))) (kind "item") (name "picture") (declared-name "picture") (range (start (line 30) (character 3)) (end (line 30) (character 20))) (parent (node (document "d0") (qualified-name "Merge Example::takePicture::shoot"))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::trigger"))) (kind "action") (name "trigger") (declared-name "trigger") (range (start (line 15) (character 2)) (end (line 15) (character 54))) (parent (node (document "d0") (qualified-name "Merge Example::takePicture"))) (authored (relationships (typing (reference "") (range none)) (flow (reference "Merge Example::takePicture::focus") (range none)))))
    (element (id (node (document "d0") (qualified-name "Merge Example::takePicture::trigger::scene"))) (kind "item") (name "scene") (declared-name "scene") (range (start (line 16) (character 3)) (end (line 16) (character 26))) (parent (node (document "d0") (qualified-name "Merge Example::takePicture::trigger"))) (authored (membership (kind Feature)) (relationships (typing (reference "Scene") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::Display::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::Picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::Focus::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::Focus::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "Scene") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::Scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::Shoot::image"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::Shoot::picture"))) (kind featureTyping) (ordinal 0)) (authored-target "Picture") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::Picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind featureTyping) (ordinal 0)) (authored-target "TakePicture") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::TakePicture")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind flowSource) (ordinal 0)) (authored-target "trigger::scene") (range (start (line 19) (character 12)) (end (line 19) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::trigger::scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind flowSource) (ordinal 1)) (authored-target "focus::image") (range (start (line 26) (character 12)) (end (line 26) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::focus::image")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind flowSource) (ordinal 2)) (authored-target "shoot::picture") (range (start (line 33) (character 12)) (end (line 33) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::shoot::picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind flowTarget) (ordinal 0)) (authored-target "focus::scene") (range (start (line 19) (character 29)) (end (line 19) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::focus::scene")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind flowTarget) (ordinal 1)) (authored-target "shoot::image") (range (start (line 26) (character 27)) (end (line 26) (character 38))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::shoot::image")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind flowTarget) (ordinal 2)) (authored-target "display::picture") (range (start (line 33) (character 29)) (end (line 33) (character 44))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::display::picture")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind performSource) (ordinal 0)) (authored-target "Merge Example::takePicture::trigger") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::trigger")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind performSource) (ordinal 1)) (authored-target "Merge Example::takePicture::focus") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind performSource) (ordinal 2)) (authored-target "Merge Example::takePicture::shoot") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind performSource) (ordinal 3)) (authored-target "Merge Example::takePicture::display") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::display")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture::_initial"))) (kind flowSource) (ordinal 0)) (authored-target "Merge Example::takePicture::start") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture::continue"))) (kind flowSource) (ordinal 0)) (authored-target "Merge Example::takePicture::trigger") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::trigger")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture::display"))) (kind featureTyping) (ordinal 0)) (authored-target "Display") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::Display")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture::display"))) (kind flowSource) (ordinal 0)) (authored-target "Merge Example::takePicture::continue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::continue")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture::focus"))) (kind featureTyping) (ordinal 0)) (authored-target "Focus") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::Focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture::focus"))) (kind flowSource) (ordinal 0)) (authored-target "Merge Example::takePicture::shoot") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture::shoot"))) (kind featureTyping) (ordinal 0)) (authored-target "Shoot") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::Shoot")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture::shoot"))) (kind flowSource) (ordinal 0)) (authored-target "Merge Example::takePicture::display") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::display")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture::trigger"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture::trigger"))) (kind flowSource) (ordinal 0)) (authored-target "Merge Example::takePicture::focus") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::takePicture::focus")))))
    (reference (id (source (node (document "d0") (qualified-name "Merge Example::takePicture::trigger::scene"))) (kind featureTyping) (ordinal 0)) (authored-target "Scene") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Merge Example::Scene")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Merge Example::Display::picture"))) (target (node (document "d0") (qualified-name "Merge Example::Picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::Display::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Merge Example::Focus::image"))) (target (node (document "d0") (qualified-name "Merge Example::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::Focus::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Merge Example::Focus::scene"))) (target (node (document "d0") (qualified-name "Merge Example::Scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::Focus::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Merge Example::Shoot::image"))) (target (node (document "d0") (qualified-name "Merge Example::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::Shoot::image"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Merge Example::Shoot::picture"))) (target (node (document "d0") (qualified-name "Merge Example::Picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::Shoot::picture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (target (node (document "d0") (qualified-name "Merge Example::TakePicture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (target (node (document "d0") (qualified-name "Merge Example::takePicture::display"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind performSource) (ordinal 3)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (target (node (document "d0") (qualified-name "Merge Example::takePicture::focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind performSource) (ordinal 1)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (target (node (document "d0") (qualified-name "Merge Example::takePicture::shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind performSource) (ordinal 2)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (target (node (document "d0") (qualified-name "Merge Example::takePicture::trigger"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind performSource) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Merge Example::takePicture::continue"))) (target (node (document "d0") (qualified-name "Merge Example::takePicture::trigger"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture::continue"))) (kind flowSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Merge Example::takePicture::display"))) (target (node (document "d0") (qualified-name "Merge Example::Display"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture::display"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Merge Example::takePicture::display"))) (target (node (document "d0") (qualified-name "Merge Example::takePicture::continue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture::display"))) (kind flowSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Merge Example::takePicture::focus"))) (target (node (document "d0") (qualified-name "Merge Example::Focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture::focus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Merge Example::takePicture::focus"))) (target (node (document "d0") (qualified-name "Merge Example::takePicture::shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture::focus"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Merge Example::takePicture::focus::image"))) (target (node (document "d0") (qualified-name "Merge Example::takePicture::shoot::image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind flowSource) (ordinal 1)) (expression (kind flow) (source "focus::image") (target "shoot::image") (source-range (start (line 26) (character 12)) (end (line 26) (character 23))) (target-range (start (line 26) (character 27)) (end (line 26) (character 38)))))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Merge Example::takePicture::shoot"))) (target (node (document "d0") (qualified-name "Merge Example::Shoot"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture::shoot"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Merge Example::takePicture::shoot"))) (target (node (document "d0") (qualified-name "Merge Example::takePicture::display"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture::shoot"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Merge Example::takePicture::shoot::picture"))) (target (node (document "d0") (qualified-name "Merge Example::takePicture::display::picture"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind flowSource) (ordinal 2)) (expression (kind flow) (source "shoot::picture") (target "display::picture") (source-range (start (line 33) (character 12)) (end (line 33) (character 25))) (target-range (start (line 33) (character 29)) (end (line 33) (character 44)))))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Merge Example::takePicture::trigger"))) (target (node (document "d0") (qualified-name "Merge Example::takePicture::focus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture::trigger"))) (kind flowSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Merge Example::takePicture::trigger::scene"))) (target (node (document "d0") (qualified-name "Merge Example::Scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture::trigger::scene"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Merge Example::takePicture::trigger::scene"))) (target (node (document "d0") (qualified-name "Merge Example::takePicture::focus::scene"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Merge Example::takePicture"))) (kind flowSource) (ordinal 0)) (expression (kind flow) (source "trigger::scene") (target "focus::scene") (source-range (start (line 19) (character 12)) (end (line 19) (character 25))) (target-range (start (line 19) (character 29)) (end (line 19) (character 40)))))
  )
  (evaluation
  )
)
~~~
