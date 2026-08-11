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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "requirement_test.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 17 1) (end 17 8))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 10) (end 20 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 17) (end 21 18))
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 21 22) (end 21 23))
      )
      (diagnostic
        (severity warning)
        (code "satisfy_target_invalid_kind")
        (source "semantic")
        (range (start 25 13) (end 25 15))
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 26 26) (end 26 27))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f78adf2c6174c9ad49a75177ac348c1d58a085d8eef46f746b2b484ec84c4aab") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "RequirementTest"))) (kind "package") (name "RequirementTest") (declared-name "RequirementTest") (range (start (line 0) (character 0)) (end (line 0) (character 478))))
    (element (id (node (document "d0") (qualified-name "RequirementTest::C"))) (kind "constraint def") (name "C") (declared-name "C") (range (start (line 1) (character 1)) (end (line 1) (character 18))) (parent (node (document "d0") (qualified-name "RequirementTest"))))
    (element (id (node (document "d0") (qualified-name "RequirementTest::R"))) (kind "requirement def") (name "R") (declared-name "R") (range (start (line 4) (character 1)) (end (line 4) (character 170))) (parent (node (document "d0") (qualified-name "RequirementTest"))))
    (element (id (node (document "d0") (qualified-name "RequirementTest::R1"))) (kind "requirement def") (name "R1") (declared-name "R1") (range (start (line 14) (character 1)) (end (line 14) (character 55))) (parent (node (document "d0") (qualified-name "RequirementTest"))))
    (element (id (node (document "d0") (qualified-name "RequirementTest::R::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (range (start (line 6) (character 2)) (end (line 6) (character 12))) (parent (node (document "d0") (qualified-name "RequirementTest::R"))))
    (element (id (node (document "d0") (qualified-name "RequirementTest::c"))) (kind "constraint") (name "c") (declared-name "c") (range (start (line 2) (character 1)) (end (line 2) (character 18))) (parent (node (document "d0") (qualified-name "RequirementTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "C") (range none)))))
    (element (id (node (document "d0") (qualified-name "RequirementTest::p"))) (kind "part") (name "p") (declared-name "p") (range (start (line 17) (character 1)) (end (line 17) (character 8))) (parent (node (document "d0") (qualified-name "RequirementTest"))))
    (element (id (node (document "d0") (qualified-name "RequirementTest::q"))) (kind "import") (name "q") (declared-name "q") (range (start (line 3) (character 1)) (end (line 3) (character 22))) (parent (node (document "d0") (qualified-name "RequirementTest"))) (authored (membership (kind Import) (visibility "private") (import (reference "q") (origin Import) (shape Membership) (recursive true)) (import-range (start (line 3) (character 16)) (end (line 3) (character 17))))))
    (element (id (node (document "d0") (qualified-name "RequirementTest::q#part"))) (kind "part") (name "q") (declared-name "q") (range (start (line 18) (character 1)) (end (line 18) (character 76))) (parent (node (document "d0") (qualified-name "RequirementTest"))))
    (element (id (node (document "d0") (qualified-name "RequirementTest::r1"))) (kind "requirement") (name "r1") (declared-name "r1") (range (start (line 24) (character 1)) (end (line 24) (character 21))) (parent (node (document "d0") (qualified-name "RequirementTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "R1") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "RequirementTest"))) (kind satisfySource) (ordinal 0)) (authored-target "r1") (range (start (line 25) (character 13)) (end (line 25) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementTest::r1")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementTest"))) (kind satisfySource) (ordinal 1)) (authored-target "r1") (range (start (line 26) (character 20)) (end (line 26) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementTest::r1")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementTest"))) (kind satisfyTarget) (ordinal 0)) (authored-target "p") (range (start (line 25) (character 19)) (end (line 25) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementTest::p")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementTest"))) (kind satisfyTarget) (ordinal 1)) (authored-target "q") (range (start (line 26) (character 26)) (end (line 26) (character 27))) (outcome (status ambiguous) (candidates (node (document "d0") (qualified-name "RequirementTest::q")) (node (document "d0") (qualified-name "RequirementTest::q#part")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementTest::c"))) (kind featureTyping) (ordinal 0)) (authored-target "C") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementTest::C")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementTest::q"))) (kind membershipImport) (ordinal 0)) (authored-target "q") (range (start (line 3) (character 16)) (end (line 3) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementTest::q#part")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementTest::q#part"))) (kind satisfySource) (ordinal 0)) (authored-target "r") (range (start (line 20) (character 10)) (end (line 20) (character 11))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RequirementTest::q#part"))) (kind satisfySource) (ordinal 1)) (authored-target "r") (range (start (line 21) (character 17)) (end (line 21) (character 18))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RequirementTest::q#part"))) (kind satisfyTarget) (ordinal 0)) (authored-target "p") (range (start (line 20) (character 15)) (end (line 20) (character 16))) (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementTest::p")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementTest::q#part"))) (kind satisfyTarget) (ordinal 1)) (authored-target "q") (range (start (line 21) (character 22)) (end (line 21) (character 23))) (outcome (status ambiguous) (candidates (node (document "d0") (qualified-name "RequirementTest::q")) (node (document "d0") (qualified-name "RequirementTest::q#part")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementTest::r1"))) (kind featureTyping) (ordinal 0)) (authored-target "R1") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementTest::R1")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "RequirementTest::c"))) (target (node (document "d0") (qualified-name "RequirementTest::C"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RequirementTest::c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "RequirementTest::r1"))) (target (node (document "d0") (qualified-name "RequirementTest::R1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RequirementTest::r1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind satisfy) (source (node (document "d0") (qualified-name "RequirementTest::r1"))) (target (node (document "d0") (qualified-name "RequirementTest::p"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RequirementTest"))) (kind satisfySource) (ordinal 0)) (expression (kind satisfy) (source "r1") (target "p") (source-range (start (line 25) (character 13)) (end (line 25) (character 15))) (target-range (start (line 25) (character 19)) (end (line 25) (character 20)))))
  )
  (evaluation
  )
)
~~~
