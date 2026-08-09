# META
~~~ini
description=KerML Simple Tests: Expansion
type=file
~~~
# SOURCE
~~~kerml
package Expansion {
	private import ControlFunctions::select;
	feature x = x->select {in y; in w; in z; w+1}; 
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwFeature,Ident,Eq,Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,KwIn,Ident,Semicolon,KwIn,Ident,Semicolon,Ident,Plus,DecimalValue,CloseCurly,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Expansion'
    (import_decl private 'ControlFunctions::select')
    (feature_def 'x' value)))
~~~
# FORMAT
~~~sysml
package Expansion {
    private import ControlFunctions::select;
    feature x = x->select {in y; in w; in z; w+1};
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
    (package 'Expansion'
      (membership_import private -> 'ControlFunctions::select'[unresolved])
      (feature_def 'x'
        (feature_value (=))))))
~~~
