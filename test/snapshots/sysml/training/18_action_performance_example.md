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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPerform,KwAction,Ident,OpenSquare,Star,CloseSquare,KwOrdered,
KwReferences,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPerform,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPerform,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Action Performance Example''
    (import_decl private ''Action Decomposition'::*')
    (part_def 'Camera')
    (part_def 'AutoFocus')
    (part_def 'Imager')
    (part_usage 'camera' : 'Camera'
      (perform_action 'takePhoto' references 'takePicture' multiplicity ordered)
      (part_usage 'f' : 'AutoFocus'
        (perform_action :>> 'takePhoto.focus'))
      (part_usage 'i' : 'Imager'
        (perform_action :>> 'takePhoto.shoot')))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'takePicture'
semantic.unresolved_name 'takePhoto::focus'
semantic.unresolved_name 'takePhoto::shoot'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'takePicture'
semantic.unresolved_name 'takePhoto::focus'
semantic.unresolved_name 'takePhoto::shoot'
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
    (element (id (node (document "d0") (qualified-name "Action Performance Example"))) (kind "package") (name "Action Performance Example") (declared-name "Action Performance Example") (range (start (line 0) (character 0)) (end (line 0) (character 358))))
    (element (id (node (document "d0") (qualified-name "Action Performance Example::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 42))) (parent (node (document "d0") (qualified-name "Action Performance Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Action Decomposition::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 38))))))
    (element (id (node (document "d0") (qualified-name "Action Performance Example::AutoFocus"))) (kind "part def") (name "AutoFocus") (declared-name "AutoFocus") (range (start (line 4) (character 1)) (end (line 4) (character 20))) (parent (node (document "d0") (qualified-name "Action Performance Example"))))
    (element (id (node (document "d0") (qualified-name "Action Performance Example::Camera"))) (kind "part def") (name "Camera") (declared-name "Camera") (range (start (line 3) (character 1)) (end (line 3) (character 17))) (parent (node (document "d0") (qualified-name "Action Performance Example"))))
    (element (id (node (document "d0") (qualified-name "Action Performance Example::Imager"))) (kind "part def") (name "Imager") (declared-name "Imager") (range (start (line 5) (character 1)) (end (line 5) (character 17))) (parent (node (document "d0") (qualified-name "Action Performance Example"))))
    (element (id (node (document "d0") (qualified-name "Action Performance Example::camera"))) (kind "part") (name "camera") (declared-name "camera") (range (start (line 7) (character 1)) (end (line 7) (character 213))) (parent (node (document "d0") (qualified-name "Action Performance Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Camera") (range (start (line 7) (character 15)) (end (line 7) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "Action Performance Example::camera::f"))) (kind "part") (name "f") (declared-name "f") (range (start (line 12) (character 2)) (end (line 12) (character 57))) (parent (node (document "d0") (qualified-name "Action Performance Example::camera"))) (authored (membership (kind Feature)) (relationships (typing (reference "AutoFocus") (range (start (line 12) (character 11)) (end (line 12) (character 20)))) (perform (reference "Action Performance Example::camera::f::takePhoto::focus") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Performance Example::camera::f::takePhoto.focus"))) (kind "action") (name "takePhoto.focus") (declared-name "takePhoto.focus") (range (start (line 13) (character 3)) (end (line 13) (character 27))) (parent (node (document "d0") (qualified-name "Action Performance Example::camera::f"))))
    (element (id (node (document "d0") (qualified-name "Action Performance Example::camera::i"))) (kind "part") (name "i") (declared-name "i") (range (start (line 16) (character 2)) (end (line 16) (character 51))) (parent (node (document "d0") (qualified-name "Action Performance Example::camera"))) (authored (membership (kind Feature)) (relationships (typing (reference "Imager") (range (start (line 16) (character 11)) (end (line 16) (character 17)))) (perform (reference "Action Performance Example::camera::i::takePhoto::shoot") (range none)))))
    (element (id (node (document "d0") (qualified-name "Action Performance Example::camera::i::takePhoto.shoot"))) (kind "action") (name "takePhoto.shoot") (declared-name "takePhoto.shoot") (range (start (line 17) (character 3)) (end (line 17) (character 27))) (parent (node (document "d0") (qualified-name "Action Performance Example::camera::i"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Action Performance Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Action Decomposition::*") (range (start (line 1) (character 16)) (end (line 1) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Action Performance Example::camera"))) (kind featureTyping) (ordinal 0)) (authored-target "Camera") (range (start (line 7) (character 15)) (end (line 7) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Performance Example::Camera")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Performance Example::camera::f"))) (kind featureTyping) (ordinal 0)) (authored-target "AutoFocus") (range (start (line 12) (character 11)) (end (line 12) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Performance Example::AutoFocus")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Performance Example::camera::f"))) (kind performSource) (ordinal 0)) (authored-target "Action Performance Example::camera::f::takePhoto::focus") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Action Performance Example::camera::i"))) (kind featureTyping) (ordinal 0)) (authored-target "Imager") (range (start (line 16) (character 11)) (end (line 16) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Action Performance Example::Imager")))))
    (reference (id (source (node (document "d0") (qualified-name "Action Performance Example::camera::i"))) (kind performSource) (ordinal 0)) (authored-target "Action Performance Example::camera::i::takePhoto::shoot") (range none) (outcome (status unresolved)))
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
