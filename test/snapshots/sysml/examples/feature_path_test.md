# META
~~~ini
description=SysML Example (Simple Tests): FeaturePathTest
type=file
~~~
# SOURCE
~~~sysml
package Q {
  part def F {
  	part a : A;
  }
  
  part f : F;
  
  part def A {
    part g = f.a;
  }
  
  part def B {
  	part f : F;
  	part a : A;
  }
  
  part def C {
	part b : B {
	  connect f.a to a.g;
	  bind f.a = a.g;
	}
  
	part c subsets b.f {
	  	part aa subsets a;
	}
	
	flow b.f.a to c.aa;
  }
  
  part e1 {
  	attribute x : E;
  	// Ensure that "e1" resolves correctly.
  	bind e1.x = E::e2;
  }
  
  enum def E {
  	enum e1;
  	enum e2;
  }
  
  part g = new A().g.g.g;
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "feature_path_test.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 8 4) (end 8 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 11) (end 18 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 18) (end 18 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 8) (end 19 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 14) (end 19 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 22 16) (end 22 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 23 20) (end 23 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 26 6) (end 26 11))
      )
      (diagnostic
        (severity warning)
        (code "connection_context_invalid")
        (source "semantic")
        (range (start 32 8) (end 32 12))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 40 2) (end 40 25))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwBind,Ident,Dot,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,KwSubsets,Ident,Dot,Ident,OpenCurly,
KwPart,Ident,KwSubsets,Ident,Semicolon,
CloseCurly,
KwFlow,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
LineComment,
KwBind,Ident,Dot,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwEnum,KwDef,Ident,OpenCurly,
KwEnum,Ident,Semicolon,
KwEnum,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Eq,Ident,Ident,OpenParen,CloseParen,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Q'
    (part_def 'F'
      (part_usage 'a' : 'A'))
    (part_usage 'f' : 'F')
    (part_def 'A'
      (part_usage 'g' value))
    (part_def 'B'
      (part_usage 'f' : 'F')
      (part_usage 'a' : 'A'))
    (part_def 'C'
      (part_usage 'b' : 'B'
        (connection_usage
          (connector_end)
          (connector_end))
        (binding_as_usage
          (connector_end)
          (connector_end)))
      (part_usage 'c' :> 'b.f'
        (part_usage 'aa' :> 'a'))
      (flow_usage 'b'))
    (part_usage 'e1'
      (attribute_usage 'x' : 'E')
      (line_comment)
      (binding_as_usage
        (connector_end)
        (connector_end)))
    (enum_def 'E'
      (enum_value 'e1')
      (enum_value 'e2'))
    (part_usage 'g' value)))
