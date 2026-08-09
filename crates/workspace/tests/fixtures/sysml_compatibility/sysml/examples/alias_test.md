# META
~~~ini
description=SysML Example (Simple Tests): AliasTest
type=file
~~~
# SOURCE
~~~sysml
package AliasTest {
	private import ISQSpaceTime::breadth; // import of an alias
	attribute b :> breadth;
	
    part def P1 {
        port porig1;
        alias po1 for porig1;
    }

    part p1 : P1 {
        port po1 :>> po1;
    }

    part p2 : P1 {
        port pdest;
        alias pd1 for pdest;
    }


    connect p1.po1 to p2.pdest;
	connect p1.po1 to p2.pd1;
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,LineComment,
KwAttribute,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
CloseCurly,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'AliasTest'
    (import_decl private 'ISQSpaceTime::breadth')
    (line_comment)
    (attribute_usage 'b' :> 'breadth')
    (part_def 'P1'
      (port_usage 'porig1')
      (alias_member 'po1' for 'porig1'))
    (part_usage 'p1' : 'P1'
      (port_usage 'po1' :>> 'po1'))
    (part_usage 'p2' : 'P1'
      (port_usage 'pdest')
      (alias_member 'pd1' for 'pdest'))
    (connection_usage
      (connector_end)
      (connector_end))
    (connection_usage
      (connector_end)
      (connector_end))))
~~~
# FORMAT
~~~sysml
package AliasTest {
    private import ISQSpaceTime::breadth;
    // import of an alias
    attribute b :> breadth;

    part def P1 {
        port porig1;
        alias po1 for porig1;
    }

    part p1 : P1 {
        port po1 :>> po1;
    }

    part p2 : P1 {
        port pdest;
        alias pd1 for pdest;
    }

    connect p1.po1 to p2.pdest;
    connect p1.po1 to p2.pd1;
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'breadth'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'breadth'
~~~
# SMG
~~~
(model
  (namespace
    (package 'AliasTest'
      (membership_import private -> 'ISQSpaceTime::breadth'[unresolved])
      (attribute_usage 'b' :> 'breadth'[unresolved])
      (part_def 'P1'
        (port_usage composite 'porig1')
        (alias_member 'po1' -> 'AliasTest::P1::porig1'[port_usage]))
      (part_usage 'p1' : 'AliasTest::P1'[part_def]
        (port_usage composite 'po1' :>> 'AliasTest::P1::po1'[alias_member]))
      (part_usage 'p2' : 'AliasTest::P1'[part_def]
        (port_usage composite 'pdest')
        (alias_member 'pd1' -> 'AliasTest::p2::pdest'[port_usage]))
      (connection_usage
        (connector_end 'p1.po1')
        (connector_end 'p2.pdest'))
      (connection_usage
        (connector_end 'p1.po1')
        (connector_end 'p2.pd1')))))
~~~
