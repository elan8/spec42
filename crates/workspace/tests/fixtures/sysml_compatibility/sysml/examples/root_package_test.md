# META
~~~ini
description=SysML Example (Simple Tests): RootPackageTest
type=file
~~~
# SOURCE
~~~sysml
package P1 {
	part def A;
}

package P2 {
	private import P1::*;
	part a : A;
}

private import P2::*;

package P3 {
	part b subsets a;
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwPart,Ident,KwSubsets,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'P1'
    (part_def 'A'))
  (package_def 'P2'
    (import_decl private 'P1::*')
    (part_usage 'a' : 'A'))
  (import_decl private 'P2::*')
  (package_def 'P3'
    (part_usage 'b' :> 'a')))
~~~
# FORMAT
~~~sysml
package P1 {
    part def A;
}

package P2 {
    private import P1::*;
    part a : A;
}

private import P2::*;

package P3 {
    part b subsets a;
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
    (package 'P1'
      (part_def 'A'))
    (package 'P2'
      (namespace_import private -> 'P1'[package])
      (part_usage 'a' : 'P1::A'[part_def]))
    (namespace_import private -> 'P2'[package])
    (package 'P3'
      (part_usage 'b' :> 'P2::a'[part_usage]))))
~~~
