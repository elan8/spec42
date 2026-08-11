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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "alias_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 37))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 6 8) (end 6 34))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 15 8) (end 15 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 19) (end 20 25))
      )
    )
  )
)
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
# EXPECTED
~~~
semantic.unresolved_name 'breadth'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'breadth'
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "b306919f2a8536fea2a1d4af6b189b9e92083b49bb706c4eabb2ea1d36dc52e5") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "AliasTest"))) (kind "package") (name "AliasTest") (declared-name "AliasTest") (range (start (line 0) (character 0)) (end (line 0) (character 372))))
    (element (id (node (document "d0") (qualified-name "AliasTest::P1"))) (kind "part def") (name "P1") (declared-name "P1") (range (start (line 4) (character 4)) (end (line 4) (character 74))) (parent (node (document "d0") (qualified-name "AliasTest"))))
    (element (id (node (document "d0") (qualified-name "AliasTest::P1::porig1"))) (kind "port") (name "porig1") (declared-name "porig1") (range (start (line 5) (character 8)) (end (line 5) (character 20))) (parent (node (document "d0") (qualified-name "AliasTest::P1"))))
    (element (id (node (document "d0") (qualified-name "AliasTest::b"))) (kind "attribute def") (name "b") (declared-name "b") (range (start (line 2) (character 1)) (end (line 2) (character 24))) (parent (node (document "d0") (qualified-name "AliasTest"))) (authored (membership (kind Owning)) (relationships (typing (reference "breadth") (range none)))))
    (element (id (node (document "d0") (qualified-name "AliasTest::breadth"))) (kind "import") (name "breadth") (declared-name "breadth") (range (start (line 1) (character 1)) (end (line 1) (character 38))) (parent (node (document "d0") (qualified-name "AliasTest"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQSpaceTime::breadth") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 37))))))
    (element (id (node (document "d0") (qualified-name "AliasTest::p1"))) (kind "part") (name "p1") (declared-name "p1") (range (start (line 9) (character 4)) (end (line 9) (character 50))) (parent (node (document "d0") (qualified-name "AliasTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "P1") (range (start (line 9) (character 14)) (end (line 9) (character 16)))))))
    (element (id (node (document "d0") (qualified-name "AliasTest::p1::po1"))) (kind "port") (name "po1") (declared-name "po1") (range (start (line 10) (character 8)) (end (line 10) (character 25))) (parent (node (document "d0") (qualified-name "AliasTest::p1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "po1") (range (start (line 10) (character 21)) (end (line 10) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "AliasTest::p2"))) (kind "part") (name "p2") (declared-name "p2") (range (start (line 13) (character 4)) (end (line 13) (character 73))) (parent (node (document "d0") (qualified-name "AliasTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "P1") (range (start (line 13) (character 14)) (end (line 13) (character 16)))))))
    (element (id (node (document "d0") (qualified-name "AliasTest::p2::pdest"))) (kind "port") (name "pdest") (declared-name "pdest") (range (start (line 14) (character 8)) (end (line 14) (character 19))) (parent (node (document "d0") (qualified-name "AliasTest::p2"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "AliasTest"))) (kind connectionSource) (ordinal 0)) (authored-target "p1::po1") (range (start (line 19) (character 12)) (end (line 19) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AliasTest::p1::po1")))))
    (reference (id (source (node (document "d0") (qualified-name "AliasTest"))) (kind connectionSource) (ordinal 1)) (authored-target "p1::po1") (range (start (line 20) (character 9)) (end (line 20) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AliasTest::p1::po1")))))
    (reference (id (source (node (document "d0") (qualified-name "AliasTest"))) (kind connectionTarget) (ordinal 0)) (authored-target "p2::pdest") (range (start (line 19) (character 22)) (end (line 19) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AliasTest::p2::pdest")))))
    (reference (id (source (node (document "d0") (qualified-name "AliasTest"))) (kind connectionTarget) (ordinal 1)) (authored-target "p2::pd1") (range (start (line 20) (character 19)) (end (line 20) (character 25))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AliasTest::b"))) (kind featureTyping) (ordinal 0)) (authored-target "breadth") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "AliasTest::breadth")))))
    (reference (id (source (node (document "d0") (qualified-name "AliasTest::breadth"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQSpaceTime::breadth") (range (start (line 1) (character 16)) (end (line 1) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AliasTest::p1"))) (kind featureTyping) (ordinal 0)) (authored-target "P1") (range (start (line 9) (character 14)) (end (line 9) (character 16))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AliasTest::P1")))))
    (reference (id (source (node (document "d0") (qualified-name "AliasTest::p1::po1"))) (kind redefinition) (ordinal 0)) (authored-target "po1") (range (start (line 10) (character 21)) (end (line 10) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AliasTest::p1::po1")))))
    (reference (id (source (node (document "d0") (qualified-name "AliasTest::p2"))) (kind featureTyping) (ordinal 0)) (authored-target "P1") (range (start (line 13) (character 14)) (end (line 13) (character 16))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AliasTest::P1")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AliasTest::b"))) (target (node (document "d0") (qualified-name "AliasTest::breadth"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AliasTest::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AliasTest::p1"))) (target (node (document "d0") (qualified-name "AliasTest::P1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AliasTest::p1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "AliasTest::p1::po1"))) (target (node (document "d0") (qualified-name "AliasTest::p1::po1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AliasTest::p1::po1"))) (kind redefinition) (ordinal 0)))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "AliasTest::p1::po1"))) (target (node (document "d0") (qualified-name "AliasTest::p2::pdest"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AliasTest"))) (kind connectionSource) (ordinal 0)) (expression (kind connection) (source "p1::po1") (target "p2::pdest") (source-range (start (line 19) (character 12)) (end (line 19) (character 18))) (target-range (start (line 19) (character 22)) (end (line 19) (character 30)))))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AliasTest::p2"))) (target (node (document "d0") (qualified-name "AliasTest::P1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AliasTest::p2"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
