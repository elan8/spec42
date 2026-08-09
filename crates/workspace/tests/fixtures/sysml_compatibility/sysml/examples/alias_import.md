# META
~~~ini
description=SysML Example (Import Tests): AliasImport
type=file
~~~
# SOURCE
~~~sysml
package AliasImport {
	package Definitions {
	    part def Vehicle;
	    
	    alias Car for Vehicle;
	}
	
	package Usages {
	    private import Definitions::Car;
	
	    part vehicle : Car;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'AliasImport'
    (package_def 'Definitions'
      (part_def 'Vehicle')
      (alias_member 'Car' for 'Vehicle'))
    (package_def 'Usages'
      (import_decl private 'Definitions::Car')
      (part_usage 'vehicle' : 'Car'))))
~~~
# FORMAT
~~~sysml
package AliasImport {
    package Definitions {
        part def Vehicle;

        alias Car for Vehicle;
    }

    package Usages {
        private import Definitions::Car;

        part vehicle : Car;
    }
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
    (package 'AliasImport'
      (package 'Definitions'
        (part_def 'Vehicle')
        (alias_member 'Car' -> 'AliasImport::Definitions::Vehicle'[part_def]))
      (package 'Usages'
        (membership_import private -> 'AliasImport::Definitions::Car'[alias_member])
        (part_usage 'vehicle' : 'AliasImport::Definitions::Car'[alias_member])))))
~~~