~~~
# EXPECTED
~~~
semantic.duplicate_name 'b'
semantic.ambiguous_member 'b'
semantic.invalid_connection_end_count
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'b'
semantic.ambiguous_member 'b'
semantic.invalid_connection_end_count
~~~
# FORMAT
~~~sysml
package Q {
    part def F {
        part a : A;
    }

    part f : F;

    part def A {
        part g = f.a;
    }

    part def B {
        part f : F;
        part a : A;
    }

    part def C {
        part b : B {
            connect f.a to a.g;
            bind f.a = a.g;
        }

        part c subsets b.f {
            part aa subsets a;
        }

        flow b.f.a to c.aa;
    }

    part e1 {
        attribute x : E;
        // Ensure that "e1" resolves correctly.
        bind e1.x = E::e2;
    }

    enum def E {
        enum e1;
        enum e2;
    }

    part g = new A().g.g.g;

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "3926c725221b4ab7fc8f3f426c4cdf2174a919bb8cebc781a811e1ba0bc0fdb8") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Q"))) (kind "package") (name "Q") (declared-name "Q") (range (start (line 0) (character 0)) (end (line 0) (character 492))))
    (element (id (node (document "d0") (qualified-name "Q::A"))) (kind "part def") (name "A") (declared-name "A") (range (start (line 7) (character 2)) (end (line 7) (character 36))) (parent (node (document "d0") (qualified-name "Q"))))
    (element (id (node (document "d0") (qualified-name "Q::A::g"))) (kind "part") (name "g") (declared-name "g") (range (start (line 8) (character 4)) (end (line 8) (character 17))) (parent (node (document "d0") (qualified-name "Q::A"))))
    (element (id (node (document "d0") (qualified-name "Q::B"))) (kind "part def") (name "B") (declared-name "B") (range (start (line 11) (character 2)) (end (line 11) (character 48))) (parent (node (document "d0") (qualified-name "Q"))))
    (element (id (node (document "d0") (qualified-name "Q::B::a"))) (kind "part") (name "a") (declared-name "a") (range (start (line 13) (character 3)) (end (line 13) (character 14))) (parent (node (document "d0") (qualified-name "Q::B"))) (authored (membership (kind Feature)) (relationships (typing (reference "A") (range (start (line 13) (character 12)) (end (line 13) (character 13)))))))
    (element (id (node (document "d0") (qualified-name "Q::B::f"))) (kind "part") (name "f") (declared-name "f") (range (start (line 12) (character 3)) (end (line 12) (character 14))) (parent (node (document "d0") (qualified-name "Q::B"))) (authored (membership (kind Feature)) (relationships (typing (reference "F") (range (start (line 12) (character 12)) (end (line 12) (character 13)))))))
    (element (id (node (document "d0") (qualified-name "Q::C"))) (kind "part def") (name "C") (declared-name "C") (range (start (line 16) (character 2)) (end (line 16) (character 151))) (parent (node (document "d0") (qualified-name "Q"))))
    (element (id (node (document "d0") (qualified-name "Q::C::b"))) (kind "part") (name "b") (declared-name "b") (range (start (line 17) (character 1)) (end (line 17) (character 58))) (parent (node (document "d0") (qualified-name "Q::C"))) (authored (membership (kind Feature)) (relationships (typing (reference "B") (range (start (line 17) (character 10)) (end (line 17) (character 11)))))))
    (element (id (node (document "d0") (qualified-name "Q::C::c"))) (kind "part") (name "c") (declared-name "c") (range (start (line 22) (character 1)) (end (line 22) (character 47))) (parent (node (document "d0") (qualified-name "Q::C"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "b.f") (range (start (line 22) (character 16)) (end (line 22) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "Q::C::c::aa"))) (kind "part") (name "aa") (declared-name "aa") (range (start (line 23) (character 4)) (end (line 23) (character 22))) (parent (node (document "d0") (qualified-name "Q::C::c"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "a") (range (start (line 23) (character 20)) (end (line 23) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "Q::E"))) (kind "enum def") (name "E") (declared-name "E") (range (start (line 35) (character 2)) (end (line 35) (character 42))) (parent (node (document "d0") (qualified-name "Q"))))
    (element (id (node (document "d0") (qualified-name "Q::E::e1"))) (kind "enumerated value") (name "e1") (declared-name "e1") (range (start (line 36) (character 8)) (end (line 36) (character 10))) (parent (node (document "d0") (qualified-name "Q::E"))))
    (element (id (node (document "d0") (qualified-name "Q::E::e2"))) (kind "enumerated value") (name "e2") (declared-name "e2") (range (start (line 37) (character 8)) (end (line 37) (character 10))) (parent (node (document "d0") (qualified-name "Q::E"))))
    (element (id (node (document "d0") (qualified-name "Q::F"))) (kind "part def") (name "F") (declared-name "F") (range (start (line 1) (character 2)) (end (line 1) (character 33))) (parent (node (document "d0") (qualified-name "Q"))))
    (element (id (node (document "d0") (qualified-name "Q::F::a"))) (kind "part") (name "a") (declared-name "a") (range (start (line 2) (character 3)) (end (line 2) (character 14))) (parent (node (document "d0") (qualified-name "Q::F"))) (authored (membership (kind Feature)) (relationships (typing (reference "A") (range (start (line 2) (character 12)) (end (line 2) (character 13)))))))
    (element (id (node (document "d0") (qualified-name "Q::e1"))) (kind "part") (name "e1") (declared-name "e1") (range (start (line 29) (character 2)) (end (line 29) (character 100))) (parent (node (document "d0") (qualified-name "Q"))))
    (element (id (node (document "d0") (qualified-name "Q::e1::x"))) (kind "attribute") (name "x") (declared-name "x") (range (start (line 30) (character 3)) (end (line 30) (character 19))) (parent (node (document "d0") (qualified-name "Q::e1"))) (authored (membership (kind Feature)) (relationships (typing (reference "E") (range none)) (typing (reference "E") (range (start (line 30) (character 17)) (end (line 30) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "Q::f"))) (kind "part") (name "f") (declared-name "f") (range (start (line 5) (character 2)) (end (line 5) (character 13))) (parent (node (document "d0") (qualified-name "Q"))) (authored (membership (kind Feature)) (relationships (typing (reference "F") (range (start (line 5) (character 11)) (end (line 5) (character 12)))))))
    (element (id (node (document "d0") (qualified-name "Q::g"))) (kind "part") (name "g") (declared-name "g") (range (start (line 40) (character 2)) (end (line 40) (character 25))) (parent (node (document "d0") (qualified-name "Q"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Q::B::a"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (range (start (line 13) (character 12)) (end (line 13) (character 13))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Q::A")))))
    (reference (id (source (node (document "d0") (qualified-name "Q::B::f"))) (kind featureTyping) (ordinal 0)) (authored-target "F") (range (start (line 12) (character 12)) (end (line 12) (character 13))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Q::F")))))
    (reference (id (source (node (document "d0") (qualified-name "Q::C"))) (kind flowSource) (ordinal 0)) (authored-target "b::f::a") (range (start (line 26) (character 6)) (end (line 26) (character 11))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Q::C"))) (kind flowTarget) (ordinal 0)) (authored-target "c::aa") (range (start (line 26) (character 15)) (end (line 26) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Q::C::c::aa")))))
    (reference (id (source (node (document "d0") (qualified-name "Q::C::b"))) (kind featureTyping) (ordinal 0)) (authored-target "B") (range (start (line 17) (character 10)) (end (line 17) (character 11))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Q::B")))))
    (reference (id (source (node (document "d0") (qualified-name "Q::C::b"))) (kind connectionSource) (ordinal 0)) (authored-target "f::a") (range (start (line 18) (character 11)) (end (line 18) (character 14))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Q::C::b"))) (kind connectionTarget) (ordinal 0)) (authored-target "a::g") (range (start (line 18) (character 18)) (end (line 18) (character 21))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Q::C::b"))) (kind bindSource) (ordinal 1)) (authored-target "f::a") (range (start (line 19) (character 8)) (end (line 19) (character 11))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Q::C::b"))) (kind bindTarget) (ordinal 1)) (authored-target "a::g") (range (start (line 19) (character 14)) (end (line 19) (character 17))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Q::C::c"))) (kind subsetting) (ordinal 0)) (authored-target "b.f") (range (start (line 22) (character 16)) (end (line 22) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Q::C::c::aa"))) (kind subsetting) (ordinal 0)) (authored-target "a") (range (start (line 23) (character 20)) (end (line 23) (character 21))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Q::F::a"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (range (start (line 2) (character 12)) (end (line 2) (character 13))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Q::A")))))
    (reference (id (source (node (document "d0") (qualified-name "Q::e1"))) (kind bindSource) (ordinal 0)) (authored-target "e1::x") (range (start (line 32) (character 8)) (end (line 32) (character 12))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Q::e1::x")))))
    (reference (id (source (node (document "d0") (qualified-name "Q::e1"))) (kind bindTarget) (ordinal 0)) (authored-target "E::e2") (range (start (line 32) (character 15)) (end (line 32) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Q::E::e2")))))
    (reference (id (source (node (document "d0") (qualified-name "Q::e1::x"))) (kind featureTyping) (ordinal 0)) (authored-target "E") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Q::E")))))
    (reference (id (source (node (document "d0") (qualified-name "Q::e1::x"))) (kind featureTyping) (ordinal 1)) (authored-target "E") (range (start (line 30) (character 17)) (end (line 30) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Q::E")))))
    (reference (id (source (node (document "d0") (qualified-name "Q::f"))) (kind featureTyping) (ordinal 0)) (authored-target "F") (range (start (line 5) (character 11)) (end (line 5) (character 12))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Q::F")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Q::B::a"))) (target (node (document "d0") (qualified-name "Q::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Q::B::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Q::B::f"))) (target (node (document "d0") (qualified-name "Q::F"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Q::B::f"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Q::C::b"))) (target (node (document "d0") (qualified-name "Q::B"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Q::C::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Q::F::a"))) (target (node (document "d0") (qualified-name "Q::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Q::F::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Q::e1::x"))) (target (node (document "d0") (qualified-name "Q::E"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Q::e1::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Q::e1::x"))) (target (node (document "d0") (qualified-name "Q::E"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Q::e1::x"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind bind) (source (node (document "d0") (qualified-name "Q::e1::x"))) (target (node (document "d0") (qualified-name "Q::E::e2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Q::e1"))) (kind bindSource) (ordinal 0)) (expression (kind bind) (source "e1::x") (target "E::e2") (source-range (start (line 32) (character 8)) (end (line 32) (character 12))) (target-range (start (line 32) (character 15)) (end (line 32) (character 20)))))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Q::f"))) (target (node (document "d0") (qualified-name "Q::F"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Q::f"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Q::A::g")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "Q::g")) (expression (status "unsupported") (error "declared expression form is not supported")))
  )
)
~~~
