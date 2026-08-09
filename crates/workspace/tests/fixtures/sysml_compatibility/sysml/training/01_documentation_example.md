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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(model
  (namespace
    (package 'Documentation Example'
      (documentation)
      (part_def 'Automobile'
        (documentation 'Document1'))
      (alias_member 'Car' -> 'Documentation Example::Automobile'[part_def])
      (alias_member 'Torque' -> 'ISQ::TorqueValue'[unresolved]))))
~~~
