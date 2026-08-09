# META
~~~ini
description=SysML Example (Simple Tests): RequirementTest
type=file
~~~
# SOURCE
~~~sysml
package RequirementTest {
	constraint def C;
	constraint c : C;
	private import q::**;
	requirement def R {
		assume constraint c1 : C;
		require c;
		doc /* */
    	requirement;
    	requirement def <'1'> A {
    		doc /* Text */
    		subject s;
    	}
	}
	requirement def R1 {
		require constraint c1 :>> c;
	}
	part p;
	part q {
		requirement r : R;
		satisfy r by p;
		assert satisfy r by q;
	}
	
	requirement r1 : R1;
	not satisfy r1 by p;
	assert not satisfy r1 by q;
	
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwConstraint,KwDef,Ident,Semicolon,
KwConstraint,Ident,Colon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,StarStar,Semicolon,
KwRequirement,KwDef,Ident,OpenCurly,
KwAssume,KwConstraint,Ident,Colon,Ident,Semicolon,
KwRequire,Ident,Semicolon,
KwDoc,RegularComment,
KwRequirement,Semicolon,
KwRequirement,KwDef,OpenAngle,UnrestrictedName,CloseAngle,Ident,OpenCurly,
KwDoc,RegularComment,
KwSubject,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwRequirement,KwDef,Ident,OpenCurly,
KwRequire,KwConstraint,Ident,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Semicolon,
KwPart,Ident,OpenCurly,
KwRequirement,Ident,Colon,Ident,Semicolon,
KwSatisfy,Ident,KwBy,Ident,Semicolon,
KwAssert,KwSatisfy,Ident,KwBy,Ident,Semicolon,
CloseCurly,
KwRequirement,Ident,Colon,Ident,Semicolon,
KwNot,KwSatisfy,Ident,KwBy,Ident,Semicolon,
KwAssert,KwNot,KwSatisfy,Ident,KwBy,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'RequirementTest'
    (constraint_def 'C')
    (constraint_usage 'c' : 'C')
    (import_decl private 'q::**')
    (requirement_def 'R'
      (sysml_decl 'c1' : 'C')
      (sysml_decl 'c')
      (documentation)
      (malformed)
      (requirement_def 'A'
        (documentation)
        (sysml_decl 's')))
    (requirement_def 'R1'
      (sysml_decl 'c1' :>> 'c'))
    (part_usage 'p')
    (part_usage 'q'
      (requirement_usage 'r' : 'R')
      (sysml_decl 'r')
      (sysml_decl 'r'))
    (requirement_usage 'r1' : 'R1')
    (malformed)
    (sysml_decl 'r1')
    (sysml_decl 'r1')))
~~~
# FORMAT
~~~sysml
package RequirementTest {
	constraint def C;
	constraint c : C;
	private import q::**;
	requirement def R {
		assume constraint c1 : C;
		require c;
		doc /* */
    	requirement;
    	requirement def <'1'> A {
    		doc /* Text */
    		subject s;
    	}
	}
	requirement def R1 {
		require constraint c1 :>> c;
	}
	part p;
	part q {
		requirement r : R;
		satisfy r by p;
		assert satisfy r by q;
	}
	
	requirement r1 : R1;
	not satisfy r1 by p;
	assert not satisfy r1 by q;
	
}
~~~
# EXPECTED
~~~
parse.expected_usage_declaration
parse.unexpected_token
semantic.duplicate_name 'r'
semantic.duplicate_name 'r'
semantic.duplicate_name 'r1'
semantic.duplicate_name 'r1'
semantic.ambiguous_member 'r1'
semantic.ambiguous_member 'r1'
~~~
# PROBLEMS
~~~
parse.expected_usage_declaration
parse.unexpected_token
semantic.duplicate_name 'r'
semantic.duplicate_name 'r'
semantic.duplicate_name 'r1'
semantic.duplicate_name 'r1'
semantic.ambiguous_member 'r1'
semantic.ambiguous_member 'r1'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "RequirementTest"))) (name "RequirementTest") (declared-name "RequirementTest")
      (contains
        (element (kind "constraint def") (id (node (document "d0") (qualified-name "RequirementTest::C"))) (name "C") (declared-name "C"))
        (element (kind "requirement def") (id (node (document "d0") (qualified-name "RequirementTest::R"))) (name "R") (declared-name "R")
          (contains
            (element (kind "require constraint") (id (node (document "d0") (qualified-name "RequirementTest::R::_requireConstraint_0"))) (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (effective (featuring-type (node (document "d0") (qualified-name "RequirementTest::R")))))
          )
        )
        (element (kind "requirement def") (id (node (document "d0") (qualified-name "RequirementTest::R1"))) (name "R1") (declared-name "R1"))
        (element (kind "constraint") (id (node (document "d0") (qualified-name "RequirementTest::c"))) (name "c") (declared-name "c"))
        (element (kind "part") (id (node (document "d0") (qualified-name "RequirementTest::p"))) (name "p") (declared-name "p") (declared (properties (composite true) (reference false) (ordered false))))
        (element (kind "import") (id (node (document "d0") (qualified-name "RequirementTest::q"))) (name "q") (declared-name "q"))
        (element (kind "part") (id (node (document "d0") (qualified-name "RequirementTest::q#part"))) (name "q") (declared-name "q") (declared (properties (composite true) (reference false) (ordered false))))
        (element (kind "requirement") (id (node (document "d0") (qualified-name "RequirementTest::r1"))) (name "r1") (declared-name "r1"))
      )
    )
    (element (kind "diagnostic") (id (node (document "d0") (qualified-name "RequirementTest::q#part::unresolved_satisfy_source"))) (name "unresolved_satisfy_source") (declared-name "unresolved_satisfy_source"))
    (element (kind "diagnostic") (id (node (document "d0") (qualified-name "RequirementTest::q#part::unresolved_satisfy_source#diagnostic"))) (name "unresolved_satisfy_source") (declared-name "unresolved_satisfy_source"))
  )
  (relationships
    (satisfy (status resolved) (from (node (document "d0") (qualified-name "RequirementTest::r1"))) (to (node (document "d0") (qualified-name "RequirementTest::p"))))
    (satisfy (status resolved) (from (node (document "d0") (qualified-name "RequirementTest::r1"))) (to (node (document "d0") (qualified-name "RequirementTest::q#part"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "RequirementTest::c"))) (to (node (document "d0") (qualified-name "RequirementTest::C"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "RequirementTest::r1"))) (to (node (document "d0") (qualified-name "RequirementTest::R1"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
    (satisfy (status pending-expression) (document "d0") (source-expression "r") (target-expression "p") (container-prefix "RequirementTest::q#part"))
    (satisfy (status pending-expression) (document "d0") (source-expression "r") (target-expression "q") (container-prefix "RequirementTest::q#part"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/requirement_test.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 17 1) (end 17 8))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_satisfy_source")
        (source "semantic")
        (range (start 20 10) (end 20 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_satisfy_source")
        (source "semantic")
        (range (start 21 17) (end 21 18))
      )
    )
  )
)
~~~
