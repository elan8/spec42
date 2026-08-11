# META
~~~ini
description=SysML Training 01 (Packages): Documentation Example
type=file
~~~
# SOURCE
~~~sysml
package 'Documentation Example' {
	doc /* This is documentation of the owning 
	     * package.
	     */
	
	part def Automobile {
		doc Document1 /* This documentation of Automobile. */
	}
	
	alias Car for Automobile {
		doc /* This is documentation of the alias. */
	}
	alias Torque for ISQ::TorqueValue;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "01_documentation_example.md"
    (diagnostics
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwDoc,RegularComment,
KwPart,KwDef,Ident,OpenCurly,
KwDoc,Ident,RegularComment,
CloseCurly,
KwAlias,Ident,KwFor,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwAlias,Ident,KwFor,Ident,ColonColon,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Documentation Example''
    (documentation)
    (part_def 'Automobile'
      (documentation 'Document1'))
    (alias_member 'Car' for 'Automobile'
      (documentation))
    (alias_member 'Torque' for 'ISQ::TorqueValue')))
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
package 'Documentation Example' {
    doc /* This is documentation of the owning 
	     * package.
	     */

    part def Automobile {
        doc Document1 /* This documentation of Automobile. */
    }

    alias Car for Automobile {
        doc /* This is documentation of the alias. */
    }
    alias Torque for ISQ::TorqueValue;
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "84f77ef0bcc3a0b727346ebc686087fb99127cad4de444e33ec32cad7b127571") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Documentation Example"))) (kind "package") (name "Documentation Example") (declared-name "Documentation Example") (range (start (line 0) (character 0)) (end (line 0) (character 307))))
    (element (id (node (document "d0") (qualified-name "Documentation Example::Automobile"))) (kind "part def") (name "Automobile") (declared-name "Automobile") (range (start (line 5) (character 1)) (end (line 5) (character 81))) (parent (node (document "d0") (qualified-name "Documentation Example"))))
    (element (id (node (document "d0") (qualified-name "Documentation Example::Automobile::_documentation"))) (kind "documentation") (name "") (range (start (line 5) (character 1)) (end (line 5) (character 81))) (parent (node (document "d0") (qualified-name "Documentation Example::Automobile"))))
    (element (id (node (document "d0") (qualified-name "Documentation Example::Car"))) (kind "alias") (name "Car") (declared-name "Car") (range (start (line 9) (character 1)) (end (line 9) (character 78))) (parent (node (document "d0") (qualified-name "Documentation Example"))))
    (element (id (node (document "d0") (qualified-name "Documentation Example::Torque"))) (kind "alias") (name "Torque") (declared-name "Torque") (range (start (line 12) (character 1)) (end (line 12) (character 35))) (parent (node (document "d0") (qualified-name "Documentation Example"))))
    (element (id (node (document "d0") (qualified-name "Documentation Example::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 307))) (parent (node (document "d0") (qualified-name "Documentation Example"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
