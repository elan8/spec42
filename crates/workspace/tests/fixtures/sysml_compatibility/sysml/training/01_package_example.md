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
(model
  (namespace
    (package 'Package Example'
      (membership_import public -> 'ISQ::TorqueValue'[unresolved])
      (namespace_import private -> 'ScalarValues'[unresolved])
      (part_def 'Automobile')
      (alias_member public 'Car' -> 'Package Example::Automobile'[part_def])
      (alias_member 'Torque' -> 'ISQ::TorqueValue'[unresolved]))))
~~~
