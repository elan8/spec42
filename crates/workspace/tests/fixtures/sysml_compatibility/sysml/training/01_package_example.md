# META
~~~ini
description=SysML Training 01 (Packages): Package Example
type=file
~~~
# SOURCE
~~~sysml
package 'Package Example' {
	public import ISQ::TorqueValue;
	private import ScalarValues::*;
	 
	private part def Automobile;
	
	public alias Car for Automobile;	                         
	alias Torque for ISQ::TorqueValue;
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwPart,KwDef,Ident,Semicolon,
KwPublic,KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,ColonColon,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Package Example''
    (import_decl public 'ISQ::TorqueValue')
    (import_decl private 'ScalarValues::*')
    (part_def private 'Automobile')
    (alias_member public 'Car' for 'Automobile')
    (alias_member 'Torque' for 'ISQ::TorqueValue')))
~~~
# FORMAT
~~~sysml
package 'Package Example' {
    public import ISQ::TorqueValue;
    private import ScalarValues::*;

    private part def Automobile;

    public alias Car for Automobile;
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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Package Example"))) (name "Package Example") (declared-name "Package Example")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Package Example::*"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Package Example::Automobile"))) (name "Automobile") (declared-name "Automobile") (declared))
        (element (kind "alias") (id (node (document "d0") (qualified-name "Package Example::Car"))) (name "Car") (declared-name "Car"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "Package Example::Torque"))) (name "Torque") (declared-name "Torque"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Package Example::TorqueValue"))) (name "TorqueValue") (declared-name "TorqueValue"))
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
