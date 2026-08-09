# META
~~~ini
description=Coverage: Metadata features with about clause and named metadata
type=file
~~~
# SOURCE
~~~sysml
metadata def Classified;
metadata def Approval;

package Annotated {
    @ Classified about Annotated;

    part def Vehicle;
    part def Engine;

    metadata m : Classified about Vehicle, Engine;

    #Classified part def AnnotatedPart;

    #Approval #Classified part def MultiAnnotated;
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
# TOKENS
~~~zig
KwMetadata,KwDef,Ident,Semicolon,
KwMetadata,KwDef,Ident,Semicolon,
KwPackage,Ident,OpenCurly,
At,Ident,KwAbout,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwMetadata,Ident,Colon,Ident,KwAbout,Ident,Comma,Ident,Semicolon,
Hash,Ident,KwPart,KwDef,Ident,Semicolon,
Hash,Ident,Hash,Ident,KwPart,KwDef,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (metadata_def 'Classified')
  (metadata_def 'Approval')
  (package_def 'Annotated'
    (metadata_feature typed 'Classified' about 'Annotated')
    (part_def 'Vehicle')
    (part_def 'Engine')
    (metadata_feature 'm' typed 'Classified' about 'Vehicle', 'Engine')
    (part_def #'Classified' 'AnnotatedPart')
    (part_def #'Approval', 'Classified' 'MultiAnnotated')))
~~~
# FORMAT
~~~sysml
metadata def Classified;
metadata def Approval;

package Annotated {
    @Classified about Annotated;

    part def Vehicle;
    part def Engine;

    metadata m : Classified about Vehicle, Engine;

    #Classified part def AnnotatedPart;

    #Approval #Classified part def MultiAnnotated;
}
~~~
# SMG
~~~
(model
  (namespace
    (metadata_def 'Classified')
    (metadata_def 'Approval')
    (package 'Annotated'
      (metadata_usage :> 'Classified'[metadata_def] annotated 'Annotated'[package])
      (part_def 'Vehicle')
      (part_def 'Engine')
      (metadata_usage 'm' :> 'Classified'[metadata_def] annotated 'Annotated::Vehicle'[part_def] annotated 'Annotated::Engine'[part_def])
      (part_def 'AnnotatedPart')
      (part_def 'MultiAnnotated'))))
~~~
