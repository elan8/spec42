# META
~~~ini
description=Group 12: Extended Definitions and Usages (SysML §8.2.2.27)
type=file
~~~
# SOURCE
~~~sysml
package ExtendedExamples {
    #situation def Failure;
    #situation def Failure :> Base;
    abstract #situation def AbstractFailure;
    #SecurityRelated #situation def Vulnerability;
    #situation def Failure { part p; }
    #situation batteryLow;
    #situation x : T;
    #situation x : T { }
    variation #situation def V;
}
~~~
# EXPECTED
~~~
semantic.duplicate_name 'Failure'
semantic.duplicate_name 'Failure'
semantic.duplicate_name 'x'
semantic.ambiguous_member 'Failure'
semantic.ambiguous_member 'Failure'
semantic.ambiguous_member 'x'
semantic.unresolved_name 'Base'
semantic.unresolved_name 'T'
semantic.unresolved_name 'T'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'Failure'
semantic.duplicate_name 'Failure'
semantic.duplicate_name 'x'
semantic.ambiguous_member 'Failure'
semantic.ambiguous_member 'Failure'
semantic.ambiguous_member 'x'
semantic.unresolved_name 'Base'
semantic.unresolved_name 'T'
semantic.unresolved_name 'T'
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
Hash,Ident,KwDef,Ident,Semicolon,
Hash,Ident,KwDef,Ident,ColonGt,Ident,Semicolon,
KwAbstract,Hash,Ident,KwDef,Ident,Semicolon,
Hash,Ident,Hash,Ident,KwDef,Ident,Semicolon,
Hash,Ident,KwDef,Ident,OpenCurly,KwPart,Ident,Semicolon,CloseCurly,
Hash,Ident,Ident,Semicolon,
Hash,Ident,Ident,Colon,Ident,Semicolon,
Hash,Ident,Ident,Colon,Ident,OpenCurly,CloseCurly,
KwVariation,Hash,Ident,KwDef,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ExtendedExamples'
    (extended_def #'situation' 'Failure')
    (extended_def #'situation' 'Failure' :> 'Base')
    (extended_def abstract #'situation' 'AbstractFailure')
    (extended_def #'SecurityRelated', 'situation' 'Vulnerability')
    (extended_def #'situation' 'Failure'
      (part_usage 'p'))
    (extended_usage #'situation' 'batteryLow')
    (extended_usage #'situation' 'x' : 'T')
    (extended_usage #'situation' 'x' : 'T')
    (extended_def variation #'situation' 'V')))
~~~
# FORMAT
~~~sysml
package ExtendedExamples {
    #situation def Failure;
    #situation def Failure :> Base;
    abstract #situation def AbstractFailure;
    #SecurityRelated #situation def Vulnerability;
    #situation def Failure {
        part p;
    }
    #situation batteryLow;
    #situation x : T;
    #situation x : T { }
    variation #situation def V;
}
~~~
# SMG
~~~
(model
  (namespace
    (package 'ExtendedExamples'
      (definition 'Failure')
      (definition 'Failure' :> 'Base'[unresolved])
      (definition abstract 'AbstractFailure')
      (definition 'Vulnerability')
      (definition 'Failure'
        (part_usage composite 'p'))
      (reference_usage 'batteryLow')
      (reference_usage 'x' : 'T'[unresolved])
      (reference_usage 'x' : 'T'[unresolved])
      (definition variation 'V'))))
~~~
